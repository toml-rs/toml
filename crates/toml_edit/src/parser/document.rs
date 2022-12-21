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
use combine::parser::byte::byte;
use combine::stream::position::{IndexPositioner, Positioner, Stream};
use combine::stream::RangeStream;
use combine::Parser;
use combine::*;
use std::cell::RefCell;

// ;; TOML

// toml = expression *( newline expression )

// expression = ( ( ws comment ) /
//                ( ws keyval ws [ comment ] ) /
//                ( ws table ws [ comment ] ) /
//                  ws )
pub(crate) fn document(s: &[u8]) -> Result<Document, TomlError> {
    // Remove BOM if present
    let s = s.strip_prefix(b"\xEF\xBB\xBF").unwrap_or(s);

    let mut parser = RefCell::new(ParseState::default());
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
            parser
                .get_mut()
                .finalize_table()
                .map_err(|e| TomlError::custom(e.to_string()))?;
            let trailing = parser.borrow().trailing.as_str().into();
            parser.get_mut().document.trailing = trailing;
            Ok(parser.into_inner().document)
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
