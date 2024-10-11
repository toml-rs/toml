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
