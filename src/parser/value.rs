use nom::{InputLength, Slice, self};
use parser::errors::{ErrorKind, is_custom};
use parser::strings::string;
use parser::datetime::date_time;
use parser::numbers::{boolean, integer, float};
use parser::inline_table::inline_table;
use parser::array::array;
use parser::Span;
use ::value;
use ::formatted;


// val = string / boolean / array / inline-table / date-time / float / integer
named!(pub value(Span) -> value::Value,
       map!(with_input!(alt_custom!(e!(ErrorKind::InvalidValue),
             string          => { |val| value::Value::from(val) }
           | boolean         => { |val| value::Value::from(val) }
           | array           => { |val| value::Value::Array(val) }
           | inline_table    => { |val| value::Value::InlineTable(val) }
           | date_time       => { |val| value::Value::from(val) }
           | float           => { |val| value::Value::from(val) }
           | integer         => { |val| value::Value::from(val) }
         )),
           |(val, raw)| formatted::value(val, raw.fragment)
       )
);
