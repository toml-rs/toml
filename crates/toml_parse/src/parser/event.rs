use crate::decode::Encoding;
use crate::ErrorSink;
use crate::Span;

pub trait EventReceiver {
    fn std_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn std_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn inline_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn inline_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
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
    fn inline_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::InlineTableOpen,
            encoding: None,
            span,
        });
    }
    fn inline_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::InlineTableClose,
            encoding: None,
            span,
        });
    }
    fn array_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        (self)(Event {
            kind: EventKind::ArrayOpen,
            encoding: None,
            span,
        });
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

#[cfg(feature = "std")]
impl EventReceiver for Vec<Event> {
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
    fn inline_table_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::InlineTableOpen,
            encoding: None,
            span,
        });
    }
    fn inline_table_close(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::InlineTableClose,
            encoding: None,
            span,
        });
    }
    fn array_open(&mut self, span: Span, _error: &mut dyn ErrorSink) {
        self.push(Event {
            kind: EventKind::ArrayOpen,
            encoding: None,
            span,
        });
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

impl EventReceiver for () {
    fn std_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn std_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn inline_table_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn inline_table_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_open(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn array_close(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn simple_key(&mut self, _span: Span, _encoding: Option<Encoding>, _error: &mut dyn ErrorSink) {
    }
    fn key_sep(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn key_val_sep(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn scalar(&mut self, _span: Span, _encoding: Option<Encoding>, _error: &mut dyn ErrorSink) {}
    fn value_sep(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn whitespace(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn comment(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn newline(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
    fn error(&mut self, _span: Span, _error: &mut dyn ErrorSink) {}
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
            EventKind::StdTableOpen => "std-table open",
            EventKind::StdTableClose => "std-table close",
            EventKind::ArrayTableOpen => "array-table open",
            EventKind::ArrayTableClose => "array-table close",
            EventKind::InlineTableOpen => "inline-table open",
            EventKind::InlineTableClose => "inline-table close",
            EventKind::ArrayOpen => "array open",
            EventKind::ArrayClose => "array close",
            EventKind::SimpleKey => "key",
            EventKind::KeySep => "key separator",
            EventKind::KeyValSep => "key-value separator",
            EventKind::Scalar => "value",
            EventKind::ValueSep => "value separator",
            EventKind::Whitespace => "whitespace",
            EventKind::Comment => "comment",
            EventKind::Newline => "newline",
            EventKind::Error => "error",
        }
    }
}
