use std::cell::RefCell;

use combine::parser::byte::byte;
use combine::stream::position::{IndexPositioner, Positioner, Stream};
use combine::stream::RangeStream;
use combine::Parser;
use combine::*;

use crate::document::Document;
use crate::key::Key;
use crate::parser::inline_table::KEYVAL_SEP;
use crate::parser::key::key;
use crate::parser::table::table;
use crate::parser::trivia::{comment, line_ending, line_trailing, newline, ws};
use crate::parser::value::value;
use crate::parser::{ParseState, TomlError};
use crate::table::TableKeyValue;
use crate::Item;

// ;; TOML

// toml = expression *( newline expression )

// expression = ( ( ws comment ) /
//                ( ws keyval ws [ comment ] ) /
//                ( ws table ws [ comment ] ) /
//                  ws )
pub(crate) fn document(s: &[u8]) -> Result<Document, TomlError> {
    // Remove BOM if present
    let s = s.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(s);

    let parser = RefCell::new(ParseState::default());
    let input = Stream::new(s);

    let parsed = parse_ws(&parser)
        .with(choice((
            eof(),
            skip_many1(
                look_ahead(any())
                    .then(|e| {
                        dispatch!(e;
                            crate::parser::trivia::COMMENT_START_SYMBOL => parse_comment(&parser),
                            crate::parser::table::STD_TABLE_OPEN => table(&parser),
                            crate::parser::trivia::LF |
                            crate::parser::trivia::CR => parse_newline(&parser),
                            _ => keyval(&parser),
                        )
                    })
                    .skip(parse_ws(&parser)),
            ),
        )))
        .easy_parse(input);
    match parsed {
        Ok((_, ref rest)) if !rest.input.is_empty() => Err(TomlError::from_unparsed(
            (&rest.positioner
                as &dyn Positioner<usize, Position = usize, Checkpoint = IndexPositioner>)
                .position(),
            s,
        )),
        Ok(..) => {
            let doc = parser
                .into_inner()
                .into_document()
                .map_err(|e| TomlError::custom(e.to_string()))?;
            Ok(doc)
        }
        Err(e) => Err(TomlError::new(e, s)),
    }
}

toml_parser!(parse_comment, parser, {
    (comment(), line_ending()).and_then::<_, _, std::str::Utf8Error>(|(c, e)| {
        let c = std::str::from_utf8(c)?;
        parser.borrow_mut().on_comment(c, e);
        Ok(())
    })
});

toml_parser!(parse_ws, parser, ws().map(|w| parser.borrow_mut().on_ws(w)));

toml_parser!(parse_newline, parser, {
    newline().map(|_| parser.borrow_mut().on_ws("\n"))
});

toml_parser!(keyval, parser, {
    parse_keyval().and_then(|(p, kv)| parser.borrow_mut().on_keyval(p, kv))
});

// keyval = key keyval-sep val
parser! {
    fn parse_keyval['a, I]()(I) -> (Vec<Key>, TableKeyValue)
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
    ] {
        (
            key(),
            byte(KEYVAL_SEP),
            (ws(), value(), line_trailing())
        ).and_then::<_, _, std::str::Utf8Error>(|(key, _, v)| {
            let mut path = key;
            let key = path.pop().expect("grammar ensures at least 1");

            let (pre, v, suf) = v;
            let suf = std::str::from_utf8(suf)?;
            let v = v.decorated(pre, suf);
            Ok((
                path,
                TableKeyValue {
                    key,
                    value: Item::Value(v),
                }
            ))
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use snapbox::assert_eq;

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
        for input in documents {
            let doc = document(input.as_bytes());
            let doc = match doc {
                Ok(doc) => doc,
                Err(err) => {
                    panic!(
                        "Parse error: {}\nFailed to parse:\n```\n{}\n```",
                        err, input
                    )
                }
            };

            dbg!(doc.to_string());
            dbg!(input);
            assert_eq(input, doc.to_string());
        }

        let parse_only = ["\u{FEFF}
[package]
name = \"foo\"
version = \"0.0.1\"
authors = []
"];
        for input in parse_only {
            let doc = document(input.as_bytes());
            match doc {
                Ok(_) => (),
                Err(err) => {
                    panic!(
                        "Parse error: {}\nFailed to parse:\n```\n{}\n```",
                        err, input
                    )
                }
            }
        }

        let invalid_inputs = [r#" hello = 'darkness' # my old friend
$"#];
        for input in invalid_inputs {
            let doc = document(input.as_bytes());

            assert!(doc.is_err());
        }
    }
}
