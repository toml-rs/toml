use crate::decor::{Formatted, Repr};
use crate::formatted;
use crate::parser::array::array;
use crate::parser::datetime::date_time;
use crate::parser::inline_table::inline_table;
use crate::parser::numbers::{boolean, float, integer};
use crate::parser::strings::string;
use crate::value as v;
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
                     Repr::new("".to_string(), "who cares?".into(), "".to_string()),
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
    ))).map(|(raw, value)| formatted::value(value, raw))
});
