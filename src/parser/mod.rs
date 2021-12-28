#![allow(clippy::unneeded_field_pattern)]
#![allow(clippy::toplevel_ref_arg)]

#[macro_use]
mod macros;
mod array;
pub(crate) mod datetime;
mod document;
mod errors;
mod inline_table;
mod key;
pub(crate) mod numbers;
pub(crate) mod strings;
mod table;
mod trivia;
mod value;

pub use self::errors::TomlError;
pub(crate) use self::key::is_unquoted_char;
pub(crate) use self::key::key as key_path;
pub(crate) use self::key::simple_key;
pub(crate) use self::value::value as value_parser;

use self::table::duplicate_key;
use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::repr::Decor;
use crate::{ArrayOfTables, Document, Entry, Item, Table};

pub(crate) struct TomlParser {
    document: Document,
    trailing: String,
    current_table_position: usize,
    current_table: Table,
    current_is_array: bool,
    current_table_path: Vec<Key>,
}

impl TomlParser {
    pub(crate) fn start_aray_table(
        &mut self,
        path: Vec<Key>,
        decor: Decor,
    ) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());
        debug_assert!(self.current_table.is_empty());
        debug_assert!(self.current_table_path.is_empty());

        // Look up the table on start to ensure the duplicate_key error points to the right line
        let root = self.document.as_table_mut();
        let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
        let key = &path[path.len() - 1];
        let entry = parent_table
            .entry_format(key)
            .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
        entry
            .as_array_of_tables()
            .ok_or_else(|| duplicate_key(&path, path.len() - 1))?;

        self.current_table_position += 1;
        self.current_table.decor = decor;
        self.current_table.set_position(self.current_table_position);
        self.current_is_array = true;
        self.current_table_path = path;

        Ok(())
    }

    pub(crate) fn start_table(&mut self, path: Vec<Key>, decor: Decor) -> Result<(), CustomError> {
        debug_assert!(!path.is_empty());
        debug_assert!(self.current_table.is_empty());
        debug_assert!(self.current_table_path.is_empty());

        // 1. Look up the table on start to ensure the duplicate_key error points to the right line
        // 2. Ensure any child tables from an implicit table are preserved
        let root = self.document.as_table_mut();
        let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
        let key = &path[path.len() - 1];
        if let Some(entry) = parent_table.remove(key.get()) {
            match entry {
                Item::Table(t) if t.implicit => {
                    self.current_table = t;
                }
                _ => return Err(duplicate_key(&path, path.len() - 1)),
            }
        }

        self.current_table_position += 1;
        self.current_table.decor = decor;
        self.current_table.set_position(self.current_table_position);
        self.current_is_array = false;
        self.current_table_path = path;

        Ok(())
    }

    pub(crate) fn finalize_table(&mut self) -> Result<(), CustomError> {
        let mut table = std::mem::take(&mut self.current_table);
        let path = std::mem::take(&mut self.current_table_path);

        let root = self.document.as_table_mut();
        if path.is_empty() {
            assert!(root.is_empty());
            std::mem::swap(&mut table, root);
        } else if self.current_is_array {
            let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
            let key = &path[path.len() - 1];

            let entry = parent_table
                .entry_format(key)
                .or_insert(Item::ArrayOfTables(ArrayOfTables::new()));
            let array = entry
                .as_array_of_tables_mut()
                .ok_or_else(|| duplicate_key(&path, path.len() - 1))?;
            array.push(table);
        } else {
            let parent_table = Self::descend_path(root, &path[..path.len() - 1], false)?;
            let key = &path[path.len() - 1];

            let entry = parent_table.entry_format(key);
            match entry {
                Entry::Occupied(entry) => {
                    match entry.into_mut() {
                        // if [a.b.c] header preceded [a.b]
                        Item::Table(ref mut t) if t.implicit => {
                            std::mem::swap(t, &mut table);
                        }
                        _ => return Err(duplicate_key(&path, path.len() - 1)),
                    }
                }
                Entry::Vacant(entry) => {
                    let item = Item::Table(table);
                    entry.insert(item);
                }
            }
        }

        Ok(())
    }
}

