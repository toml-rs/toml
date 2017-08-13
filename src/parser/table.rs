use combine::*;
use combine::char::char;
use combine::range::{range, recognize_with_value};
use combine::primitives::RangeStream;
use parser::TomlParser;
use parser::errors::CustomError;
use parser::trivia::{line_trailing, ws};
use parser::key::key;
use decor::{InternalString, Repr};
use table::{Header, HeaderKind, Table, TableChildMut, TableEntry};
use std::mem;
use std::cell::RefCell;
// https://github.com/rust-lang/rust/issues/41358
#[allow(unused_imports)]
use std::ops::DerefMut;


// table-key-sep   = ws %x2E ws  ; . Period
const TABLE_KEY_SEP: char = '.';
// std-table-open  = %x5B ws     ; [ Left square bracket
const STD_TABLE_OPEN: char = '[';
// std-table-close = ws %x5D     ; ] Right square bracket
const STD_TABLE_CLOSE: char = ']';
// array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
const ARRAY_TABLE_OPEN: &'static str = "[[";
// array-table-close = ws %x5D.5D  ; ]] Double right quare bracket
const ARRAY_TABLE_CLOSE: &'static str = "]]";


// note: this rule is not present in the original grammar
// key-path = key *( table-key-sep key)
parse!(key_path() -> (&'a str, Vec<InternalString>), {
    recognize_with_value(
        sep_by1(between(ws(), ws(), key().map(|(_, key)| key)),
                char(TABLE_KEY_SEP))
    )
});


// ;; Standard Table

// std-table = std-table-open key *( table-key-sep key) std-table-close
parser!{
    fn std_table['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        (
            between(char(STD_TABLE_OPEN), char(STD_TABLE_CLOSE),
                    key_path()),
            line_trailing(),
        )
            .and_then(|(h, t)|
                      parser
                      .borrow_mut()
                      .deref_mut()
                      .on_std_header(h, t))
    }
}

// ;; Array Table

// array-table = array-table-open key *( table-key-sep key) array-table-close
parser!{
    fn array_table['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        (
            between(range(ARRAY_TABLE_OPEN), range(ARRAY_TABLE_CLOSE),
                    key_path()),
            line_trailing(),
        )
            .and_then(|(h, t)|
                      parser
                      .borrow_mut()
                      .deref_mut()
                      .on_array_header(h, t))
    }
}


// ;; Table

// table = std-table / array-table
parser!{
    pub fn table['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        array_table(parser)
            .or(std_table(parser))
            .message("While parsing a Table Header")
    }
}

impl TomlParser {
    fn descend_path<'a>(
        table: &'a mut Table,
        path: &[InternalString],
    ) -> Result<&'a mut Table, CustomError> {
        if let Some(key) = path.get(0) {
            let header = table.child_header(key, HeaderKind::Implicit);
            let parent_header = table.header.repr.raw_value.to_string();
            match table.append_table_with_header(key, header) {
                TableChildMut::Value(..) => Err(CustomError::DuplicateKey {
                    key: key.clone(),
                    table: parent_header,
                }),
                TableChildMut::Array(array) => {
                    debug_assert!(!array.is_empty());

                    let i = array.len() - 1;
                    let last_child = array.get_mut(i).unwrap();

                    Self::descend_path(last_child, &path[1..])
                }
                TableChildMut::Table(sweet_child_of_mine) => {
                    TomlParser::descend_path(sweet_child_of_mine, &path[1..])
                }
            }
        } else {
            Ok(table)
        }
    }

    fn on_std_header(
        &mut self,
        header: (&str, Vec<InternalString>),
        trailing: &str,
    ) -> Result<(), CustomError> {
        let (span, header) = header;
        debug_assert!(!header.is_empty());

        let leading = mem::replace(&mut self.document.trailing, InternalString::new());
        let mut table = self.document.root_mut();

        let table = Self::descend_path(table, &header[..header.len() - 1]);
        let key = &header[header.len() - 1];

        match table {
            Ok(table) => {
                let header = Header {
                    repr: Repr::new(leading, span.into(), trailing.into()),
                    kind: HeaderKind::Standard,
                };

                match table.entry(key) {
                    // if [a.b.c] header preceded [a.b]
                    TableEntry::Table(ref mut t) if t.header.kind == HeaderKind::Implicit => {
                        t.move_to_end();
                        t.header = header;
                        self.current_table = *t;
                        return Ok(());
                    }
                    TableEntry::Vacant(ref mut parent) => {
                        let mut table = parent.append_table_with_header(key, header);
                        self.current_table = table.as_table_mut().unwrap();
                        return Ok(());
                    }
                    _ => {}
                }
                Err(CustomError::DuplicateKey {
                    key: key.clone(),
                    table: table.header.repr.raw_value.to_string(),
                })
            }
            Err(e) => Err(e),
        }
    }

    fn on_array_header(
        &mut self,
        header: (&str, Vec<InternalString>),
        trailing: &str,
    ) -> Result<(), CustomError> {
        let (span, header) = header;
        debug_assert!(!header.is_empty());

        let leading = mem::replace(&mut self.document.trailing, InternalString::new());
        let mut table = self.document.root_mut();

        let key = &header[header.len() - 1];
        let table = Self::descend_path(table, &header[..header.len() - 1]);

        match table {
            Ok(table) => if !table.contains_table(key) && !table.contains_value(key) {
                let header = Header {
                    repr: Repr::new(leading, span.into(), trailing.into()),
                    kind: HeaderKind::Array,
                };

                let array = table.insert_array_assume_vacant(key);
                self.current_table = array.append_with_header(header, self.current_table);

                Ok(())
            } else {
                Err(CustomError::DuplicateKey {
                    key: key.clone(),
                    table: table.header.repr.raw_value.to_string(),
                })
            },
            Err(e) => Err(e),
        }
    }
}
