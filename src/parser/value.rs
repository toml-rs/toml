use crate::parser::array::array;
use crate::parser::datetime::date_time;
use crate::parser::inline_table::inline_table;
use crate::parser::numbers::{boolean, float, integer};
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
    recognize_with_value(choice((
        string()
            .map(|s|
                v::Value::String(Formatted::new(
                    s,
                ))
            ),
        boolean()
            .map(v::Value::from),
        array()
            .map(v::Value::Array),
        inline_table()
            .map(v::Value::InlineTable),
        date_time()
            .map(v::Value::from),
        float()
            .map(v::Value::from),
        integer()
            .map(v::Value::from),
    ))).and_then(|(raw, value)| apply_raw(value, raw))
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
