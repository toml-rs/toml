#[doc(hidden)]
#[macro_export]
macro_rules! parse (
    ($name:ident( $($arg: ident :  $arg_type: ty),* ) -> $ret:ty, $code:expr) => (
        parser!{
            pub fn $name['a, I]($($arg : $arg_type),*)(I) -> $ret
                where
                [I: RangeStream<Range = &'a str, Item = char>,]
            {
                $code
            }
        }
    );
);
