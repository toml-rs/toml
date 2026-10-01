use crate::ErrorSink;
use crate::ParseError;
use crate::Source;
use crate::Span;
use crate::decoder::Encoding;

pub trait EventReceiver {
    fn std_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn std_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    /// Returns if entering the inline table is allowed
    #[must_use]
    fn inline_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) -> bool {
        true
    }
    fn inline_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    /// Returns if entering the array is allowed
    #[must_use]
    fn array_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) -> bool {
        true
    }
    fn array_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn simple_key(&mut self, _span: Span, _kind: Option<Encoding>, _error: &mut dyn ErrorSink) {}
    fn key_sep(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn key_val_sep(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn scalar(&mut self, _span: Span, _kind: Option<Encoding>, _error: &mut dyn ErrorSink) {}
    fn value_sep(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn whitespace(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn comment(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn newline(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn error(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
}

impl<F> EventReceiver for F
where
    F: FnMut(Event),
{
    fn std_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::StdTableOpen,
            encoding: None,
            span,
        });
    }
    fn std_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::StdTableClose,
            encoding: None,
            span,
        });
    }
    fn array_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::ArrayTableOpen,
            encoding: None,
            span,
        });
    }
    fn array_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::ArrayTableClose,
            encoding: None,
            span,
        });
    }
    fn inline_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) -> bool {
        (self)(Event {
            kind: EventKind::InlineTableOpen,
            encoding: None,
            span,
        });
        true
    }
    fn inline_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::InlineTableClose,
            encoding: None,
            span,
        });
    }
    fn array_open(&mut self, span: Span, _error: &mut dyn ErrorSink) -> bool {
        (self)(Event {
            kind: EventKind::ArrayOpen,
            encoding: None,
            span,
        });
        true
    }
    fn array_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::ArrayClose,
            encoding: None,
            span,
        });
    }
    fn simple_key(&mut self, span: Span, encoding: Option<Encoding>, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::SimpleKey,
            encoding,
            span,
        });
    }
    fn key_sep(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::KeySep,
            encoding: None,
            span,
        });
    }
    fn key_val_sep(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::KeyValSep,
            encoding: None,
            span,
        });
    }
    fn scalar(&mut self, span: Span, encoding: Option<Encoding>, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::Scalar,
            encoding,
            span,
        });
    }
    fn value_sep(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::ValueSep,
            encoding: None,
            span,
        });
    }
    fn whitespace(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::Whitespace,
            encoding: None,
            span,
        });
    }
    fn comment(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::Comment,
            encoding: None,
            span,
        });
    }
    fn newline(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::Newline,
            encoding: None,
            span,
        });
    }
    fn error(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::Error,
            encoding: None,
            span,
        });
    }
}

#[cfg(feature = "alloc")]
#[allow(unused_qualifications)]
impl EventReceiver for alloc::vec::Vec<Event> {
    fn std_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::StdTableOpen,
            encoding: None,
            span,
        });
    }
    fn std_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::StdTableClose,
            encoding: None,
            span,
        });
    }
    fn array_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::ArrayTableOpen,
            encoding: None,
            span,
        });
    }
    fn array_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::ArrayTableClose,
            encoding: None,
            span,
        });
    }
    fn inline_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) -> bool {
        self.push(Event {
            kind: EventKind::InlineTableOpen,
            encoding: None,
            span,
        });
        true
    }
    fn inline_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::InlineTableClose,
            encoding: None,
            span,
        });
    }
    fn array_open(&mut self, span: Span, _error: &mut dyn ErrorSink) -> bool {
        self.push(Event {
            kind: EventKind::ArrayOpen,
            encoding: None,
            span,
        });
        true
    }
    fn array_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::ArrayClose,
            encoding: None,
            span,
        });
    }
    fn simple_key(&mut self, span: Span, encoding: Option<Encoding>, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::SimpleKey,
            encoding,
            span,
        });
    }
    fn key_sep(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::KeySep,
            encoding: None,
            span,
        });
    }
    fn key_val_sep(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::KeyValSep,
            encoding: None,
            span,
        });
    }
    fn scalar(&mut self, span: Span, encoding: Option<Encoding>, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::Scalar,
            encoding,
            span,
        });
    }
    fn value_sep(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::ValueSep,
            encoding: None,
            span,
        });
    }
    fn whitespace(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::Whitespace,
            encoding: None,
            span,
        });
    }
    fn comment(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::Comment,
            encoding: None,
            span,
        });
    }
    fn newline(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::Newline,
            encoding: None,
            span,
        });
    }
    fn error(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::Error,
            encoding: None,
            span,
        });
    }
}

impl EventReceiver for () {}

/// Centralize validation for all whitespace-like content
pub struct ValidateWhitespace<'r, 's> {
    receiver: &'r mut dyn EventReceiver,
    source: Source<'s>,
}

