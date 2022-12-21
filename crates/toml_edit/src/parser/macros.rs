#[doc(hidden)]
#[macro_export]
macro_rules! parse (
    ($name:ident( $($arg: ident :  $arg_type: ty),* ) -> $ret:ty, $code:expr) => (
        parser!{
            pub(crate) fn $name['a, I]($($arg : $arg_type),*)(I) -> $ret
                where
                [I: RangeStream<
                 Range = &'a [u8],
                 Token = u8>,
                 I::Error: ParseError<u8, &'a [u8], <I as StreamOnce>::Position>,
                 <I::Error as ParseError<u8, &'a [u8], <I as StreamOnce>::Position>>::StreamError:
                 From<std::num::ParseIntError> +
                 From<std::num::ParseFloatError> +
                 From<std::str::Utf8Error> +
                 From<$crate::parser::errors::CustomError>
                ]
            {
                $code
            }
        }
    );
);

#[doc(hidden)]
#[macro_export]
macro_rules! toml_parser (
    ($name:ident, $argh:ident, $closure:expr) => (
        parser!{
            fn $name['a, 'b, I]($argh: &'b RefCell<ParseState>)(I) -> ()
            where
                [I: RangeStream<
                 Range = &'a [u8],
                 Token = u8>,
                 I::Error: ParseError<u8, &'a [u8], <I as StreamOnce>::Position>,
                 <I::Error as ParseError<u8, &'a [u8], <I as StreamOnce>::Position>>::StreamError:
                 From<std::num::ParseIntError> +
                 From<std::num::ParseFloatError> +
                 From<std::str::Utf8Error> +
                 From<$crate::parser::errors::CustomError>
                ]
            {
                $closure
            }
        }
    );
);

#[cfg(test)]
macro_rules! parsed_eq {
    ($parsed:ident, $expected:expr) => {{
        assert!($parsed.is_ok(), "{:?}", $parsed.err().unwrap());
        let (v, rest) = $parsed.unwrap();
        assert_eq!(v, $expected);
        assert!(rest.input.is_empty());
    }};
}

#[cfg(test)]
macro_rules! parsed_float_eq {
    ($input:ident, $expected:expr) => {{
        let parsed = crate::parser::numbers::float().easy_parse(Stream::new($input.as_bytes()));
        let (v, rest) = match parsed {
            Ok(parsed) => parsed,
            Err(err) => {
                panic!("Unexpected error for {:?}: {:?}", $input, err);
            }
        };
        if $expected.is_nan() {
            assert!(v.is_nan());
        } else if $expected.is_infinite() {
            assert!(v.is_infinite());
            assert_eq!($expected.is_sign_positive(), v.is_sign_positive());
        } else {
            dbg!($expected);
            dbg!(v);
            assert!(($expected - v).abs() < std::f64::EPSILON);
        }
        assert!(rest.input.is_empty());
    }};
}

#[cfg(test)]
macro_rules! parsed_value_eq {
    ($input:expr) => {
        use combine::EasyParser;
        let parsed = crate::parser::value::value()
            .easy_parse(combine::stream::position::Stream::new($input.as_bytes()));
        let (v, rest) = match parsed {
            Ok(parsed) => parsed,
            Err(err) => {
                panic!("Unexpected error for {:?}: {:?}", $input, err);
            }
        };
        snapbox::assert_eq(v.to_string(), $input);
        assert!(rest.input.is_empty());
    };
}

#[cfg(test)]
macro_rules! parsed_date_time_eq {
    ($input:expr, $is:ident) => {{
        use combine::EasyParser;
        let parsed = crate::parser::value::value()
            .easy_parse(combine::stream::position::Stream::new($input.as_bytes()));
        assert!(parsed.is_ok());
        let (v, rest) = parsed.unwrap();
        snapbox::assert_eq(v.to_string(), $input);
        assert!(rest.input.is_empty());
        assert!(v.$is());
    }};
}
