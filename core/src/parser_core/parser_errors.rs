use slk_tokenstream::{Mark, TokenStream};

use crate::{
    core::Span,
    lexer_core::lex_tokens::{LexToken, LexTokenKind},
};

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
    },
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
        let got = ts
            .peek()
            .map(|c| c.kind().clone())
            .unwrap_or(LexTokenKind::EOF);

        Self::ExpectedFound { expected, got }
    }

    pub fn expected_got_eof(expected: &'static [&'static str]) -> Self {
        Self::ExpectedFound {
            expected,
            got: LexTokenKind::EOF,
        }
    }

    pub fn expected_got_from_opt(
        expected: &'static [&'static str],
        got: &Option<&LexToken>,
        start: Mark,
        end: Mark,
    ) -> (Self, Span) {
        let got = if let Some(s) = got {
            s.kind().clone()
        } else {
            LexTokenKind::EOF
        };

        (
            Self::ExpectedFound { expected, got },
            Span::from_tokenstream_mark(start, end),
        )
    }
}

impl ParserError {
    pub fn replace_desired(&mut self, new_expected: &'static [&'static str]) {
        match &mut self.kind {
            ParserErrorKind::ExpectedFound { expected, got: _ } => *expected = new_expected,
        }
    }
}
