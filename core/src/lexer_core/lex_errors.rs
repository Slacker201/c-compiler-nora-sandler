use crate::core::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LexError {
    span: Span,
    kind: LexErrorKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LexErrorKind {
    InvalidIdentifier,
    NonTerminatedBlockComment,
    InvalidNumber,
}

impl LexError {
    pub fn new(span: Span, kind: LexErrorKind) -> Self {
        Self { span, kind }
    }
    pub fn span(&self) -> Span {
        self.span
    }
    pub fn kind(&self) -> &LexErrorKind {
        &self.kind
    }
}
