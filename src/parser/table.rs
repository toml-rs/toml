use crate::array_of_tables::ArrayOfTables;
use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::key::key;
use crate::parser::trivia::line_trailing;
use crate::parser::TomlParser;
use crate::repr::Decor;
use crate::{Entry, Item, Table};
use combine::parser::char::char;
use combine::parser::range::range;
use combine::stream::RangeStream;
use combine::*;
use std::cell::RefCell;
use std::mem;
// https://github.com/rust-lang/rust/issues/41358
#[allow(unused_imports)]
use std::ops::DerefMut;

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
        .and_then(|(h, t)| parser.borrow_mut().deref_mut().on_std_header(&h, t))
});

// ;; Array Table

// array-table = array-table-open key *( table-key-sep key) array-table-close
toml_parser!(array_table, parser, {
    (
        between(range(ARRAY_TABLE_OPEN), range(ARRAY_TABLE_CLOSE), key()),
        line_trailing(),
    )
        .and_then(|(h, t)| parser.borrow_mut().deref_mut().on_array_header(&h, t))
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
         From<chrono::ParseError> +
         From<crate::parser::errors::CustomError>
    ]    {
        array_table(parser)
            .or(std_table(parser))
            .message("While parsing a Table Header")
    }
}

pub(crate) fn duplicate_key(path: &[Key], i: usize) -> CustomError {
    assert!(i < path.len());
    let header: Vec<&str> = path[..i].iter().map(|k| k.repr().as_raw()).collect();
    CustomError::DuplicateKey {
        key: path[i].repr().as_raw().into(),
        table: format!("[{}]", header.join(".")),
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

    fn on_std_header(&mut self, path: &[Key], trailing: &str) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        let leading = mem::take(&mut self.document.trailing);
        let table = self.document.as_table_mut();
        self.current_table_position += 1;

        let table = Self::descend_path(table, &path[..path.len() - 1], 0, false)?;
        let key = &path[path.len() - 1];

        let decor = Decor::new(leading, trailing);

        let entry = table.entry_format(key);
        match entry {
            Entry::Occupied(entry) => {
                match entry.into_mut() {
                    // if [a.b.c] header preceded [a.b]
                    Item::Table(ref mut t) if t.implicit => {
                        debug_assert!(t.values_len() == 0);

                        t.decor = decor;
                        t.position = Some(self.current_table_position);
                        t.set_implicit(false);

                        self.current_table_path = path.to_vec();
                        Ok(())
                    }
                    _ => Err(duplicate_key(path, path.len() - 1)),
                }
            }
            Entry::Vacant(entry) => {
                let item = Item::Table(Table::with_decor_and_pos(
                    decor,
                    Some(self.current_table_position),
                ));
                entry.insert(item);
                self.current_table_path = path.to_vec();
                Ok(())
            }
        }
    }

    fn on_array_header(&mut self, path: &[Key], trailing: &str) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());

        let leading = mem::take(&mut self.document.trailing);
        let table = self.document.as_table_mut();

        let key = &path[path.len() - 1];
        let table = Self::descend_path(table, &path[..path.len() - 1], 0, false);

        match table {
            Ok(table) => {
                if !table.contains_table(key.get()) && !table.contains_value(key.get()) {
                    let decor = Decor::new(leading, trailing);

                    let entry = table
                        .entry_format(key)
                        .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
                    let array = entry.as_array_of_tables_mut().unwrap();

                    self.current_table_position += 1;
                    array.append(Table::with_decor_and_pos(
                        decor,
                        Some(self.current_table_position),
                    ));
                    self.current_table_path = path.to_vec();

                    Ok(())
                } else {
                    Err(duplicate_key(path, path.len() - 1))
                }
            }
            Err(e) => Err(e),
        }
    }
}
