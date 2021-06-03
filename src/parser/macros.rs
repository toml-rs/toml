#[doc(hidden)]
#[macro_export]
macro_rules! parse (
    ($name:ident( $($arg: ident :  $arg_type: ty),* ) -> $ret:ty, $code:expr) => (
        parser!{
            pub fn $name['a, I]($($arg : $arg_type),*)(I) -> $ret
                where
                [I: RangeStream<
                 Range = &'a str,
                 Token = char>,
                 I::Error: ParseError<char, &'a str, <I as StreamOnce>::Position>,
                 <I::Error as ParseError<char, &'a str, <I as StreamOnce>::Position>>::StreamError:
                 From<std::num::ParseIntError> +
                 From<std::num::ParseFloatError> +
                 From<chrono::ParseError> +
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
                 Range = &'a str,
                 Token = char>,
                 I::Error: ParseError<char, &'a str, <I as StreamOnce>::Position>,
                 <I::Error as ParseError<char, &'a str, <I as StreamOnce>::Position>>::StreamError:
                 From<std::num::ParseIntError> +
                 From<std::num::ParseFloatError> +
                 From<chrono::ParseError> +
                 From<crate::parser::errors::CustomError>
                ]
            {
                $closure
            }
        }
    );
);
