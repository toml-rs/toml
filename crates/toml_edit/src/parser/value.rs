use nom8::branch::alt;
use nom8::bytes::any;
use nom8::combinator::fail;
use nom8::combinator::peek;

use crate::parser::array::array;
use crate::parser::datetime::date_time;
use crate::parser::inline_table::inline_table;
use crate::parser::numbers::{float, integer};
use crate::parser::prelude::*;
use crate::parser::strings::string;
use crate::parser::trivia::from_utf8_unchecked;
use crate::repr::{Formatted, Repr};
use crate::value as v;
use crate::Value;

// val = string / boolean / array / inline-table / date-time / float / integer
pub(crate) fn value(
    check: RecursionCheck,
) -> impl FnMut(Input<'_>) -> IResult<Input<'_>, v::Value, ParserError<'_>> {
    move |input| {
        dispatch!{peek(any);
            crate::parser::strings::QUOTATION_MARK |
            crate::parser::strings::APOSTROPHE => string.map(|s| {
                v::Value::String(Formatted::new(
                    s.into_owned()
                ))
            }),
            crate::parser::array::ARRAY_OPEN => array(check).map(v::Value::Array),
            crate::parser::inline_table::INLINE_TABLE_OPEN => inline_table(check).map(v::Value::InlineTable),
            // Date/number starts
            b'+' | b'-' | b'0'..=b'9' => {
                // Uncommon enough not to be worth optimizing at this time
                alt((
                    date_time
                        .map(v::Value::from),
                    float
                        .map(v::Value::from),
                    integer
                        .map(v::Value::from),
                ))
            },
            // Report as if they were numbers because its most likely a typo
            b'_' => {
                    integer
                        .map(v::Value::from)
                .context(Context::Expected(ParserValue::Description("leading digit")))
            },
            // Report as if they were numbers because its most likely a typo
            b'.' =>  {
                    float
                        .map(v::Value::from)
                .context(Context::Expected(ParserValue::Description("leading digit")))
            },
            b't' => {
                crate::parser::numbers::true_.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            b'f' => {
                crate::parser::numbers::false_.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            b'i' => {
                crate::parser::numbers::inf.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            b'n' => {
                crate::parser::numbers::nan.map(v::Value::from)
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
            _ => {
                fail
                    .context(Context::Expression("string"))
                    .context(Context::Expected(ParserValue::CharLiteral('"')))
                    .context(Context::Expected(ParserValue::CharLiteral('\'')))
            },
    }
        .with_recognized()
        .map_res(|(value, raw)| apply_raw(value, raw))
        .parse(input)
    }
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn values() {
        let inputs = [
            "1979-05-27T00:32:00.999999",
            "-239",
            "1e200",
            "9_224_617.445_991_228_313",
            r#"'''I [dw]on't need \d{2} apples'''"#,
            r#"'''
The first newline is
trimmed in raw strings.
   All other whitespace
   is preserved.
'''"#,
            r#""Jos\u00E9\n""#,
            r#""\\\"\b/\f\n\r\t\u00E9\U000A0000""#,
            r#"{ hello = "world", a = 1}"#,
            r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        ];
        for input in inputs {
            let parsed = value(Default::default()).parse(new_input(input)).finish();
            assert_eq!(parsed.map(|a| a.to_string()), Ok(input.to_owned()));
        }
    }
}
