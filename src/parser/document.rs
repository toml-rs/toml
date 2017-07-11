use nom;
use std::mem;
use parser::{Span, Parser};
use parser::errors::{Error, ErrorKind, is_custom, to_error};
use parser::trivia::{ws, line_trailing, line_ending, comment};
use parser::key::key;
use parser::value::value;
use parser::inline_table::KEYVAL_SEP;
use ::decor::{InternalString, Repr};
use ::document::Document;
use ::value::KeyValue;
use ::formatted::decorated;

macro_rules! try_parser (
    ($parser:ident, $span:ident, $method:ident) => (
        {
            let (p, res) = $parser.$method($span);
            $parser = p;
            match res {
                nom::IResult::Done(rest, _) => {
                    if rest.fragment.len() == 0 {
                        return Ok($parser.document);
                    }
                    $span = rest;
                    continue;
                }
                nom::IResult::Error(e) => {
                    if is_custom(&e) {
                        return Err(to_error(&e));
                    }
                }
                _ => unreachable!("expressions are complete"),
            }
        }
    );
);

impl Parser {
    // ;; TOML

    // toml = expression *( newline expression )

    // expression = ( ( ws comment ) /
    //                ( ws keyval ws [ comment ] ) /
    //                ( ws table ws [ comment ] ) /
    //                  ws )
    pub fn parse(s: &str) -> Result<Document, Error> {
        let mut parser = Self::default();
        let mut span = Span::new(s);
        loop {
            try_parser!(parser, span, ws_comment);
            try_parser!(parser, span, keyval);
            try_parser!(parser, span, table);
            try_parser!(parser, span, ws);
            return Err(Error::new(ErrorKind::Unknown, span));
        }
    }

    method!(ws_comment<Parser>(Span) -> (), mut self,
            do_parse!(
                w: ws >>
                c: comment >>
                e: line_ending >>
                   ({
                       self.document.trailing.push_str(w.fragment);
                       self.document.trailing.push_str(c.fragment);
                       self.document.trailing.push_str(e.fragment);
                   })
            )
    );

    method!(ws<Parser>(Span) -> (), mut self,
            do_parse!(
              w: ws >>
              e: line_ending >>
                 ({
                     self.document.trailing.push_str(w.fragment);
                     self.document.trailing.push_str(e.fragment);
                 })
            )
    );

    method!(keyval<Parser>(Span) -> (), mut self,
            do_parse!(
                p: call_m!(self.parse_keyval) >>
                   err_parser!(self, Self::on_keyval,
                               call!(p)) >>
                   ()
            )
    );


    fn on_keyval<'a>(
        rest: Span<'a>,
        me: &mut Parser,
        keyval: (Span<'a>, InternalString, KeyValue),
    ) -> nom::IResult<Span<'a>, ()>
    {
        let table = unsafe { &mut *me.current_table };
        let (span, key, kv) = keyval;
        if table.contains_key(&key) {
            e!(ErrorKind::DuplicateKey, span)
        } else {
            table.key_value_pairs.insert(key, kv);
            nom::IResult::Done(rest, ())
        }
    }

    // keyval = key keyval-sep val
    method!(parse_keyval<Parser>(Span) -> (Span, InternalString, KeyValue), mut self,
       do_parse!(
           k: complete!(tuple!(ws, key, ws))      >>
              err_m!(self, ErrorKind::ExpectedEquals,
                     complete!(tag!(KEYVAL_SEP))) >>
           v: tuple!(ws, value, line_trailing)    >>
               ({
                   let (pre, v, suf) = v;
                   let v = decorated(v, pre.fragment, suf.fragment);
                   let (pre, (key, raw), suf) = k;
                   let prefix = mem::replace(&mut self.document.trailing, InternalString::new());
                   let prefix = prefix + pre.fragment;
                   (raw, key, KeyValue {
                       key: Repr::new(prefix, raw.fragment.into(), suf.fragment.into()),
                       value: v,
                   })
               })
       )
    );
}
