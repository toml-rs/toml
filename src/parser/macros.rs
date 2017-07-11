// generic

#[doc(hidden)]
#[macro_export]
macro_rules! with_input (
    ($input:expr, $submac:ident!( $($args:tt)*)) => (
        {
            match $submac!($input, $($args)*) {
                nom::IResult::Done(rest, s) =>
                    nom::IResult::Done(rest, (s, $input.slice(..$input.input_len() - rest.input_len()))),
                nom::IResult::Incomplete(e) => nom::IResult::Incomplete(e),
                nom::IResult::Error(e) => nom::IResult::Error(e),
            }
        }
    );
);

// custom errors

#[doc(hidden)]
#[macro_export]
macro_rules! err (
    ($input:expr, $code:expr, $submac:ident!( $($args:tt)*)) => (
        {
            return_error!($input, nom::ErrorKind::Custom($code as u32), $submac!($($args)*))
        }
    );
);

#[doc(hidden)]
#[macro_export]
macro_rules! e (
    ($code:expr, $i:expr) => (
        e_kind!(nom::ErrorKind::Custom($code as u32), $i)
    );
);

#[doc(hidden)]
macro_rules! e_kind (
    ($kind:expr, $i:expr) => (
        nom::IResult::Error(error_position!($kind, $i))
    );
);

#[doc(hidden)]
#[macro_export]
macro_rules! err_m (
    ($i:expr, $self_:ident, $code:expr, $submac:ident!( $($args:tt)* )) => (
        {
            let i_ = $i.clone();
            let cl = || {
                $submac!(i_, $($args)*)
            };

            match cl() {
                nom::IResult::Incomplete(x) => nom::IResult::Incomplete(x),
                nom::IResult::Done(i, o)    => nom::IResult::Done(i, o),
                nom::IResult::Error(_)      => {
                    return ($self_, e!($code, $i));
                }
            }
        }
    );
    ($i:expr, $self_:ident, $code:expr, $f:expr) => (
        err_m!($i, $self_, $code, call!($f));
    );
);

#[doc(hidden)]
#[macro_export]
macro_rules! err_parser (
    ($i:expr, $self_:ident, $f:path, $submac:ident!( $($args:tt)* )) => (
        {
            let i_ = $i.clone();
            let res = {
                let cl = || {
                    $submac!(i_, $f, &mut $self_, $($args)*)
                };
                cl()
            };

            match res {
                nom::IResult::Incomplete(x) => nom::IResult::Incomplete(x),
                nom::IResult::Done(i, o)    => nom::IResult::Done(i, o),
                nom::IResult::Error(e)      => {
                    return ($self_, nom::IResult::Error(e));
                }
            }
        }
    );
);

// propagate early return on custom error

#[doc(hidden)]
#[macro_export]
macro_rules! alt_custom (
    (__impl $i:expr, $err:ident!( $code:expr ), $e:ident, $($rest:tt)* ) => (
        alt_custom!(__impl $i, $err!($code), call!($e) , $($rest)*);
    );
    (__impl $i:expr, $err:ident!( $code:expr ), $e:ident | $($rest:tt)*) => (
        alt_custom!(__impl $i, $err!($code), call!($e) | $($rest)*);
    );

    (__impl $i:expr, $err:ident!( $code:expr ), $subrule:ident!( $($args:tt)*) | $($rest:tt)*) => (
        {
            let i_ = $i.clone();
            let res = complete!(i_, $subrule!($($args)*));
            match res {
                nom::IResult::Done(_,_)     => res,
                // argh
                res => {
                    if let nom::IResult::Error(e) = res {
                        if is_custom(&e) {
                            return nom::IResult::Error(e);
                        }
                    }
                    alt_custom!(__impl $i, $err!($code), $($rest)*)
                }
            }
        }
    );

    (__impl $i:expr, $err:ident!( $code:expr ), $subrule:ident!( $($args:tt)* ) => { $gen:expr } | $($rest:tt)*) => (
        {
            let i_ = $i.clone();
            match complete!(i_, $subrule!($($args)* )) {
                nom::IResult::Done(i,o)     => nom::IResult::Done(i,$gen(o)),
                // argh
                res => {
                    if let nom::IResult::Error(e) = res {
                        if is_custom(&e) {
                            return nom::IResult::Error(e);
                        }
                    }
                    alt_custom!(__impl $i, $err!($code), $($rest)*)
                }
            }
        }
    );

    (__impl $i:expr, $err:ident!( $code:expr ), $e:ident => { $gen:expr } | $($rest:tt)*) => (
        alt_custom!(__impl $i, $err!($code), call!($e) => { $gen } | $($rest)*);
    );

    (__impl $i:expr, $err:ident!( $code:expr ), __end) => (
        $err!($code, $i)
    );

    ($i:expr, $err:ident!( $code:expr ), $($rest:tt)*) => (
        {
            alt_custom!(__impl $i, $err!($code), $($rest)* | __end)
        }
    );
);

#[doc(hidden)]
#[macro_export]
macro_rules! fold_many0_custom (
    ($i:expr, $submac:ident!( $($args:tt)* ), $init:expr, $f:expr) => (
        {
            use nom::InputLength;
            let ret;
            let f         = $f;
            let mut res   = $init;
            let mut input = $i.clone();

            loop {
                if input.input_len() == 0 {
                    ret = nom::IResult::Done(input, res);
                    break;
                }

                match $submac!(input, $($args)*) {
                    nom::IResult::Error(e)                         => {
                        if is_custom(&e) {
                            ret = nom::IResult::Error(e);
                        } else {
                            ret = nom::IResult::Done(input, res);
                        }
                        break;
                    },
                    nom::IResult::Incomplete(i) => {
                        ret = nom::IResult::Incomplete(i);
                        break;
                    },
                    nom::IResult::Done(i, o)                          => {
                        // loop trip must always consume (otherwise infinite loops)
                        if i == input {
                            ret = nom::IResult::Error(
                                error_position!(nom::ErrorKind::Many0,input)
                            );
                            break;
                        }

                        res = f(res, &o);
                        input = i;
                    }
                }
            }

            ret
        }
    );
    ($i:expr, $f:expr, $init:expr, $fold_f:expr) => (
        fold_many0_custom!($i, call!($f), $init, $fold_f);
    );
);
