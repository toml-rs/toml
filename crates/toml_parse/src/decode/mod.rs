#[cfg(feature = "alloc")]
use alloc::borrow::Cow;
#[cfg(feature = "alloc")]
use alloc::string::String;

pub(crate) mod string;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum Encoding {
    LiteralString = crate::lexer::APOSTROPHE,
    BasicString = crate::lexer::QUOTATION_MARK,
    MlLiteralString = 1,
    MlBasicString,
}

impl Encoding {
    pub const fn description(&self) -> &'static str {
        match self {
            Encoding::LiteralString => crate::lexer::TokenKind::LiteralString.description(),
            Encoding::BasicString => crate::lexer::TokenKind::BasicString.description(),
            Encoding::MlLiteralString => crate::lexer::TokenKind::MlLiteralString.description(),
            Encoding::MlBasicString => crate::lexer::TokenKind::MlBasicString.description(),
        }
    }
}

pub trait StringBuilder<'s> {
    fn clear(&mut self);
    #[must_use]
    fn push_str(&mut self, append: &'s str) -> bool;
    #[must_use]
    fn push_char(&mut self, append: char) -> bool;
}

impl<'s> StringBuilder<'s> for () {
    fn clear(&mut self) {}
    fn push_str(&mut self, _append: &'s str) -> bool {
        true
    }
    fn push_char(&mut self, _append: char) -> bool {
        true
    }
}

impl<'s> StringBuilder<'s> for &'s str {
    fn clear(&mut self) {
        *self = &self[0..0];
    }
    fn push_str(&mut self, append: &'s str) -> bool {
        if self.is_empty() {
            *self = append;
            true
        } else {
            false
        }
    }
    fn push_char(&mut self, _append: char) -> bool {
        false
    }
}

#[cfg(feature = "alloc")]
impl<'s> StringBuilder<'s> for Cow<'s, str> {
    fn clear(&mut self) {
        match self {
            Cow::Borrowed(s) => {
                s.clear();
            }
            Cow::Owned(s) => s.clear(),
        }
    }
    fn push_str(&mut self, append: &'s str) -> bool {
        match self {
            Cow::Borrowed(s) => {
                if !s.push_str(append) {
                    self.to_mut().push_str(append);
                }
            }
            Cow::Owned(s) => s.push_str(append),
        }
        true
    }
    fn push_char(&mut self, append: char) -> bool {
        self.to_mut().push(append);
        true
    }
}

#[cfg(feature = "alloc")]
impl<'s> StringBuilder<'s> for String {
    fn clear(&mut self) {
        self.clear();
    }
    fn push_str(&mut self, append: &'s str) -> bool {
        self.push_str(append);
        true
    }
    fn push_char(&mut self, append: char) -> bool {
        self.push(append);
        true
    }
}
