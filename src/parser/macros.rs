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
                 From<crate::parser::errors::CustomError>
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
            fn $name['a, 'b, I]($argh: &'b RefCell<TomlParser>)(I) -> ()
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
                ]
            {
                $closure
            }
        }
    );
);