impl<'r, 's> ValidateWhitespace<'r, 's> {
    pub fn new(receiver: &'r mut dyn EventReceiver, source: Source<'s>) -> Self {
        Self { receiver, source }
    }
}

impl EventReceiver for ValidateWhitespace<'_, '_> {
    fn std_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.std_table_open(span, error);
    }
    fn std_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.std_table_close(span, error);
    }
    fn array_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_table_open(span, error);
    }
    fn array_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_table_close(span, error);
    }
    fn inline_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) -> bool {
        self.receiver.inline_table_open(span, error)
    }
    fn inline_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.inline_table_close(span, error);
    }
    fn array_open(&mut self, span: Span, error: &mut dyn ErrorSink) -> bool {
        self.receiver.array_open(span, error)
    }
    fn array_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_close(span, error);
    }
    fn simple_key(&mut self, span: Span, encoding: Option<Encoding>, error: &mut dyn ErrorSink) {
        self.receiver.simple_key(span, encoding, error);
    }
    fn key_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.key_sep(span, error);
    }
    fn key_val_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.key_val_sep(span, error);
    }
    fn scalar(&mut self, span: Span, encoding: Option<Encoding>, error: &mut dyn ErrorSink) {
        self.receiver.scalar(span, encoding, error);
    }
    fn value_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.value_sep(span, error);
    }
    fn whitespace(&mut self, span: Span, error: &mut dyn ErrorSink) {
        // `EventReceiver` is a safe trait and `ValidateWhitespace` is constructed
        // through a safe public API, so `span` cannot be trusted to be valid even
        // when the `unsafe` feature is enabled: always bounds-check it.
        let raw = self.source.get(span).expect("token spans are valid");
        raw.decode_whitespace(error);

        self.receiver.whitespace(span, error);
    }
    fn comment(&mut self, span: Span, error: &mut dyn ErrorSink) {
        // See the comment in `whitespace` above: this must stay on the checked path
        // regardless of the `unsafe` feature.
        let raw = self.source.get(span).expect("token spans are valid");
        raw.decode_comment(error);

        self.receiver.comment(span, error);
    }
    fn newline(&mut self, span: Span, error: &mut dyn ErrorSink) {
        // See the comment in `whitespace` above: this must stay on the checked path
        // regardless of the `unsafe` feature.
        let raw = self.source.get(span).expect("token spans are valid");
        raw.decode_newline(error);

        self.receiver.newline(span, error);
    }
    fn error(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.error(span, error);
    }
}

pub struct RecursionGuard<'r> {
    receiver: &'r mut dyn EventReceiver,
    max_depth: u32,
    depth: i64,
}

impl<'r> RecursionGuard<'r> {
    pub fn new(receiver: &'r mut dyn EventReceiver, max_depth: u32) -> Self {
        Self {
            receiver,
            max_depth,
            depth: 0,
        }
    }

    fn within_depth(&self) -> bool {
        self.depth <= self.max_depth as i64
    }
}

