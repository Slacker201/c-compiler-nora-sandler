use slk_c_core::{
    lexer_core::lex_tokens::{LexToken, LexTokenKind},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::Constant,
    },
};
use slk_tokenstream::{TokenStream, TokenstreamSpan};

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
                    return Ok(Constant::from_lex_constant(c.clone(), tok.span()));
                } else {
                    unreachable!()
                }
            }
            Err(e) => {
                if let Some(c) = e {
                    return Err(ParserError::new(
                        c.span(),
                        ParserErrorKind::expected_got(&["Constant"], ts),
                    ));
                } else {
                    return Err(ParserError::new(
                        TokenstreamSpan::new(start, start).into(),
                        ParserErrorKind::expected_got(&["Constant"], ts),
                    ));
                }
            }
        }
    }
}
