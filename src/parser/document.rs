use crate::document::Document;
use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::inline_table::KEYVAL_SEP;
use crate::parser::key::key;
use crate::parser::table::table;
use crate::parser::trivia::{comment, line_ending, line_trailing, newline, ws};
use crate::parser::value::value;
use crate::parser::{TomlError, TomlParser};
use crate::repr::Decor;
use crate::table::TableKeyValue;
use crate::Item;
use combine::parser::char::char;
use combine::parser::range::recognize;
use combine::stream::position::Stream;
use combine::stream::RangeStream;
use combine::Parser;
use combine::*;
use std::cell::RefCell;
use std::mem;
use std::ops::DerefMut;

toml_parser!(parse_comment, parser, {
    (comment(), line_ending()).map(|(c, e)| parser.borrow_mut().deref_mut().on_comment(c, e))
});

toml_parser!(
    parse_ws,
    parser,
    ws().map(|w| parser.borrow_mut().deref_mut().on_ws(w))
);

toml_parser!(parse_newline, parser, {
    recognize(newline()).map(|w| parser.borrow_mut().deref_mut().on_ws(w))
});

toml_parser!(keyval, parser, {
    parse_keyval().and_then(|(p, kv)| parser.borrow_mut().deref_mut().on_keyval(p, kv))
});

// keyval = key keyval-sep val
parser! {
    fn parse_keyval['a, I]()(I) -> (Vec<Key>, TableKeyValue)
    where
        [I: RangeStream<
         Range = &'a str,
         Token = char>,
         I::Error: ParseError<char, &'a str, <I as StreamOnce>::Position>,
         <I::Error as ParseError<char, &'a str, <I as StreamOnce>::Position>>::StreamError:
         From<std::num::ParseIntError> +
         From<std::num::ParseFloatError> +
         From<crate::parser::errors::CustomError>
    ] {
        (
            key(),
            char(KEYVAL_SEP),
            (ws(), value(), line_trailing())
        ).map(|(key, _, v)| {
            let mut path = key.into_vec();
            let key = path.pop().expect("Was vec1, so at least one exists");

            let (pre, v, suf) = v;
            let v = v.decorated(pre, suf);
            (
                path,
                TableKeyValue {
                    key,
                    value: Item::Value(v),
                }
            )
        })
    }
}

impl TomlParser {
    // ;; TOML

    // toml = expression *( newline expression )

    // expression = ( ( ws comment ) /
    //                ( ws keyval ws [ comment ] ) /
    //                ( ws table ws [ comment ] ) /
    //                  ws )
    pub(crate) fn parse(s: &str) -> Result<Document, TomlError> {
        let mut parser = RefCell::new(Self::default());
        let input = Stream::new(s);

        let parsed = parse_ws(&parser)
            .with(choice((
                eof(),
                skip_many1(
                    choice((
                        parse_comment(&parser),
                        keyval(&parser),
                        table(&parser),
                        parse_newline(&parser),
                    ))
                    .skip(parse_ws(&parser)),
                ),
            )))
            .easy_parse(input);
        match parsed {
            Ok((_, ref rest)) if !rest.input.is_empty() => {
                Err(TomlError::from_unparsed(rest.positioner, s))
            }
            Ok(..) => {
                parser
                    .get_mut()
                    .finalize_table()
                    .map_err(|e| TomlError::custom(e.to_string()))?;
                Ok(*parser.into_inner().document)
            }
            Err(e) => Err(TomlError::new(e, s)),
        }
    }

    fn on_ws(&mut self, w: &str) {
        self.document.trailing.push_str(w);
    }

    fn on_comment(&mut self, c: &str, e: &str) {
        self.document.trailing.push_str(c);
        self.document.trailing.push_str(e);
    }

    fn on_keyval(&mut self, mut path: Vec<Key>, mut kv: TableKeyValue) -> Result<(), CustomError> {
        {
            let prefix = mem::take(&mut self.document.trailing);
            let first_key = if path.is_empty() {
                &mut kv.key
            } else {
                &mut path[0]
            };
            first_key.decor = Decor::new(
                prefix + first_key.decor.prefix().unwrap_or_default(),
                first_key.decor.suffix().unwrap_or_default(),
            );
        }

        let table = &mut self.current_table;
        let table = Self::descend_path(table, &path, 0, true)?;

        // "Since tables cannot be defined more than once, redefining such tables using a [table] header is not allowed"
        let duplicate_key = table.contains_key(kv.key.get());
        // "Likewise, using dotted keys to redefine tables already defined in [table] form is not allowed"
        let mixed_table_types = table.is_dotted() == path.is_empty();
        if duplicate_key || mixed_table_types {
            Err(CustomError::DuplicateKey {
                key: kv.key.into(),
                table: "<unknown>".into(), // TODO: get actual table name
            })
        } else {
            let key = kv.key.get().to_owned();
            table.items.insert(key, kv);
            Ok(())
        }
    }
}
