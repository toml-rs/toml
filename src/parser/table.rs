use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::key::key;
use crate::parser::trivia::line_trailing;
use crate::parser::TomlParser;
use crate::repr::Decor;
use crate::{Item, Table};
use combine::parser::char::char;
use combine::parser::range::range;
use combine::stream::RangeStream;
use combine::*;
use itertools::Itertools;
use std::cell::RefCell;
use std::mem;
// https://github.com/rust-lang/rust/issues/41358
#[allow(unused_imports)]
use std::ops::DerefMut;
use vec1::Vec1;

// std-table-open  = %x5B ws     ; [ Left square bracket
const STD_TABLE_OPEN: char = '[';
// std-table-close = ws %x5D     ; ] Right square bracket
const STD_TABLE_CLOSE: char = ']';
// array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
const ARRAY_TABLE_OPEN: &str = "[[";
// array-table-close = ws %x5D.5D  ; ]] Double right quare bracket
const ARRAY_TABLE_CLOSE: &str = "]]";

// ;; Standard Table

// std-table = std-table-open key *( table-key-sep key) std-table-close
toml_parser!(std_table, parser, {
    (
        between(char(STD_TABLE_OPEN), char(STD_TABLE_CLOSE), key()),
        line_trailing(),
    )
        .and_then(|(h, t)| parser.borrow_mut().deref_mut().on_std_header(h, t))
});

// ;; Array Table

// array-table = array-table-open key *( table-key-sep key) array-table-close
toml_parser!(array_table, parser, {
    (
        between(range(ARRAY_TABLE_OPEN), range(ARRAY_TABLE_CLOSE), key()),
        line_trailing(),
    )
        .and_then(|(h, t)| parser.borrow_mut().deref_mut().on_array_header(h, t))
});

// ;; Table

// table = std-table / array-table
parser! {
    pub(crate) fn table['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
    where
        [I: RangeStream<
         Range = &'a str,
         Token = char>,
         I::Error: ParseError<char, &'a str, <I as StreamOnce>::Position>,
         <I::Error as ParseError<char, &'a str, <I as StreamOnce>::Position>>::StreamError:
         From<std::num::ParseIntError> +
         From<std::num::ParseFloatError> +
         From<crate::parser::errors::CustomError>
    ]    {
        array_table(parser)
            .or(std_table(parser))
            .message("While parsing a Table Header")
    }
}

pub(crate) fn duplicate_key(path: &[Key], i: usize) -> CustomError {
    assert!(i < path.len());
    let header = path[..i]
        .iter()
        .map(|k| k.to_repr().as_ref().as_raw().to_owned())
        .join(".");
    CustomError::DuplicateKey {
        key: path[i].to_repr().as_ref().as_raw().into(),
        table: format!("[{}]", header),
    }
}

impl TomlParser {
    pub(crate) fn descend_path<'a>(
        table: &'a mut Table,
        path: &'a [Key],
        i: usize,
        dotted: bool,
    ) -> Result<&'a mut Table, CustomError> {
        if let Some(key) = path.get(i) {
            let entry = table.entry_format(key).or_insert_with(|| {
                let mut new_table = Table::new();
                new_table.set_implicit(true);
                new_table.set_dotted(dotted);

                Item::Table(new_table)
            });
            match *entry {
                Item::Value(..) => Err(duplicate_key(path, i)),
                Item::ArrayOfTables(ref mut array) => {
                    debug_assert!(!array.is_empty());

                    let index = array.len() - 1;
                    let last_child = array.get_mut(index).unwrap();

                    Self::descend_path(last_child, path, i + 1, dotted)
                }
                Item::Table(ref mut sweet_child_of_mine) => {
                    TomlParser::descend_path(sweet_child_of_mine, path, i + 1, dotted)
                }
                _ => unreachable!(),
            }
        } else {
            Ok(table)
        }
    }

    fn on_std_header(&mut self, path: Vec1<Key>, trailing: &str) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        self.finalize_table()?;
        let leading = mem::take(&mut self.document.trailing);
        self.start_table(path, Decor::new(leading, trailing))?;

        Ok(())
    }

    fn on_array_header(&mut self, path: Vec1<Key>, trailing: &str) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        self.finalize_table()?;
        let leading = mem::take(&mut self.document.trailing);
        self.start_aray_table(path, Decor::new(leading, trailing))?;

        Ok(())
    }
}
