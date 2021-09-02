use crate::key::Key;
use crate::parser::errors::CustomError;
use crate::parser::key::key;
use crate::parser::table::duplicate_key;
use crate::parser::trivia::ws;
use crate::parser::value::value;
use crate::repr::InternalString;
use crate::table::TableKeyValue;
use crate::{InlineTable, Item, Value};
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
    let mut root = InlineTable {
        preamble: InternalString::from(preamble),
        ..Default::default()
    };

    for (position, (path, mut kv)) in v.into_iter().enumerate() {
        kv.key.set_position(Some(position));

        let table = descend_path(&mut root, &path, 0)?;
        if table.contains_key(kv.key.get()) {
            return Err(CustomError::DuplicateKey {
                key: kv.key.into(),
                table: "inline".into(),
            });
        }
        table.items.insert(kv.key.get().to_owned(), kv);
    }
    Ok(root)
}

fn descend_path<'a>(
    table: &'a mut InlineTable,
    path: &'a [Key],
    i: usize,
) -> Result<&'a mut InlineTable, CustomError> {
    if let Some(key) = path.get(i) {
        let entry = table.entry_format(key).or_insert_with(|| {
            let mut new_table = InlineTable::new();
            new_table.set_dotted(true);

            Value::InlineTable(new_table)
        });
        match *entry {
            Value::InlineTable(ref mut sweet_child_of_mine) => {
                descend_path(sweet_child_of_mine, path, i + 1)
            }
            _ => Err(duplicate_key(path, i)),
        }
    } else {
        Ok(table)
    }
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
});