impl EventReceiver for RecursionGuard<'_> {
    fn std_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.std_table_open(span, error);
    }
    fn std_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.std_table_close(span, error);
    }
    fn array_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_table_open(span, error);
    }
    fn array_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.array_table_close(span, error);
    }
    fn inline_table_open(&mut self, span: Span, error: &mut dyn ErrorSink) -> bool {
        let allowed = self.receiver.inline_table_open(span, error);
        self.depth += 1;
        let within_depth = self.within_depth();
        if allowed && !within_depth {
            error.report_error(
                ParseError::new("cannot recurse further; max recursion depth met")
                    .with_unexpected(span),
            );
        }
        allowed && within_depth
    }
    fn inline_table_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.depth -= 1;
        self.receiver.inline_table_close(span, error);
    }
    fn array_open(&mut self, span: Span, error: &mut dyn ErrorSink) -> bool {
        let allowed = self.receiver.array_open(span, error);
        self.depth += 1;
        let within_depth = self.within_depth();
        if allowed && !within_depth {
            error.report_error(
                ParseError::new("cannot recurse further; max recursion depth met")
                    .with_unexpected(span),
            );
        }
        allowed && within_depth
    }
    fn array_close(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.depth -= 1;
        self.receiver.array_close(span, error);
    }
    fn simple_key(&mut self, span: Span, encoding: Option<Encoding>, error: &mut dyn ErrorSink) {
        self.receiver.simple_key(span, encoding, error);
    }
    fn key_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.key_sep(span, error);
    }
    fn key_val_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.key_val_sep(span, error);
    }
    fn scalar(&mut self, span: Span, encoding: Option<Encoding>, error: &mut dyn ErrorSink) {
        self.receiver.scalar(span, encoding, error);
    }
    fn value_sep(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.value_sep(span, error);
    }
    fn whitespace(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.whitespace(span, error);
    }
    fn comment(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.comment(span, error);
    }
    fn newline(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.newline(span, error);
    }
    fn error(&mut self, span: Span, error: &mut dyn ErrorSink) {
        self.receiver.error(span, error);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Event {
    kind: EventKind,
    encoding: Option<Encoding>,
    span: Span,
}

impl Event {
    pub fn new_unchecked(kind: EventKind, encoding: Option<Encoding>, span: Span) -> Self {
        Self {
            kind,
            encoding,
            span,
        }
    }

    #[inline(always)]
    pub fn kind(&self) -> EventKind {
        self.kind
    }

    #[inline(always)]
    pub fn encoding(&self) -> Option<Encoding> {
        self.encoding
    }

    #[inline(always)]
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum EventKind {
    StdTableOpen,
    StdTableClose,
    ArrayTableOpen,
    ArrayTableClose,
    InlineTableOpen,
    InlineTableClose,
    ArrayOpen,
    ArrayClose,
    SimpleKey,
    KeySep,
    KeyValSep,
    Scalar,
    ValueSep,
    Whitespace,
    Comment,
    Newline,
    Error,
}

impl EventKind {
    pub const fn description(&self) -> &'static str {
        match self {
            Self::StdTableOpen => "std-table open",
            Self::StdTableClose => "std-table close",
            Self::ArrayTableOpen => "array-table open",
            Self::ArrayTableClose => "array-table close",
            Self::InlineTableOpen => "inline-table open",
            Self::InlineTableClose => "inline-table close",
            Self::ArrayOpen => "array open",
            Self::ArrayClose => "array close",
            Self::SimpleKey => "key",
            Self::KeySep => "key separator",
            Self::KeyValSep => "key-value separator",
            Self::Scalar => "value",
            Self::ValueSep => "value separator",
            Self::Whitespace => "whitespace",
            Self::Comment => "comment",
            Self::Newline => "newline",
            Self::Error => "error",
        }
    }
}

#[cfg(test)]
#[allow(clippy::std_instead_of_core)] // `std::panic::catch_unwind` has no `core` equivalent
mod test {
    use super::*;
    use crate::Source;

    // Regression test for the soundness issue reported in toml-rs/toml#1175.
    //
    // `ValidateWhitespace` is a safe, public `EventReceiver` implementation. Its
    // `whitespace`/`comment`/`newline` methods must never rely on the caller having
    // passed a valid `Span`, even when the opt-in `unsafe` feature is enabled,
    // because 100% safe code can construct an out-of-bounds `Span` (e.g. via the
    // safe `Span::new_unchecked`) and hand it to these methods.
    //
    // Before the fix, with `--features unsafe`, this reaches
    // `self.source.get_unchecked(span)` on an out-of-range span, which is immediate
    // Undefined Behavior (confirmed under Miri: "pointer not dereferenceable").
    // After the fix, the same call always goes through the checked path and turns
    // into a controlled panic instead of memory unsafety.
    #[test]
    fn whitespace_rejects_out_of_bounds_span() {
        let source = Source::new("abc");
        let mut receiver = ();
        let mut validator = ValidateWhitespace::new(&mut receiver, source);
        let oob_span = Span::new_unchecked(0, 100);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            validator.whitespace(oob_span, &mut ());
        }));

        assert!(
            result.is_err(),
            "out-of-bounds span must be rejected via a controlled panic, not silently \
             accepted or turned into undefined behavior"
        );
    }

    #[test]
    fn comment_rejects_out_of_bounds_span() {
        let source = Source::new("abc");
        let mut receiver = ();
        let mut validator = ValidateWhitespace::new(&mut receiver, source);
        let oob_span = Span::new_unchecked(0, 100);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            validator.comment(oob_span, &mut ());
        }));

        assert!(
            result.is_err(),
            "out-of-bounds span must be rejected via a controlled panic, not silently \
             accepted or turned into undefined behavior"
        );
    }

    #[test]
    fn newline_rejects_out_of_bounds_span() {
        let source = Source::new("abc");
        let mut receiver = ();
        let mut validator = ValidateWhitespace::new(&mut receiver, source);
        let oob_span = Span::new_unchecked(0, 100);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            validator.newline(oob_span, &mut ());
        }));

        assert!(
            result.is_err(),
            "out-of-bounds span must be rejected via a controlled panic, not silently \
             accepted or turned into undefined behavior"
        );
    }

    // Sanity check: a span that legitimately spans the whole (short) source must
    // still be accepted and not treated as out-of-bounds.
    #[test]
    fn whitespace_accepts_in_bounds_span() {
        let source = Source::new("   ");
        let mut receiver = ();
        let mut validator = ValidateWhitespace::new(&mut receiver, source);
        let in_bounds_span = Span::new_unchecked(0, 3);

        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            validator.whitespace(in_bounds_span, &mut ());
        }));

        assert!(
            result.is_ok(),
            "a span within bounds of the source must be accepted"
        );
    }
}
