use crate::Span;

pub trait ErrorSink {
    fn report_error(&mut self, error: ParseError);
}

impl<F> ErrorSink for F
where
    F: FnMut(ParseError),
{
    fn report_error(&mut self, error: ParseError) {
        (self)(error);
    }
}

impl ErrorSink for () {
    fn report_error(&mut self, _error: ParseError) {}
}

impl ErrorSink for Option<ParseError> {
    fn report_error(&mut self, error: ParseError) {
        self.get_or_insert(error);
    }
}

#[cfg(feature = "std")]
impl ErrorSink for Vec<ParseError> {
    fn report_error(&mut self, error: ParseError) {
        self.push(error);
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub struct ParseError {
    pub context: Span,
    pub description: &'static str,
    pub expected: &'static [Expected],
    pub unexpected: Span,
}

impl ParseError {
    pub fn new(description: &'static str) -> Self {
        Self {
            context: Default::default(),
            description,
            expected: &[],
            unexpected: Default::default(),
        }
    }

    pub fn with_context(mut self, context: Span) -> Self {
        self.context = context;
        self
    }

    pub fn with_expected(mut self, expected: &'static [Expected]) -> Self {
        self.expected = expected;
        self
    }

    pub fn with_unexpected(mut self, unexpected: Span) -> Self {
        self.unexpected = unexpected;
        self
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[non_exhaustive]
pub enum Expected {
    Literal(&'static str),
    Description(&'static str),
}
