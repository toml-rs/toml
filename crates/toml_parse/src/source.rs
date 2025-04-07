use crate::lexer::Lexer;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Source<'i> {
    input: &'i str,
}

impl<'i> Source<'i> {
    pub fn new(input: &'i str) -> Self {
        Self { input }
    }

    pub fn lex(&self) -> Lexer<'i> {
        Lexer::new(self.input)
    }

    pub fn input(&self) -> &'i str {
        self.input
    }

    /// Return a subslice of the input
    pub fn get(&self, span: impl SourceIndex) -> Option<Raw<'i>> {
        span.get(self)
    }

    /// Return an unchecked subslice of the input
    ///
    /// ## Safety
    ///
    /// Callers of this function are responsible that these preconditions are satisfied:
    /// - The starting index must not exceed the ending index;
    /// - Indexes must be within bounds of the original slice;
    /// - Indexes must lie on UTF-8 sequence boundaries.
    ///
    /// Or one of:
    /// - `span` came from [`Source::lex`]
    ///
    /// Failing any of those, the returned string slice may reference invalid memory or violate the invariants communicated by `str` type.
    #[cfg(feature = "unsafe")]
    pub unsafe fn get_unchecked(&self, span: impl SourceIndex) -> Raw<'i> {
        // SAFETY: Same safety guarantees are required
        unsafe { span.get_unchecked(self) }
    }

    /// Return a subslice of the input
    fn get_raw_str(&self, span: Span) -> Option<&'i str> {
        let index = span.start()..span.end();
        self.input.get(index)
    }

    /// Return an unchecked subslice of the input
    ///
    /// ## Safety
    ///
    /// Callers of this function are responsible that these preconditions are satisfied:
    /// - The starting index must not exceed the ending index;
    /// - Indexes must be within bounds of the original slice;
    /// - Indexes must lie on UTF-8 sequence boundaries.
    ///
    /// Or one of:
    /// - `span` came from [`Source::lex`]
    ///
    /// Failing any of those, the returned string slice may reference invalid memory or violate the invariants communicated by `str` type.
    #[cfg(feature = "unsafe")]
    unsafe fn get_raw_str_unchecked(&self, span: Span) -> &'i str {
        let index = span.start()..span.end();
        // SAFETY: Same safety guarantees are required
        unsafe { self.input.get_unchecked(index) }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Raw<'i> {
    raw: &'i str,
}

impl<'i> Raw<'i> {
    pub fn new_unchecked(raw: &'i str) -> Self {
        Self { raw }
    }

    pub fn as_str(&self) -> &'i str {
        self.raw
    }
}

/// Location within the original document
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Span {
    start: usize,
    end: usize,
}

impl Span {
    pub fn new_unchecked(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn is_empty(&self) -> bool {
        self.end <= self.start
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }

    pub fn start(&self) -> usize {
        self.start
    }

    pub fn end(&self) -> usize {
        self.end
    }
}

impl core::fmt::Debug for Span {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        (self.start..self.end).fmt(f)
    }
}

/// A helper trait used for indexing operations on [`Source`]
pub trait SourceIndex: sealed::Sealed {
    /// Return a subslice of the input
    fn get<'i>(self, source: &Source<'i>) -> Option<Raw<'i>>;

    /// Return an unchecked subslice of the input
    ///
    /// ## Safety
    ///
    /// Callers of this function are responsible that these preconditions are satisfied:
    /// - The starting index must not exceed the ending index;
    /// - Indexes must be within bounds of the original slice;
    /// - Indexes must lie on UTF-8 sequence boundaries.
    ///
    /// Or one of:
    /// - `span` came from [`Source::lex`]
    ///
    /// Failing any of those, the returned string slice may reference invalid memory or violate the invariants communicated by `str` type.
    #[cfg(feature = "unsafe")]
    unsafe fn get_unchecked<'i>(self, source: &Source<'i>) -> Raw<'i>;
}

impl SourceIndex for Span {
    fn get<'i>(self, source: &Source<'i>) -> Option<Raw<'i>> {
        (&self).get(source)
    }

    #[cfg(feature = "unsafe")]
    unsafe fn get_unchecked<'i>(self, source: &Source<'i>) -> Raw<'i> {
        // SAFETY: Same safety guarantees are required
        unsafe { (&self).get_unchecked(source) }
    }
}

impl SourceIndex for &Span {
    fn get<'i>(self, source: &Source<'i>) -> Option<Raw<'i>> {
        source.get_raw_str(*self).map(Raw::new_unchecked)
    }

    #[cfg(feature = "unsafe")]
    unsafe fn get_unchecked<'i>(self, source: &Source<'i>) -> Raw<'i> {
        // SAFETY: Same safety guarantees are required
        let raw = unsafe { source.get_raw_str_unchecked(*self) };
        Raw::new_unchecked(raw)
    }
}

mod sealed {
    pub trait Sealed {}

    impl Sealed for crate::Span {}
    impl Sealed for &crate::Span {}
}
