use std::cell::RefCell;

use combine::parser::byte::byte;
use combine::parser::range::range;
use combine::stream::RangeStream;
use combine::*;

use crate::parser::key::key;
use crate::parser::trivia::line_trailing;
use crate::parser::ParseState;

// std-table-open  = %x5B ws     ; [ Left square bracket
pub(crate) const STD_TABLE_OPEN: u8 = b'[';
// std-table-close = ws %x5D     ; ] Right square bracket
const STD_TABLE_CLOSE: u8 = b']';
// array-table-open  = %x5B.5B ws  ; [[ Double left square bracket
const ARRAY_TABLE_OPEN: &[u8] = b"[[";
// array-table-close = ws %x5D.5D  ; ]] Double right quare bracket
const ARRAY_TABLE_CLOSE: &[u8] = b"]]";

// ;; Standard Table

// std-table = std-table-open key *( table-key-sep key) std-table-close
toml_parser!(std_table, parser, {
    (
        between(byte(STD_TABLE_OPEN), byte(STD_TABLE_CLOSE), key()),
        line_trailing().and_then(std::str::from_utf8),
    )
        .and_then(|(h, t)| parser.borrow_mut().on_std_header(h, t))
});

// ;; Array Table

// array-table = array-table-open key *( table-key-sep key) array-table-close
toml_parser!(array_table, parser, {
    (
        between(range(ARRAY_TABLE_OPEN), range(ARRAY_TABLE_CLOSE), key()),
        line_trailing().and_then(std::str::from_utf8),
    )
        .and_then(|(h, t)| parser.borrow_mut().on_array_header(h, t))
});

// ;; Table

// table = std-table / array-table
parser! {
    pub(crate) fn table['a, 'b, I](parser: &'b RefCell<ParseState>)(I) -> ()
    where
        [I: RangeStream<
         Range = &'a [u8],
         Token = u8>,
         I::Error: ParseError<u8, &'a [u8], <I as StreamOnce>::Position>,
         <I::Error as ParseError<u8, &'a [u8], <I as StreamOnce>::Position>>::StreamError:
         From<std::num::ParseIntError> +
         From<std::num::ParseFloatError> +
         From<std::str::Utf8Error> +
         From<crate::parser::errors::CustomError>
    ]    {
        array_table(parser)
            .or(std_table(parser))
            .message("While parsing a Table Header")
    }
}
