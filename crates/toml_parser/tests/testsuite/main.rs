#![allow(clippy::dbg_macro)] // clippy config failing
#![cfg(feature = "std")]

use snapbox::assert_data_eq;
use snapbox::prelude::*;

use toml_parser::ErrorSink as _;
use toml_parser::ParseError;
use toml_parser::Source;
use toml_parser::Span;
use toml_parser::decoder::ScalarKind;
use toml_parser::parser::*;

mod parse_document;
mod parse_simple_key;
mod parse_value;

#[derive(Debug)]
pub struct EventResults<'i> {
    pub input: &'i str,
    pub events: Vec<Event>,
    pub errors: Vec<ParseError>,
}

impl<'i> EventResults<'i> {
    pub fn new(input: &'i str) -> Self {
        Self {
            input,
            events: Vec::new(),
            errors: Vec::new(),
        }
    }

    #[track_caller]
    pub fn validate(&mut self, expected: impl IntoData) {
        let doc = Source::new(self.input);
        for event in &self.events {
            if event.kind() == EventKind::SimpleKey {
                let raw = doc.get(event).unwrap();
                raw.decode_key(&mut (), &mut self.errors);
            } else if event.kind() == EventKind::Comment {
                let raw = doc.get(event).unwrap();
                raw.decode_comment(&mut self.errors);
            } else if event.kind() == EventKind::Newline {
                let raw = doc.get(event).unwrap();
                raw.decode_newline(&mut self.errors);
            } else if event.kind() == EventKind::Scalar {
                let raw = doc.get(event).unwrap();
                let mut value = String::new();
                let kind = raw.decode_scalar(&mut value, &mut self.errors);
                dbg!(&value);
                match kind {
                    ScalarKind::String => {}
                    ScalarKind::Boolean(v) => {
                        let value = value.parse::<bool>();
                        if value.is_err() {
                            self.errors.report_error(
                                ParseError::new("failed to parse bool")
                                    .with_context(Span::new_unchecked(0, raw.len()))
                                    .with_unexpected(Span::new_unchecked(0, 2)),
                            );
                        } else if value != Ok(v) {
                            self.errors.report_error(
                                ParseError::new("mismatched bool value")
                                    .with_context(Span::new_unchecked(0, raw.len()))
                                    .with_unexpected(Span::new_unchecked(0, 2)),
                            );
                        }
                    }
                    ScalarKind::DateTime => {
                        let value = value.parse::<toml_datetime::Datetime>();
                        if value.is_err() {
                            self.errors.report_error(
                                ParseError::new("failed to parse datetime")
                                    .with_context(Span::new_unchecked(0, raw.len()))
                                    .with_unexpected(Span::new_unchecked(0, 2)),
                            );
                        }
                    }
                    ScalarKind::Float => {
                        let value = value.parse::<f64>();
                        if value.is_err() {
                            self.errors.report_error(
                                ParseError::new("failed to parse f64")
                                    .with_context(Span::new_unchecked(0, raw.len()))
                                    .with_unexpected(Span::new_unchecked(0, 2)),
                            );
                        }
                    }
                    ScalarKind::Integer(radix) => {
                        let value = i64::from_str_radix(&value, radix.value());
                        if value.is_err() {
                            self.errors.report_error(
                                ParseError::new("failed to parse i64")
                                    .with_context(Span::new_unchecked(0, raw.len()))
                                    .with_unexpected(Span::new_unchecked(0, 2)),
                            );
                        }
                    }
                }
            }
        }

        assert_data_eq!(self.to_debug(), expected);
        if !self.events.is_empty() {
            let spans = self.events.iter().map(|t| t.span()).collect::<Vec<_>>();
            if !self.input.as_bytes().starts_with(BOM) {
                assert_eq!(
                    spans.first().unwrap().start(),
                    0,
                    "first span needs to start at 0"
                );
            }
            assert_eq!(
                spans.last().unwrap().end(),
                self.input.len(),
                "last span needs to be at the end"
            );
            for i in 0..(spans.len() - 1) {
                let current = &spans[i];
                let next = &spans[i + 1];
                assert_eq!(
                    current.end(),
                    next.start(),
                    "events must not have gaps in spans"
                );
            }
        }
        if self.events.iter().any(|e| e.kind() == EventKind::Error) {
            assert!(!self.errors.is_empty());
        }
    }
}

const BOM: &[u8] = b"\xEF\xBB\xBF";
