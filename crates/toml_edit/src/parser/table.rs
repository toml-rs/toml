use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::key::key;
use crate::parser::trivia::line_trailing;
use crate::parser::TomlParser;
use crate::repr::Decor;
use crate::{Item, Table};
use combine::parser::byte::byte;
use combine::parser::range::range;
use combine::stream::RangeStream;
use combine::*;
use std::cell::RefCell;
use std::mem;
// https://github.com/rust-lang/rust/issues/41358
#[allow(unused_imports)]
use std::ops::DerefMut;

// std-table-open  = %x5B ws     ; [ Left square bracket
pub(crate) const STD_TABLE_OPEN: u8 = b'[';
// std-table-close = ws %x5D     ; ] Right square bracket
const STD_TABLE_CLOSE: u8 = b']';
// array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
const ARRAY_TABLE_OPEN: &[u8] = b"[[";
// array-table-close = ws %x5D.5D  ; ]] Double right quare bracket
const ARRAY_TABLE_CLOSE: &[u8] = b"]]";

// ;; Standard Table

// std-table = std-table-open key *( table-key-sep key) std-table-close
toml_parser!(std_table, parser, {
    (
        between(byte(STD_TABLE_OPEN), byte(STD_TABLE_CLOSE), key()),
        line_trailing().and_then(std::str::from_utf8),
    )
        .and_then(|(h, t)| parser.borrow_mut().deref_mut().on_std_header(h, t))
});

// ;; Array Table

// array-table = array-table-open key *( table-key-sep key) array-table-close
toml_parser!(array_table, parser, {
    (
        between(range(ARRAY_TABLE_OPEN), range(ARRAY_TABLE_CLOSE), key()),
        line_trailing().and_then(std::str::from_utf8),
    )
        .and_then(|(h, t)| parser.borrow_mut().deref_mut().on_array_header(h, t))
});

// ;; Table

// table = std-table / array-table
parser! {
    pub(crate) fn table['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
    where
        [I: RangeStream<
         Range = &'a [u8],
         Token = u8>,
         I::Error: ParseError<u8, &'a [u8], <I as StreamOnce>::Position>,
         <I::Error as ParseError<u8, &'a [u8], <I as StreamOnce>::Position>>::StreamError:
         From<std::num::ParseIntError> +
         From<std::num::ParseFloatError> +
         From<std::str::Utf8Error> +
         From<crate::parser::errors::CustomError>
    ]    {
        array_table(parser)
            .or(std_table(parser))
            .message("While parsing a Table Header")
    }
}

pub(crate) fn duplicate_key(path: &[Key], i: usize) -> CustomError {
    assert!(i < path.len());
    CustomError::DuplicateKey {
        key: path[i].to_repr().as_ref().as_raw().into(),
        table: Some(path[..i].to_vec()),
    }
}

pub(crate) fn extend_wrong_type(path: &[Key], i: usize, actual: &'static str) -> CustomError {
    assert!(i < path.len());
    CustomError::DottedKeyExtendWrongType {
        key: path[..=i].to_vec(),
        actual,
    }
}

impl TomlParser {
    pub(crate) fn descend_path<'t, 'k>(
        mut table: &'t mut Table,
        path: &'k [Key],
        dotted: bool,
    ) -> Result<&'t mut Table, CustomError> {
        for (i, key) in path.iter().enumerate() {
            let entry = table.entry_format(key).or_insert_with(|| {
                let mut new_table = Table::new();
                new_table.set_implicit(true);
                new_table.set_dotted(dotted);

                Item::Table(new_table)
            });
            match *entry {
                Item::Value(ref v) => {
                    return Err(extend_wrong_type(path, i, v.type_name()));
                }
                Item::ArrayOfTables(ref mut array) => {
                    debug_assert!(!array.is_empty());

                    let index = array.len() - 1;
                    let last_child = array.get_mut(index).unwrap();

                    table = last_child;
                }
                Item::Table(ref mut sweet_child_of_mine) => {
                    table = sweet_child_of_mine;
                }
                _ => unreachable!(),
            }
        }
        Ok(table)
    }

    fn on_std_header(&mut self, path: Vec<Key>, trailing: &str) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        self.finalize_table()?;
        let leading = mem::take(&mut self.trailing);
        self.start_table(path, Decor::new(leading, trailing))?;

        Ok(())
    }

    fn on_array_header(&mut self, path: Vec<Key>, trailing: &str) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        self.finalize_table()?;
        let leading = mem::take(&mut self.trailing);
        self.start_aray_table(path, Decor::new(leading, trailing))?;

        Ok(())
    }
}
