use crate::core::Span;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LexToken {
    span: Span,
    kind: LexTokenKind,
}

impl LexToken {
    pub fn new(span: Span, kind: LexTokenKind) -> Self {
        Self { span, kind }
    }
    pub fn kind(&self) -> &LexTokenKind {
        &self.kind
    }
    pub fn span(&self) -> Span {
        self.span
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LexTokenKind {
    KeyWord(KeyWord),
    Symbol(Symbol),
    Constant(Constant),
    Identifier(Identifier),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum KeyWord {
    Int,
    Void,
    Return,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Symbol {
    OpenParen,
    CloseParen,
    OpenBracket,
    CloseBracket,
    SemiColon,
    Minus,
    Tilda,
    Decrement,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Constant {
    I32(i32),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    ident: String,
}

impl Identifier {
    pub fn new(ident: String) -> Self {
        Self { ident }
    }
}
