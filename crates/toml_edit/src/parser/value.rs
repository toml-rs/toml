use crate::parser::array::array;
use crate::parser::datetime::date_time;
use crate::parser::inline_table::inline_table;
use crate::parser::numbers::{float, integer};
use crate::parser::strings::string;
use crate::parser::trivia::from_utf8_unchecked;
use crate::repr::{Formatted, Repr};
use crate::value as v;
use crate::Value;
use combine::parser::range::recognize_with_value;
use combine::stream::RangeStream;
use combine::*;

// val = string / boolean / array / inline-table / date-time / float / integer
parse!(value() -> v::Value, {
    recognize_with_value(look_ahead(any()).then(|e| {
        dispatch!(e;
            crate::parser::strings::QUOTATION_MARK |
            crate::parser::strings::APOSTROPHE => string().map(|s| {
                v::Value::String(Formatted::new(
                    s,
                ))
            }),
            crate::parser::array::ARRAY_OPEN => array().map(v::Value::Array),
            crate::parser::inline_table::INLINE_TABLE_OPEN => inline_table().map(v::Value::InlineTable),
            // Date/number starts
            b'+' | b'-' | b'0'..=b'9' |
            // Report as if they were numbers because its most likely a typo
            b'.' | b'_' => {
                // Uncommon enough not to be worth optimizing at this time
                choice((
                    date_time()
                        .map(v::Value::from),
                    float()
                        .map(v::Value::from),
                    integer()
                        .map(v::Value::from),
                ))
            },
            b't' => {
                crate::parser::numbers::true_().map(v::Value::from).expected("quoted string")
            },
            b'f' => {
                crate::parser::numbers::false_().map(v::Value::from).expected("quoted string")
            },
            b'i' => {
                crate::parser::numbers::inf().map(v::Value::from).expected("quoted string")
            },
            b'n' => {
                crate::parser::numbers::nan().map(v::Value::from).expected("quoted string")
            },
            _ => {
                // Pick something random to fail, we'll override `expected` anyways
                crate::parser::numbers::nan().map(v::Value::from).expected("quoted string")
            },
        )
    })).and_then(|(raw, value)| apply_raw(value, raw))
});

fn apply_raw(mut val: Value, raw: &[u8]) -> Result<Value, std::str::Utf8Error> {
    match val {
        Value::String(ref mut f) => {
            let raw = std::str::from_utf8(raw)?;
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Integer(ref mut f) => {
            let raw = unsafe { from_utf8_unchecked(raw, "`integer()` filters out non-ASCII") };
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Float(ref mut f) => {
            let raw = unsafe { from_utf8_unchecked(raw, "`float()` filters out non-ASCII") };
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Boolean(ref mut f) => {
            let raw = unsafe { from_utf8_unchecked(raw, "`boolean()` filters out non-ASCII") };
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Datetime(ref mut f) => {
            let raw = unsafe { from_utf8_unchecked(raw, "`date_time()` filters out non-ASCII") };
            f.set_repr_unchecked(Repr::new_unchecked(raw));
        }
        Value::Array(_) | Value::InlineTable(_) => {}
    };
    val.decorate("", "");
    Ok(val)
}
