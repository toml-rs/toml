use nom;
use parser::errors::ErrorKind;
use parser::trivia::ws_comment_newline;
use parser::value::value;
use parser::{LenWorkaround, Span};
use ::value::{Value, Array};
use ::decor::InternalString;
use ::formatted::decorated;

// ;; Array

// array = array-open array-values array-close
named!(parse_array(Span) -> (Vec<Value>, &str),
       delimited!(
           complete!(tag!(ARRAY_OPEN)),
           array_values,
           err!(ErrorKind::UnterminatedArray,
                complete!(tag!(ARRAY_CLOSE)))
       )
);

fn array_from_vec(trailing: &str, v: Vec<Value>) -> Option<Array> {
    let mut array = Array::default();
    array.trailing = InternalString::from(trailing);
    for val in v {
        if !array.push_value(val, false) {
            return None;
        }
    }
    Some(array)
}

pub fn array(input: Span) -> nom::IResult<Span, Array> {
    let (rest, p) = try_parse!(input, parse_array);

    match array_from_vec(p.1, p.0) {
        Some(a) => nom::IResult::Done(rest, a),
        _ => e!(ErrorKind::MixedArrayType, rest),
    }
}

// note: we're omitting ws and newlines here, because
// they should be part of the formatted values
// array-open  = %x5B ws-newline  ; [
const ARRAY_OPEN: &str = "[";
// array-close = ws-newline %x5D  ; ]
const ARRAY_CLOSE: &str = "]";
// array-sep = ws %x2C ws  ; , Comma
const ARRAY_SEP: &str = ",";

// note: this rule is modified
// array-values = [ ( array-value array-sep array-values ) /
//                  array-value / ws-comment-newline ]

named!(array_values(Span) -> (Vec<Value>, &str),
       do_parse!(
           v: opt!(
               do_parse!(
                v: separated_nonempty_list_complete!(
                       tag!(ARRAY_SEP),
                       array_value
                   ) >>
                t: opt!(trailing) >>
                   (v, t.map(|s| s.fragment).unwrap_or(""))
               )
           ) >>
           w: ws_comment_newline >>
           (v.unwrap_or_else(|| (Vec::new(), w.fragment)))
       )
);

named!(array_value(Span) -> Value,
       do_parse!(
           ws1: ws_comment_newline >>
             v: value >>
           ws2: ws_comment_newline >>
                (decorated(v, ws1.fragment, ws2.fragment))
       )
);

named!(trailing(Span) -> Span,
       recognize!(
           tuple!(
               complete!(tag!(ARRAY_SEP)),
               ws_comment_newline
           )
       )
);
