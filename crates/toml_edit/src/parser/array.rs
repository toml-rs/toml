use nom8::combinator::cut;
use nom8::combinator::opt;
use nom8::multi::separated_list1;
use nom8::sequence::delimited;

use crate::parser::trivia::ws_comment_newline;
use crate::parser::value::value;
use crate::{Array, Item, Value};

use crate::parser::prelude::*;

// ;; Array

// array = array-open array-values array-close
pub(crate) fn array(input: Input<'_>) -> IResult<Input<'_>, Array, ParserError<'_>> {
    delimited(
        ARRAY_OPEN,
        cut(array_values),
        cut(ARRAY_CLOSE)
            .context(Context::Expression("array"))
            .context(Context::Expected(ParserValue::CharLiteral(']'))),
    )
    .parse(input)
}

// note: we're omitting ws and newlines here, because
// they should be part of the formatted values
// array-open  = %x5B ws-newline  ; [
pub(crate) const ARRAY_OPEN: u8 = b'[';
// array-close = ws-newline %x5D  ; ]
const ARRAY_CLOSE: u8 = b']';
// array-sep = ws %x2C ws  ; , Comma
const ARRAY_SEP: u8 = b',';

// note: this rule is modified
// array-values = [ ( array-value array-sep array-values ) /
//                  array-value / ws-comment-newline ]
pub(crate) fn array_values(input: Input<'_>) -> IResult<Input<'_>, Array, ParserError<'_>> {
    (
        opt(
            (separated_list1(ARRAY_SEP, array_value), opt(ARRAY_SEP)).map(
                |(v, trailing): (Vec<Value>, Option<u8>)| {
                    (
                        Array::with_vec(v.into_iter().map(Item::Value).collect()),
                        trailing.is_some(),
                    )
                },
            ),
        ),
        ws_comment_newline,
    )
        .map_res::<_, _, std::str::Utf8Error>(|(array, trailing)| {
            let (mut array, comma) = array.unwrap_or_default();
            array.set_trailing_comma(comma);
            array.set_trailing(std::str::from_utf8(trailing)?);
            Ok(array)
        })
        .parse(input)
}

pub(crate) fn array_value(input: Input<'_>) -> IResult<Input<'_>, Value, ParserError<'_>> {
    (ws_comment_newline, value, ws_comment_newline)
        .map_res::<_, _, std::str::Utf8Error>(|(ws1, v, ws2)| {
            let v = v.decorated(std::str::from_utf8(ws1)?, std::str::from_utf8(ws2)?);
            Ok(v)
        })
        .parse(input)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn arrays() {
        let inputs = [
            r#"[]"#,
            r#"[   ]"#,
            r#"[
  1, 2, 3
]"#,
            r#"[
  1,
  2, # this is ok
]"#,
            r#"[# comment
# comment2


   ]"#,
            r#"[# comment
# comment2
      1

#sd
,
# comment3

   ]"#,
            r#"[1]"#,
            r#"[1,]"#,
            r#"[ "all", 'strings', """are the same""", '''type''']"#,
            r#"[ 100, -2,]"#,
            r#"[1, 2, 3]"#,
            r#"[1.1, 2.1, 3.1]"#,
            r#"["a", "b", "c"]"#,
            r#"[ [ 1, 2 ], [3, 4, 5] ]"#,
            r#"[ [ 1, 2 ], ["a", "b", "c"] ]"#,
            r#"[ { x = 1, a = "2" }, {a = "a",b = "b",     c =    "c"} ]"#,
        ];
        for input in inputs {
            let parsed = array.parse(input.as_bytes()).finish();
            assert_eq!(parsed.map(|a| a.to_string()), Ok(input.to_owned()));
        }

        let invalid_inputs = [r#"["#, r#"[,]"#, r#"[,2]"#, r#"[1e165,,]"#];
        for input in invalid_inputs {
            let parsed = array.parse(input.as_bytes()).finish();
            assert!(parsed.is_err());
        }
    }
}
