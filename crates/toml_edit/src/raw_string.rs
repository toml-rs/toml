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

    /// Access the underlying string
    pub fn as_str(&self) -> &str {
        self.value.as_str()
    }

    /// Access the underlying span
    pub fn span(&self) -> Option<std::ops::Range<usize>> {
        self.span.clone()
    }

    pub(crate) fn despan(&mut self) {
        self.span = None;
    }

    pub(crate) fn encode(&self, buf: &mut dyn std::fmt::Write) -> std::fmt::Result {
        write!(buf, "{}", self.value)
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
