#![allow(clippy::type_complexity)]

use serde_spanned::Spanned;
#[cfg(not(feature = "unbounded"))]
use toml_parse::parser::RecursionGuard;
use toml_parse::parser::ValidateWhitespace;

use crate::alloc_prelude::*;
use crate::de::DeTable;
use crate::de::DeValue;

pub(crate) mod array;
#[cfg(feature = "debug")]
pub(crate) mod debug;
pub(crate) mod document;
pub(crate) mod inline_table;
pub(crate) mod key;
pub(crate) mod value;

pub(crate) fn parse_document<'i>(
    source: toml_parse::Source<'i>,
    errors: &mut dyn prelude::ErrorSink,
) -> Spanned<DeTable<'i>> {
    let tokens = source.lex().into_vec();

    let mut events = Vec::with_capacity(tokens.len());
    let mut receiver = ValidateWhitespace::new(&mut events, source);
    #[cfg(not(feature = "unbounded"))]
    let mut receiver = RecursionGuard::new(&mut receiver, LIMIT);
    #[cfg(not(feature = "unbounded"))]
    let receiver = &mut receiver;
    #[cfg(feature = "unbounded")]
    let receiver = &mut receiver;
    toml_parse::parser::parse_document(&tokens, receiver, errors);

    let mut input = prelude::Input::new(&events);
    let doc = document::document(&mut input, source, errors);
    doc
}

pub(crate) fn parse_value<'i>(
    source: toml_parse::Source<'i>,
    errors: &mut dyn prelude::ErrorSink,
) -> Spanned<DeValue<'i>> {
    let tokens = source.lex().into_vec();

    let mut events = Vec::with_capacity(tokens.len());
    let mut receiver = ValidateWhitespace::new(&mut events, source);
    #[cfg(not(feature = "unbounded"))]
    let mut receiver = RecursionGuard::new(&mut receiver, LIMIT);
    #[cfg(not(feature = "unbounded"))]
    let receiver = &mut receiver;
    #[cfg(feature = "unbounded")]
    let receiver = &mut receiver;
    toml_parse::parser::parse_value(&tokens, receiver, errors);

    let mut input = prelude::Input::new(&events);
    let value = value::value(&mut input, source, errors);
    value
}

#[cfg(not(feature = "unbounded"))]
const LIMIT: u32 = 80;

pub(crate) mod prelude {
    pub(crate) use toml_parse::parser::EventKind;
    pub(crate) use toml_parse::ErrorSink;
    pub(crate) use toml_parse::ParseError;
    pub(crate) use winnow::stream::Stream as _;

    pub(crate) type Input<'i> = winnow::stream::TokenSlice<'i, toml_parse::parser::Event>;

    #[cfg(feature = "debug")]
    pub(crate) use super::debug::trace;
    #[cfg(feature = "debug")]
    pub(crate) use super::debug::TraceScope;
}
