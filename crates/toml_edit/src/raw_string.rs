use crate::InternalString;

/// Opaque string storage for raw TOML; internal to `toml_edit`
#[derive(PartialEq, Eq, Clone, Debug, Default, Hash)]
pub struct RawString {
    value: InternalString,
    span: Option<std::ops::Range<usize>>,
}

impl RawString {
    pub(crate) fn new(s: impl Into<RawString>) -> Self {
        s.into()
    }

    pub(crate) fn with_span(mut self, span: std::ops::Range<usize>) -> Self {
        self.span = Some(span);
        self
    }

    pub(crate) fn as_str(&self) -> &str {
        self.value.as_str()
    }

    pub(crate) fn span(&self) -> Option<std::ops::Range<usize>> {
        self.span.clone()
    }

    pub(crate) fn despan(&mut self) {
        self.span = None;
    }
}

impl From<&str> for RawString {
    #[inline]
    fn from(s: &str) -> Self {
        Self {
            value: InternalString::from(s),
            span: None,
        }
    }
}

impl From<String> for RawString {
    #[inline]
    fn from(s: String) -> Self {
        Self {
            value: InternalString::from(s),
            span: None,
        }
    }
}

impl From<&String> for RawString {
    #[inline]
    fn from(s: &String) -> Self {
        Self {
            value: InternalString::from(s),
            span: None,
        }
    }
}

impl From<InternalString> for RawString {
    #[inline]
    fn from(s: InternalString) -> Self {
        Self {
            value: s,
            span: None,
        }
    }
}

impl From<&InternalString> for RawString {
    #[inline]
    fn from(s: &InternalString) -> Self {
        Self {
            value: InternalString::from(s),
            span: None,
        }
    }
}

impl From<Box<str>> for RawString {
    #[inline]
    fn from(s: Box<str>) -> Self {
        Self {
            value: InternalString::from(s),
            span: None,
        }
    }
}