impl Default for TomlParser {
    fn default() -> Self {
        Self {
            document: Document::new(),
            trailing: String::new(),
            current_table_position: 0,
            current_table: Table::new(),
            current_is_array: false,
            current_table_path: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::*;
    use combine::stream::position::Stream;
    use combine::*;
    use pretty_assertions::assert_eq;
    use std::fmt;
    // Copied from https://github.com/colin-kiegel/rust-pretty-assertions/issues/24
    /// Wrapper around string slice that makes debug output `{:?}` to print string same way as `{}`.
    /// Used in different `assert*!` macros in combination with `pretty_assertions` crate to make
    /// test failures to show nice diffs.
    #[derive(PartialEq, Eq)]
    struct PrettyString<'a>(pub(crate) &'a str);
    /// Make diff to display string as multi-line string
    impl<'a> fmt::Debug for PrettyString<'a> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.0)
        }
    }

    macro_rules! parsed_eq {
        ($parsed:ident, $expected:expr) => {{
            assert!($parsed.is_ok(), "{:?}", $parsed.err().unwrap());
            let (v, rest) = $parsed.unwrap();
            assert_eq!(v, $expected);
            assert!(rest.input.is_empty());
        }};
    }

    macro_rules! parsed_float_eq {
        ($input:ident, $expected:expr) => {{
            let parsed = numbers::float().easy_parse(Stream::new($input.as_bytes()));
            let (v, rest) = match parsed {
                Ok(parsed) => parsed,
                Err(err) => {
                    panic!("Unexpected error for {:?}: {:?}", $input, err);
                }
            };
            if $expected.is_nan() {
                assert!(v.is_nan());
            } else if $expected.is_infinite() {
                assert!(v.is_infinite());
                assert_eq!($expected.is_sign_positive(), v.is_sign_positive());
            } else {
                dbg!($expected);
                dbg!(v);
                assert!(($expected - v).abs() < std::f64::EPSILON);
            }
            assert!(rest.input.is_empty());
        }};
    }

    macro_rules! parsed_value_eq {
        ($input:expr) => {
            let parsed = value::value().easy_parse(Stream::new($input.as_bytes()));
            let (v, rest) = match parsed {
                Ok(parsed) => parsed,
                Err(err) => {
                    panic!("Unexpected error for {:?}: {:?}", $input, err);
                }
            };
            assert_eq!(v.to_string(), *$input);
            assert!(rest.input.is_empty());
        };
    }

    macro_rules! parsed_date_time_eq {
        ($input:expr, $is:ident) => {{
            let parsed = value::value().easy_parse(Stream::new($input.as_bytes()));
            assert!(parsed.is_ok());
            let (v, rest) = parsed.unwrap();
            assert_eq!(v.to_string(), *$input);
            assert!(rest.input.is_empty());
            assert!(v.$is());
        }};
    }

    #[test]
    fn integers() {
        let cases = [
            ("+99", 99),
            ("42", 42),
            ("0", 0),
            ("-17", -17),
            ("1_000", 1_000),
            ("5_349_221", 5_349_221),
            ("1_2_3_4_5", 1_2_3_4_5),
            ("0xF", 15),
            ("0o0_755", 493),
            ("0b1_0_1", 5),
            (&std::i64::MIN.to_string()[..], std::i64::MIN),
            (&std::i64::MAX.to_string()[..], std::i64::MAX),
        ];
        for &(input, expected) in &cases {
            let parsed = numbers::integer().easy_parse(Stream::new(input.as_bytes()));
            parsed_eq!(parsed, expected);
        }

        let overflow = "1000000000000000000000000000000000";
        let parsed = numbers::integer().easy_parse(Stream::new(overflow.as_bytes()));
        assert!(parsed.is_err());
    }

    #[test]
    fn floats() {
        let cases = [
            ("+1.0", 1.0),
            ("3.1419", 3.1419),
            ("-0.01", -0.01),
            ("5e+22", 5e+22),
            ("1e6", 1e6),
            ("-2E-2", -2E-2),
            ("6.626e-34", 6.626e-34),
            ("9_224_617.445_991_228_313", 9_224_617.445_991_227),
            ("-1.7976931348623157e+308", std::f64::MIN),
            ("1.7976931348623157e+308", std::f64::MAX),
            ("nan", f64::NAN),
            ("+nan", f64::NAN),
            ("-nan", f64::NAN),
            ("inf", f64::INFINITY),
            ("+inf", f64::INFINITY),
            ("-inf", f64::NEG_INFINITY),
            // ("1e+400", std::f64::INFINITY),
        ];
        for &(input, expected) in &cases {
            parsed_float_eq!(input, expected);
        }
    }

    #[test]
    fn basic_string() {
        let input =
            r#""I'm a string. \"You can quote me\". Name\tJos\u00E9\nLocation\tSF. \U0002070E""#;
        let parsed = strings::string().easy_parse(Stream::new(input.as_bytes()));
        parsed_eq!(
            parsed,
            "I\'m a string. \"You can quote me\". Name\tJosé\nLocation\tSF. \u{2070E}"
        );
    }

    #[test]
    fn ml_basic_string() {
        let cases = [
            (
                r#""""
Roses are red
Violets are blue""""#,
                r#"Roses are red
Violets are blue"#,
            ),
            (r#"""" \""" """"#, " \"\"\" "),
            (r#"""" \\""""#, " \\"),
        ];

        for &(input, expected) in &cases {
            dbg!(input);
            let parsed = strings::string().easy_parse(Stream::new(input.as_bytes()));
            parsed_eq!(parsed, expected);
        }

        let invalid_cases = [r#""""  """#, r#""""  \""""#];

        for input in &invalid_cases {
            let parsed = strings::ml_basic_string().easy_parse(Stream::new(input.as_bytes()));
            assert!(parsed.is_err());
        }
    }

    #[test]
    fn ml_basic_string_escape_ws() {
        let inputs = [
            r#""""
The quick brown \


  fox jumps over \
    the lazy dog.""""#,
            r#""""\
       The quick brown \
       fox jumps over \
       the lazy dog.\
       """"#,
        ];
        for input in &inputs {
            dbg!(input);
            let parsed = strings::string().easy_parse(Stream::new(input.as_bytes()));
            parsed_eq!(parsed, "The quick brown fox jumps over the lazy dog.");
        }
        let empties = [
            r#""""\
       """"#,
            r#""""
\
  \
""""#,
        ];
        for empty in &empties {
            let parsed = strings::string().easy_parse(Stream::new(empty.as_bytes()));
            parsed_eq!(parsed, "");
        }
    }

    #[test]
    fn literal_string() {
        let inputs = [
            r#"'C:\Users\nodejs\templates'"#,
            r#"'\\ServerX\admin$\system32\'"#,
            r#"'Tom "Dubs" Preston-Werner'"#,
            r#"'<\i\c*\s*>'"#,
        ];

        for input in &inputs {
            let parsed = strings::string().easy_parse(Stream::new(input.as_bytes()));
            parsed_eq!(parsed, &input[1..input.len() - 1]);
        }
    }

    #[test]
    fn ml_literal_string() {
        let input = r#"'''I [dw]on't need \d{2} apples'''"#;
        let parsed = strings::string().easy_parse(Stream::new(input.as_bytes()));
        parsed_eq!(parsed, &input[3..input.len() - 3]);
        let input = r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"#;
        let parsed = strings::string().easy_parse(Stream::new(input.as_bytes()));
        parsed_eq!(parsed, &input[4..input.len() - 3]);
    }

    #[test]
    fn offset_date_time() {
        let inputs = [
            "1979-05-27T07:32:00Z",
            "1979-05-27T00:32:00-07:00",
            "1979-05-27T00:32:00.999999-07:00",
        ];
        for input in &inputs {
            parsed_date_time_eq!(input, is_datetime);
        }
    }

    #[test]
    fn local_date_time() {
        let inputs = ["1979-05-27T07:32:00", "1979-05-27T00:32:00.999999"];
        for input in &inputs {
            parsed_date_time_eq!(input, is_datetime);
        }
    }

    #[test]
    fn local_date() {
        let inputs = ["1979-05-27", "2017-07-20"];
        for input in &inputs {
            parsed_date_time_eq!(input, is_datetime);
        }
    }

    #[test]
    fn local_time() {
        let inputs = ["07:32:00", "00:32:00.999999"];
        for input in &inputs {
            parsed_date_time_eq!(input, is_datetime);
        }
    }

    #[test]
    fn time_fraction_truncated() {
        let input = "1987-07-05T17:45:00.123456789012345Z";
        parsed_date_time_eq!(input, is_datetime);
    }

    #[test]
    fn trivia() {
        let inputs = [
            "",
            r#" "#,
            r#"
"#,
            r#"
# comment

# comment2


"#,
            r#"
        "#,
            r#"# comment
# comment2


   "#,
        ];
        for input in &inputs {
            let parsed = trivia::ws_comment_newline().easy_parse(Stream::new(input.as_bytes()));
            assert!(parsed.is_ok());
            let (t, rest) = parsed.unwrap();
            assert!(rest.input.is_empty());
            assert_eq!(t, input.as_bytes());
        }
    }

    #[test]
    fn arrays() {
        let inputs = [
            r#"[]"#,
            r#"[   ]"#,
            r#"[
  1, 2, 3
]"#,
            r#"[
  1,
  2, # this is ok
]"#,
            r#"[# comment
# comment2


   ]"#,
            r#"[# comment
# comment2
      1

#sd
,
# comment3

   ]"#,
            r#"[1]"#,
            r#"[1,]"#,
            r#"[ "all", 'strings', """are the same""", '''type''']"#,
            r#"[ 100, -2,]"#,
            r#"[1, 2, 3]"#,
            r#"[1.1, 2.1, 3.1]"#,
            r#"["a", "b", "c"]"#,
            r#"[ [ 1, 2 ], [3, 4, 5] ]"#,
            r#"[ [ 1, 2 ], ["a", "b", "c"] ]"#,
            r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        ];
        for input in &inputs {
            parsed_value_eq!(input);
        }

        let invalid_inputs = [r#"["#, r#"[,]"#, r#"[,2]"#, r#"[1e165,,]"#];
        for input in &invalid_inputs {
            let parsed = array::array().easy_parse(Stream::new(input.as_bytes()));
            assert!(parsed.is_err());
        }
    }

    #[test]
    fn inline_tables() {
        let inputs = [
            r#"{}"#,
            r#"{   }"#,
            r#"{a = 1e165}"#,
            r#"{ hello = "world", a = 1}"#,
            r#"{ hello.world = "a" }"#,
        ];
        for input in &inputs {
            parsed_value_eq!(input);
        }
        let invalid_inputs = [r#"{a = 1e165"#, r#"{ hello = "world", a = 2, hello = 1}"#];
        for input in &invalid_inputs {
            let parsed = inline_table::inline_table().easy_parse(Stream::new(input.as_bytes()));
            assert!(parsed.is_err());
        }
    }

    #[test]
    fn keys() {
        let cases = [
            ("a", "a"),
            (r#""hello\n ""#, "hello\n "),
            (r#"'hello\n '"#, "hello\\n "),
        ];

        for &(input, expected) in &cases {
            let parsed = key::simple_key().easy_parse(Stream::new(input.as_bytes()));
            assert!(parsed.is_ok());
            let ((.., k), rest) = parsed.unwrap();
            assert_eq!(k.as_str(), expected);
            assert_eq!(rest.input.len(), 0);
        }
    }

    #[test]
    fn values() {
        let inputs = [
            "1979-05-27T00:32:00.999999",
            "-239",
            "1e200",
            "9_224_617.445_991_228_313",
            r#"'''I [dw]on't need \d{2} apples'''"#,
            r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"#,
            r#""Jos\u00E9\n""#,
            r#""\\\"\b/\f\n\r\t\u00E9\U000A0000""#,
            r#"{ hello = "world", a = 1}"#,
            r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        ];
        for input in &inputs {
            parsed_value_eq!(input);
        }
    }

    #[test]
    fn documents() {
        let documents = [
            r#"
# This is a TOML document.

title = "TOML Example"

    [owner]
    name = "Tom Preston-Werner"
    dob = 1979-05-27T07:32:00-08:00 # First class dates

    [database]
    server = "192.168.1.1"
    ports = [ 8001, 8001, 8002 ]
    connection_max = 5000
    enabled = true

    [servers]

    # Indentation (tabs and/or spaces) is allowed but not required
[servers.alpha]
    ip = "10.0.0.1"
    dc = "eqdc10"

    [servers.beta]
    ip = "10.0.0.2"
    dc = "eqdc10"

    [clients]
    data = [ ["gamma", "delta"], [1, 2] ]

    # Line breaks are OK when inside arrays
hosts = [
    "alpha",
    "omega"
]

   'some.wierd .stuff'   =  """
                         like
                         that
                      #   """ # this broke my sintax highlighting
   " also. like " = '''
that
'''
   double = 2e39 # this number looks familiar
# trailing comment"#,
            r#""#,
            r#"  "#,
            r#" hello = 'darkness' # my old friend
"#,
            r#"[parent . child]
key = "value"
"#,
            r#"hello.world = "a"
"#,
            r#"foo = 1979-05-27 # Comment
"#,
        ];
        for document in &documents {
            let doc = TomlParser::parse(document.as_bytes());
            let doc = match doc {
                Ok(doc) => doc,
                Err(err) => {
                    panic!(
                        "Parse error: {}\nFailed to parse:\n```\n{}\n```",
                        err, document
                    )
                }
            };

            dbg!(doc.to_string());
            dbg!(document);
            assert_eq!(PrettyString(document), PrettyString(&doc.to_string()));
        }

        let parse_only = ["\u{FEFF}
[package]
name = \"foo\"
version = \"0.0.1\"
authors = []
"];
        for document in &parse_only {
            let doc = TomlParser::parse(document.as_bytes());
            match doc {
                Ok(_) => (),
                Err(err) => {
                    panic!(
                        "Parse error: {}\nFailed to parse:\n```\n{}\n```",
                        err, document
                    )
                }
            }
        }

        let invalid_inputs = [r#" hello = 'darkness' # my old friend
$"#];
        for document in &invalid_inputs {
            let doc = TomlParser::parse(document.as_bytes());

            assert!(doc.is_err());
        }
    }
}
