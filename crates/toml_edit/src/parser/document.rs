use std::cell::RefCell;

use nom8::bytes::any;
use nom8::bytes::one_of;
use nom8::combinator::cut;
use nom8::combinator::eof;
use nom8::combinator::opt;
use nom8::combinator::peek;
use nom8::error::FromExternalError;
use nom8::multi::many0_count;

use crate::document::Document;
use crate::key::Key;
use crate::parser::inline_table::KEYVAL_SEP;
use crate::parser::key::key;
use crate::parser::prelude::*;
use crate::parser::state::ParseState;
use crate::parser::table::table;
use crate::parser::trivia::{comment, line_ending, line_trailing, newline, ws};
use crate::parser::value::value;
use crate::table::TableKeyValue;
use crate::Item;

// ;; TOML

// toml = expression *( newline expression )

// expression = ( ( ws comment ) /
//                ( ws keyval ws [ comment ] ) /
//                ( ws table ws [ comment ] ) /
//                  ws )
pub(crate) fn document(input: Input<'_>) -> IResult<Input<'_>, Document, ParserError<'_>> {
    let state = RefCell::new(ParseState::default());
    let state_ref = &state;

    let (i, _o) = (
        // Remove BOM if present
        opt(b"\xEF\xBB\xBF"),
        parse_ws(state_ref),
        many0_count((
            dispatch! {peek(any);
                crate::parser::trivia::COMMENT_START_SYMBOL => cut(parse_comment(state_ref)),
                crate::parser::table::STD_TABLE_OPEN => cut(table(state_ref)),
                crate::parser::trivia::LF |
                crate::parser::trivia::CR => parse_newline(state_ref),
                _ => cut(keyval(state_ref)),
            },
            parse_ws(state_ref),
        )),
        eof,
    )
        .parse(input)?;
    state
        .into_inner()
        .into_document()
        .map(|document| (i, document))
        .map_err(|err| {
            nom8::Err::Error(ParserError::from_external_error(
                i,
                nom8::error::ErrorKind::MapRes,
                err,
            ))
        })
}

pub(crate) fn parse_comment<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl FnMut(Input<'i>) -> IResult<Input<'i>, (), ParserError<'_>> + 's {
    move |i| {
        (comment, line_ending)
            .map_res::<_, _, std::str::Utf8Error>(|(c, e)| {
                let c = std::str::from_utf8(c)?;
                state.borrow_mut().on_comment(c, e);
                Ok(())
            })
            .parse(i)
    }
}

pub(crate) fn parse_ws<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl FnMut(Input<'i>) -> IResult<Input<'i>, (), ParserError<'i>> + 's {
    move |i| ws.map(|w| state.borrow_mut().on_ws(w)).parse(i)
}

pub(crate) fn parse_newline<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl FnMut(Input<'i>) -> IResult<Input<'i>, (), ParserError<'i>> + 's {
    move |i| newline.map(|_| state.borrow_mut().on_ws("\n")).parse(i)
}

pub(crate) fn keyval<'s, 'i>(
    state: &'s RefCell<ParseState>,
) -> impl FnMut(Input<'i>) -> IResult<Input<'i>, (), ParserError<'i>> + 's {
    move |i| {
        parse_keyval
            .map_res(|(p, kv)| state.borrow_mut().on_keyval(p, kv))
            .parse(i)
    }
}

// keyval = key keyval-sep val
pub(crate) fn parse_keyval(
    input: Input<'_>,
) -> IResult<Input<'_>, (Vec<Key>, TableKeyValue), ParserError<'_>> {
    (
        key,
        cut((
            one_of(KEYVAL_SEP)
                .context(Context::Expected(ParserValue::CharLiteral('.')))
                .context(Context::Expected(ParserValue::CharLiteral('='))),
            (
                ws,
                value(RecursionCheck::default()),
                line_trailing
                    .context(Context::Expected(ParserValue::CharLiteral('\n')))
                    .context(Context::Expected(ParserValue::CharLiteral('#'))),
            ),
        )),
    )
        .map_res::<_, _, std::str::Utf8Error>(|(key, (_, v))| {
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
                },
            ))
        })
        .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn documents() {
        let documents = [
            "",
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
            dbg!(input);
            let parsed = document.parse(new_input(input)).finish();
            let doc = match parsed {
                Ok(doc) => doc,
                Err(err) => {
                    panic!(
                        "Parse error: {:?}\nFailed to parse:\n```\n{}\n```",
                        err, input
                    )
                }
            };

            snapbox::assert_eq(input, doc.to_string());
        }
    }

    #[test]
    fn documents_parse_only() {
        let parse_only = ["\u{FEFF}
[package]
name = \"foo\"
version = \"0.0.1\"
authors = []
"];
        for input in parse_only {
            dbg!(input);
            let parsed = document.parse(new_input(input)).finish();
            match parsed {
                Ok(_) => (),
                Err(err) => {
                    panic!(
                        "Parse error: {:?}\nFailed to parse:\n```\n{}\n```",
                        err, input
                    )
                }
            }
        }
    }

    #[test]
    fn invalid_documents() {
        let invalid_inputs = [r#" hello = 'darkness' # my old friend
$"#];
        for input in invalid_inputs {
            dbg!(input);
            let parsed = document.parse(new_input(input)).finish();
            assert!(parsed.is_err(), "Input: {:?}", input);
        }
    }
}
