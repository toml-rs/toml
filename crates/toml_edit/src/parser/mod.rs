#![allow(clippy::type_complexity)]

#[macro_use]
pub(crate) mod macros;

pub(crate) mod array;
pub(crate) mod datetime;
pub(crate) mod document;
pub(crate) mod errors;
pub(crate) mod inline_table;
pub(crate) mod key;
pub(crate) mod numbers;
pub(crate) mod state;
pub(crate) mod strings;
pub(crate) mod table;
pub(crate) mod trivia;
pub(crate) mod value;

pub use errors::TomlError;

mod prelude {
    pub(crate) use super::errors::Context;
    pub(crate) use super::errors::ParserError;
    pub(crate) use super::errors::ParserValue;
    pub(crate) use nom8::IResult;
    pub(crate) use nom8::Parser as _;

    #[cfg(test)]
    pub(crate) use nom8::FinishIResult as _;

    pub(crate) type Input<'b> = &'b [u8];

    pub(crate) fn ok_error<I, O, E>(res: IResult<I, O, E>) -> Result<Option<(I, O)>, nom8::Err<E>> {
        match res {
            Ok(ok) => Ok(Some(ok)),
            Err(nom8::Err::Error(_)) => Ok(None),
            Err(err) => Err(err),
        }
    }

    #[allow(dead_code)]
    pub(crate) fn trace<I: std::fmt::Debug, O: std::fmt::Debug, E: std::fmt::Debug>(
        context: impl std::fmt::Display,
        mut parser: impl nom8::Parser<I, O, E>,
    ) -> impl FnMut(I) -> IResult<I, O, E> {
        static DEPTH: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        move |input: I| {
            let depth = DEPTH.fetch_add(1, std::sync::atomic::Ordering::SeqCst) * 2;
            eprintln!("{:depth$}--> {} {:?}", "", context, input);
            match parser.parse(input) {
                Ok((i, o)) => {
                    DEPTH.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                    eprintln!("{:depth$}<-- {} {:?}", "", context, i);
                    Ok((i, o))
                }
                Err(err) => {
                    DEPTH.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                    eprintln!("{:depth$}<-- {} {:?}", "", context, err);
                    Err(err)
                }
            }
        }
    }
}
