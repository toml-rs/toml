use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::key::key;
use crate::parser::table::duplicate_key;
use crate::parser::trivia::ws;
use crate::parser::value::value;
use crate::table::TableKeyValue;
use crate::{InlineTable, InternalString, Item, Value};
use combine::parser::char::char;
use combine::stream::RangeStream;
use combine::*;

// ;; Inline Table

// inline-table = inline-table-open inline-table-keyvals inline-table-close
parse!(inline_table() -> InlineTable, {
    between(char(INLINE_TABLE_OPEN), char(INLINE_TABLE_CLOSE),
            inline_table_keyvals().and_then(|(kv, p)| table_from_pairs(kv, p)))
});

fn table_from_pairs(
    v: Vec<(Vec<Key>, TableKeyValue)>,
    preamble: &str,
) -> Result<InlineTable, CustomError> {
    let mut root = InlineTable::new();
    root.preamble = InternalString::from(preamble);
    // Assuming almost all pairs will be directly in `root`
    root.items.reserve(v.len());

    for (path, kv) in v {
        let table = descend_path(&mut root, &path)?;
        let key: InternalString = kv.key.get_internal().into();
        let old = table.items.insert(key.clone(), kv);
        let duplicate_key = old.is_some();
        if duplicate_key {
            return Err(CustomError::DuplicateKey {
                key: key.as_str().into(),
                table: "inline".into(),
            });
        }
    }
    Ok(root)
}

fn descend_path<'a>(
    mut table: &'a mut InlineTable,
    path: &'a [Key],
) -> Result<&'a mut InlineTable, CustomError> {
    for (i, key) in path.iter().enumerate() {
        let entry = table.entry_format(key).or_insert_with(|| {
            let mut new_table = InlineTable::new();
            new_table.set_dotted(true);

            Value::InlineTable(new_table)
        });
        match *entry {
            Value::InlineTable(ref mut sweet_child_of_mine) => {
                table = sweet_child_of_mine;
            }
            _ => {
                return Err(duplicate_key(path, i));
            }
        }
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

parse!(inline_table_keyvals() -> (Vec<(Vec<Key>, TableKeyValue)>, &'a str), {
    (
        sep_by(keyval(), char(INLINE_TABLE_SEP)),
        ws(),
    )
});

parse!(keyval() -> (Vec<Key>, TableKeyValue), {
    (
        key(),
        char(KEYVAL_SEP),
        (ws(), value(), ws()),
    ).map(|(key, _, v)| {
        let mut path = key;
        let key = path.pop().expect("grammar ensures at least 1");

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
});
