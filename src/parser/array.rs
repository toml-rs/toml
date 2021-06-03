use crate::decor::InternalString;
use crate::formatted::decorated;
use crate::parser::errors::CustomError;
use crate::parser::trivia::ws_comment_newline;
use crate::parser::value::value;
use crate::value::{Array, Value};
use combine::parser::char::char;
use combine::parser::range::recognize_with_value;
use combine::stream::RangeStream;
use combine::*;

// ;; Array

// array = array-open array-values array-close
parse!(array() -> Array, {
    between(char(ARRAY_OPEN), char(ARRAY_CLOSE),
            array_values().and_then(|(v, c, t)| array_from_vec(v, c, t)))
});

fn array_from_vec(v: Vec<Value>, comma: bool, trailing: &str) -> Result<Array, CustomError> {
    let mut array = Array {
        trailing_comma: comma,
        trailing: InternalString::from(trailing),
        ..Default::default()
    };
    for val in v {
        let err = Err(CustomError::MixedArrayType {
            got: format!("{:?}", val.get_type()),
            expected: format!("{:?}", array.value_type()),
        });
        if array.push_formatted(val).is_err() {
            return err;
        }
    }
    Ok(array)
}

// note: we're omitting ws and newlines here, because
// they should be part of the formatted values
// array-open  = %x5B ws-newline  ; [
const ARRAY_OPEN: char = '[';
// array-close = ws-newline %x5D  ; ]
const ARRAY_CLOSE: char = ']';
// array-sep = ws %x2C ws  ; , Comma
const ARRAY_SEP: char = ',';

// note: this rule is modified
// array-values = [ ( array-value array-sep array-values ) /
//                  array-value / ws-comment-newline ]
parse!(array_values() -> (Vec<Value>, bool, &'a str), {
    (
        optional(
            recognize_with_value(
                sep_end_by1(array_value(), char(ARRAY_SEP))
            ).map(|(r, v): (&'a str, _)| (v, r.ends_with(',')))
        ),
        ws_comment_newline(),
    ).map(|(v, t)| {
        let (v, c) = v.unwrap_or_default();
        (v, c, t)
    })
});

parse!(array_value() -> Value, {
    attempt((
        ws_comment_newline(),
        value(),
        ws_comment_newline(),
    )).map(|(ws1, v, ws2)| decorated(v, ws1, ws2))
});
