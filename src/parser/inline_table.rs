use combine::*;
use combine::char::char;
use combine::primitives::RangeStream;
use parser::errors::CustomError;
use parser::trivia::ws;
use parser::key::key;
use parser::value::value;
use value::{InlineTable, KeyValue};
use decor::{InternalString, Repr};
use formatted::decorated;

// ;; Inline Table

// inline-table = inline-table-open inline-table-keyvals inline-table-close
parse!(inline_table() -> InlineTable, {
    between(char(INLINE_TABLE_OPEN), char(INLINE_TABLE_CLOSE),
            inline_table_keyvals().and_then(|(p, v)| table_from_pairs(p, v)))
});

fn table_from_pairs(
    preamble: &str,
    v: Vec<(InternalString, KeyValue)>,
) -> Result<InlineTable, CustomError> {
    let mut table = InlineTable::default();
    table.preamble = InternalString::from(preamble);

    for (k, kv) in v {
        if table.contains_key(&k) {
            return Err(CustomError::DuplicateKey {
                key: k.into(),
                table: "inline".into(),
            });
        }
        table.key_value_pairs.insert(k, kv);
    }
    Ok(table)
}

// inline-table-open  = %x7B ws     ; {
const INLINE_TABLE_OPEN: char = '{';
// inline-table-close = ws %x7D     ; }
const INLINE_TABLE_CLOSE: char = '}';
// inline-table-sep   = ws %x2C ws  ; , Comma
const INLINE_TABLE_SEP: char = ',';
// keyval-sep = ws %x3D ws ; =
pub(crate) const KEYVAL_SEP: char = '=';

// inline-table-keyvals = [ inline-table-keyvals-non-empty ]
// inline-table-keyvals-non-empty =
// ( key keyval-sep val inline-table-sep inline-table-keyvals-non-empty ) /
// ( key keyval-sep val )
parser!{
    fn inline_table_keyvals['a, I]()(I) -> (&'a str, Vec<(InternalString, KeyValue)>)
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        (
            sep_by(keyval(), char(INLINE_TABLE_SEP)),
            ws(),
        ).map(|(v, w)| {
            (w, v)
        })
    }
}

parser!{
    fn keyval['a, I]()(I) -> (InternalString, KeyValue)
        where
        [I: RangeStream<Range = &'a str, Item = char>,]
    {
        (
            try((ws(), key(), ws())),
            char(KEYVAL_SEP),
            (ws(), value(), ws()),
        ).map(|(k, _, v)| {
            let (pre, v, suf) = v;
            let v = decorated(v, pre, suf);
            let (pre, (raw, key), suf) = k;
            (
                key,
                KeyValue {
                    key: Repr::new(pre, raw, suf),
                    value: v,
                }
            )
        })
    }
}
