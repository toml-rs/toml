use combine::*;
use combine::primitives::RangeStream;
use combine::range::recognize_with_value;
use parser::strings::string;
use parser::datetime::date_time;
use parser::numbers::{boolean, integer, float};
use parser::inline_table::inline_table;
use parser::array::array;
use ::value as v;
use ::formatted;


// val = string / boolean / array / inline-table / date-time / float / integer
parse!(value() -> v::Value, {
    recognize_with_value(choice((
        string()
            .map(v::Value::from),
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
