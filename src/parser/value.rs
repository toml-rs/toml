use crate::parser::array::array;
use crate::parser::datetime::date_time;
use crate::parser::inline_table::inline_table;
use crate::parser::numbers::{boolean, float, integer};
use crate::parser::strings::string;
use crate::repr::{Decor, Formatted, InternalString, Repr};
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
                    Repr::new("who cares?"),
                    Decor::new("", ""),
                ))
            ),
        boolean()
            .map(v::Value::from),
        array()
            .map(v::Value::Array),
        inline_table()
            .map(v::Value::InlineTable),
        date_time(),
        float()
            .map(v::Value::from),
        integer()
            .map(v::Value::from),
    ))).map(|(raw, value)| apply_raw(value, raw))
});

fn apply_raw(mut val: Value, raw: &str) -> Value {
    match val {
        Value::String(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::Integer(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::Float(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::Boolean(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::OffsetDateTime(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::LocalDateTime(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::LocalDate(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::LocalTime(ref mut f) => {
            f.repr.raw_value = InternalString::from(raw);
        }
        Value::Array(_) | Value::InlineTable(_) => {}
    };
    val.decorate("", "");
    val
}
