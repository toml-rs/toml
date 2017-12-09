use combine::*;
use combine::Parser;
use combine::char::char;
use combine::range::recognize;
use combine::primitives::RangeStream;
use parser::{TomlError, TomlParser};
use parser::errors::CustomError;
use parser::trivia::{comment, line_ending, line_trailing, newline, ws};
use parser::key::key;
use parser::value::value;
use parser::table::table;
use parser::inline_table::KEYVAL_SEP;
use decor::{InternalString, Repr};
use document::Document;
use table::{Item, TableKeyValue};
use formatted::decorated;
use std::mem;
use std::cell::RefCell;
// https://github.com/rust-lang/rust/issues/41358
#[allow(unused_imports)]
use std::ops::DerefMut;


parser!{
    fn parse_comment['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        (
            comment(),
            line_ending(),
        ).map(|(c, e)|
              parser
              .borrow_mut()
              .deref_mut()
              .on_comment(c, e))
    }
}

parser!{
    fn parse_ws['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        ws()
            .map(|w|
                 parser
                 .borrow_mut()
                 .deref_mut()
                 .on_ws(w))
    }
}

parser!{
    fn parse_newline['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        recognize(newline())
            .map(|w|
                 parser
                 .borrow_mut()
                 .deref_mut()
                 .on_ws(w))
    }
}

parser!{
    fn keyval['a, 'b, I](parser: &'b RefCell<TomlParser>)(I) -> ()
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        parse_keyval().and_then(|(k, kv)|
                                parser
                                .borrow_mut()
                                .deref_mut()
                                .on_keyval(k, kv))
    }
}

// keyval = key keyval-sep val
parser!{
    fn parse_keyval['a, I]()(I) -> (InternalString, TableKeyValue)
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        (
            (key(), ws()),
            char(KEYVAL_SEP),
            (ws(), value(), line_trailing())
        ).map(|(k, _, v)| {
            let (pre, v, suf) = v;
            let v = decorated(v, pre, suf);
            let ((raw, key), suf) = k;
            (
                key,
                TableKeyValue {
                    key: Repr::new("", raw, suf),
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
    pub fn parse(s: &str) -> Result<Document, TomlError> {
        let parser = RefCell::new(Self::default());
        let input = State::new(s);

        let parsed = parse_ws(&parser)
            .with(choice((
                eof(),
                skip_many1(
                    choice((
                        parse_comment(&parser),
                        keyval(&parser),
                        table(&parser),
                        parse_newline(&parser),
                    )).skip(parse_ws(&parser)),
                ),
            )))
            .parse(input);
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

    fn on_keyval(&mut self, key: InternalString, mut kv: TableKeyValue) -> Result<(), CustomError> {
        let table = unsafe { &mut *self.current_table };

        let prefix = mem::replace(&mut self.document.trailing, InternalString::new());
        kv.key.decor.prefix = prefix + &kv.key.decor.prefix;

        if table.contains_key(&key) {
            Err(CustomError::DuplicateKey {
                key: key,
                table: "<unknown>".into(), // TODO: get actual table name
            })
        } else {
            let tkv = TableKeyValue {
                key: kv.key,
                value: kv.value,
            };
            table.items.insert(key, tkv);
            Ok(())
        }
    }
}
