use slk_tokenstream::TokenStream;

use crate::{core::Span, lexer_core::lex_tokens::{LexToken, LexTokenKind}};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ParserError {
    span: Span,
    kind: ParserErrorKind,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ParserErrorKind {
    ExpectedFound {
        expected: &'static [&'static str],
        got: LexTokenKind,
    }
}

impl ParserError {
    pub fn new(span: Span, kind: ParserErrorKind) -> Self {
        Self { span, kind }
    }
    pub fn span(&self) -> Span {
        self.span
    }
    pub fn kind(&self) -> &ParserErrorKind {
        &self.kind
    }
}

impl ParserErrorKind {
    pub fn expected_got(expected: &'static [&'static str], ts: &TokenStream<'_, LexToken>) -> Self {
        let got = ts.peek().map(|c| c.kind().clone()).unwrap_or(LexTokenKind::EOF);

        Self::ExpectedFound { expected, got }
    }

    pub fn expected_got_eof(expected: &'static [&'static str]) -> Self {
        Self::ExpectedFound { expected, got: LexTokenKind::EOF }
    }
}