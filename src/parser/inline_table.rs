use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::key::simple_key;
use crate::parser::trivia::ws;
use crate::parser::value::value;
use crate::repr::{Decor, InternalString, Repr};
use crate::table::TableKeyValue;
use crate::{InlineTable, Item};
use combine::parser::char::char;
use combine::stream::RangeStream;
use combine::*;

// ;; Inline Table

// inline-table = inline-table-open inline-table-keyvals inline-table-close
parse!(inline_table() -> InlineTable, {
    between(char(INLINE_TABLE_OPEN), char(INLINE_TABLE_CLOSE),
            inline_table_keyvals().and_then(|(p, v)| table_from_pairs(p, v)))
});

fn table_from_pairs(preamble: &str, v: Vec<TableKeyValue>) -> Result<InlineTable, CustomError> {
    let mut table = InlineTable {
        preamble: InternalString::from(preamble),
        ..Default::default()
    };

    for kv in v {
        if table.contains_key(kv.key.get()) {
            return Err(CustomError::DuplicateKey {
                key: kv.key.into(),
                table: "inline".into(),
            });
        }
        table.items.insert(kv.key.get().to_owned(), kv);
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

parse!(inline_table_keyvals() -> (&'a str, Vec<TableKeyValue>), {
    (
        sep_by(keyval(), char(INLINE_TABLE_SEP)),
        ws(),
    ).map(|(v, w)| {
        (w, v)
    })
});

parse!(keyval() -> TableKeyValue, {
    (
        attempt((ws(), simple_key(), ws())),
        char(KEYVAL_SEP),
        (ws(), value(), ws()),
    ).map(|(k, _, v)| {
        let (pre, (raw, key), suf) = k;
        let key_decor = Decor::new(pre, suf);
        let key = Key::new_unchecked(Repr::new_unchecked(raw), key, key_decor);

        let (pre, v, suf) = v;
        let v = v.decorated(pre, suf);
        TableKeyValue {
            key,
            value: Item::Value(v),
        }
    })
});
