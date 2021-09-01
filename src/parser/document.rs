use crate::document::Document;
use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::inline_table::KEYVAL_SEP;
use crate::parser::key::simple_key;
use crate::parser::table::table;
use crate::parser::trivia::{comment, line_ending, line_trailing, newline, ws};
use crate::parser::value::value;
use crate::parser::{TomlError, TomlParser};
use crate::repr::{Decor, Repr};
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
    parse_keyval().and_then(|kv| parser.borrow_mut().deref_mut().on_keyval(kv))
});

// keyval = key keyval-sep val
parser! {
    fn parse_keyval['a, I]()(I) -> TableKeyValue
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
    ] {
        (
            (simple_key(), ws()),
            char(KEYVAL_SEP),
            (ws(), value(), line_trailing())
        ).map(|(k, _, v)| {
            let ((raw, key), suf) = k;
            let key_repr = Repr::new_unchecked(raw);
            let key_decor = Decor::new("", suf);
            let key = Key::new_unchecked(key_repr, key, key_decor);

            let (pre, v, suf) = v;
            let v = v.decorated(pre, suf);
            TableKeyValue {
                key,
                value: Item::Value(v),
            }
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
        let parser = RefCell::new(Self::default());
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
            Ok(..) => Ok(*parser.into_inner().document),
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

    fn on_keyval(&mut self, mut kv: TableKeyValue) -> Result<(), CustomError> {
        let prefix = mem::take(&mut self.document.trailing);
        kv.key.decor = Decor::new(
            prefix + kv.key.decor.prefix().unwrap_or_default(),
            kv.key.decor.suffix().unwrap_or_default(),
        );

        let root = self.document.as_table_mut();
        let table = Self::descend_path(root, self.current_table_path.as_slice(), 0)
            .expect("the table path is valid; qed");
        if table.contains_key(kv.key.get()) {
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
