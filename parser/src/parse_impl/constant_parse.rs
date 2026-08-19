use slk_c_core::{
    core::Span,
    lexer_core::lex_tokens::{LexToken, LexTokenKind},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::Constant,
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

impl Parse for Constant {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let start = ts.mark();

        match ts.consume_if_else_err(|token| matches!(token.kind(), LexTokenKind::Constant(_))) {
            Ok(tok) => {
                if let LexTokenKind::Constant(c) = tok.kind() {
                    return Ok(Constant::from_lex_constant(
                        c.clone(),
                        Span::from_tokenstream_mark(start, ts.mark()),
                    ));
                } else {
                    unreachable!()
                }
            }
            Err(e) => {
                if e.is_some() {
                    return Err(ParserError::new(
                        Span::from_tokenstream_mark(start, ts.mark()),
                        ParserErrorKind::expected_got(&["Constant"], ts),
                    ));
                } else {
                    return Err(ParserError::new(
                        Span::from_tokenstream_mark(start, start).into(),
                        ParserErrorKind::expected_got(&["Constant"], ts),
                    ));
                }
            }
        }
    }
}
