#![allow(clippy::dbg_macro)] // clippy config failing
#![cfg(feature = "std")]

use snapbox::assert_data_eq;
use snapbox::prelude::*;

use toml_parse::parser::*;
use toml_parse::ParseError;
use toml_parse::Source;

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
            }
        }

        assert_data_eq!(self.to_debug(), expected);
        if !self.events.is_empty() {
            let spans = self.events.iter().map(|t| t.span()).collect::<Vec<_>>();
            assert_eq!(spans.first().unwrap().start(), 0);
            assert_eq!(spans.last().unwrap().end(), self.input.len());
            for i in 0..(spans.len() - 1) {
                let current = &spans[i];
                let next = &spans[i + 1];
                assert_eq!(current.end(), next.start());
            }
        }
        if self.events.iter().any(|e| e.kind() == EventKind::Error) {
            assert!(!self.errors.is_empty());
        }
    }
}
