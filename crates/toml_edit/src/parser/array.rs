use combine::parser::byte::byte;
use combine::parser::range::recognize_with_value;
use combine::stream::RangeStream;
use combine::*;

use crate::parser::trivia::ws_comment_newline;
use crate::parser::value::value;
use crate::{Array, Value};

// ;; Array

// array = array-open array-values array-close
parse!(array() -> Array, {
    between(byte(ARRAY_OPEN), byte(ARRAY_CLOSE),
            array_values())
});

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
parse!(array_values() -> Array, {
    (
        optional(
            recognize_with_value(
                sep_end_by1(array_value(), byte(ARRAY_SEP))
            ).map(|(r, v): (&'a [u8], Array)| (v, r[r.len() - 1] == b','))
        ),
        ws_comment_newline(),
    ).and_then::<_, _, std::str::Utf8Error>(|(array, trailing)| {
        let (mut array, comma) = array.unwrap_or_default();
        array.set_trailing_comma(comma);
        array.set_trailing(std::str::from_utf8(trailing)?);
        Ok(array)
    })
});

parse!(array_value() -> Value, {
    attempt((
        ws_comment_newline(),
        value(),
        ws_comment_newline(),
    )).and_then::<_, _, std::str::Utf8Error>(|(ws1, v, ws2)| {
        let v = v.decorated(
            std::str::from_utf8(ws1)?,
            std::str::from_utf8(ws2)?,
        );
        Ok(v)
    })
});

#[cfg(test)]
mod test {
    use super::*;

    use combine::stream::position::Stream;

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
            parsed_value_eq!(input);
        }

        let invalid_inputs = [r#"["#, r#"[,]"#, r#"[,2]"#, r#"[1e165,,]"#];
        for input in invalid_inputs {
            let parsed = array().easy_parse(Stream::new(input.as_bytes()));
            assert!(parsed.is_err());
        }
    }
}
