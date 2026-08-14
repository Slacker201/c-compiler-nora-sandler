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
}