//! Lexed TOML tokens

use super::Span;
use super::APOSTROPHE;
use super::COMMENT_START_SYMBOL;
use super::QUOTATION_MARK;
use super::WSCHAR;
use crate::decode::Encoding;

/// An unvalidated TOML Token
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Token {
    pub(super) kind: TokenKind,
    pub(super) span: Span,
}

impl Token {
    pub(super) fn new(kind: TokenKind, span: Span) -> Self {
        Self { kind, span }
    }

    #[inline(always)]
    pub fn kind(&self) -> TokenKind {
        self.kind
    }

    #[inline(always)]
    pub fn span(&self) -> Span {
        self.span
    }

    pub fn to_error(self, expected: &'static [crate::Expected]) -> crate::ParseError {
        crate::ParseError {
            context: self.span(),
            description: self.kind().description(),
            expected,
            unexpected: self.span(),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u8)]
pub enum TokenKind {
    /// Either for dotted-key or float
    Dot = b'.',
    /// Key-value separator
    Equals = b'=',
    /// Value separator
    Comma = b',',
    /// Either array or standard-table start
    LeftSquareBracket = b'[',
    /// Either array or standard-table end
    RightSquareBracket = b']',
    /// Inline table start
    LeftCurlyBracket = b'{',
    /// Inline table end
    RightCurlyBracket = b'}',
    Whitespace = WSCHAR.0,
    Comment = COMMENT_START_SYMBOL,
    Newline = b'\n',
    LiteralString = APOSTROPHE,
    BasicString = QUOTATION_MARK,
    MlLiteralString = 1,
    MlBasicString,

    /// Anything else
    Atom,

    Eof,
}

impl TokenKind {
    pub const fn description(&self) -> &'static str {
        match self {
            TokenKind::Dot => "`.`",
            TokenKind::Equals => "`=`",
            TokenKind::Comma => "`,`",
            TokenKind::LeftSquareBracket => "`[`",
            TokenKind::RightSquareBracket => "`]`",
            TokenKind::LeftCurlyBracket => "`{`",
            TokenKind::RightCurlyBracket => "`}`",
            TokenKind::Whitespace => "whitespace",
            TokenKind::Comment => "comment",
            TokenKind::Newline => "newline",
            TokenKind::LiteralString => "literal string",
            TokenKind::BasicString => "basic string",
            TokenKind::MlLiteralString => "multi-line literal string",
            TokenKind::MlBasicString => "multi-line basic string",
            TokenKind::Atom => "token",
            TokenKind::Eof => "end-of-input",
        }
    }

    pub fn encoding(&self) -> Option<Encoding> {
        match self {
            TokenKind::LiteralString => Some(Encoding::LiteralString),
            TokenKind::BasicString => Some(Encoding::BasicString),
            TokenKind::MlLiteralString => Some(Encoding::MlLiteralString),
            TokenKind::MlBasicString => Some(Encoding::MlBasicString),
            TokenKind::Atom
            | TokenKind::LeftSquareBracket
            | TokenKind::RightSquareBracket
            | TokenKind::Dot
            | TokenKind::Equals
            | TokenKind::Comma
            | TokenKind::RightCurlyBracket
            | TokenKind::LeftCurlyBracket
            | TokenKind::Whitespace
            | TokenKind::Newline
            | TokenKind::Comment
            | TokenKind::Eof => None,
        }
    }
}
