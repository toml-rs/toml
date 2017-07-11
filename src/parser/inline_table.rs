use nom;
use parser::errors::ErrorKind;
use parser::trivia::ws;
use parser::key::key;
use parser::value::value;
use parser::{LenWorkaround, Span};
use ::value::{KeyValue, InlineTable};
use ::decor::{InternalString, Repr};
use ::formatted::decorated;

// ;; Inline Table

// inline-table = inline-table-open inline-table-keyvals inline-table-close
named!(parse_inline_table(Span) -> (&str, Vec<(InternalString, KeyValue)>),
       delimited!(
           complete!(tag!(INLINE_TABLE_OPEN)),
           inline_table_keyvals,
           err!(ErrorKind::UnterminatedInlineTable,
                complete!(tag!(INLINE_TABLE_CLOSE)))
       )
);

fn table_from_pairs(preamble: &str, v: Vec<(InternalString, KeyValue)>) -> Option<InlineTable> {
    let mut table = InlineTable::default();
    table.preamble = InternalString::from(preamble);
    for (k, kv) in v {
        if table.contains_key(&k) {
            return None;
        }
        table.key_value_pairs.insert(k, kv);
    }
    Some(table)
}

pub fn inline_table(input: Span) -> nom::IResult<Span, InlineTable> {
    let (rest, p) = try_parse!(input, parse_inline_table);

    match table_from_pairs(p.0, p.1) {
        Some(a) => nom::IResult::Done(rest, a),
        _ => e!(ErrorKind::DuplicateKey, rest),
    }
}

// inline-table-open  = %x7B ws     ; {
const INLINE_TABLE_OPEN: &str = "{";
// inline-table-close = ws %x7D     ; }
const INLINE_TABLE_CLOSE: &str = "}";
// inline-table-sep   = ws %x2C ws  ; , Comma
const INLINE_TABLE_SEP: &str = ",";
// keyval-sep = ws %x3D ws ; =
pub(crate) const KEYVAL_SEP: &str = "=";

// inline-table-keyvals = [ inline-table-keyvals-non-empty ]
// inline-table-keyvals-non-empty = ( key keyval-sep val inline-table-sep inline-table-keyvals-non-empty ) /
//                                  ( key keyval-sep val )
named!(inline_table_keyvals(Span) -> (&str, Vec<(InternalString, KeyValue)>),
       do_parse!(
        v: opt!(separated_nonempty_list_complete!(
               tag!(INLINE_TABLE_SEP),
               keyval
           )) >>
        w: ws >>
           (w.fragment, v.unwrap_or_default())
       )
);

named!(keyval(Span) -> (InternalString, KeyValue),
       do_parse!(
        k: tuple!(ws, key, ws)   >>
           err!(ErrorKind::ExpectedEquals,
                complete!(tag!(KEYVAL_SEP))) >>
        v: tuple!(ws, value, ws) >>
           ({
               let (pre, v, suf) = v;
               let v = decorated(v, pre.fragment, suf.fragment);
               let (pre, (key, raw), suf) = k;
               (key, KeyValue {
                   key: Repr::new(pre.fragment, raw.fragment, suf.fragment),
                   value: v,
               })
           })
       )
);
