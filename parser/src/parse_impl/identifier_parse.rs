use slk_c_core::{
    core::Span,
    lexer_core::lex_tokens::{LexToken, LexTokenKind},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::Identifier,
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

impl Parse for Identifier {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let start = ts.mark();

        match ts.consume_if_else_err(|token| matches!(token.kind(), LexTokenKind::Identifier(_))) {
            Ok(tok) => {
                if let LexTokenKind::Identifier(c) = tok.kind() {
                    return Ok(Identifier::new(c.ident().to_string(), tok.span()));
                } else {
                    unreachable!()
                }
            }
            Err(e) => {
                if let Some(c) = e {
                    return Err(ParserError::new(
                        c.span(),
                        ParserErrorKind::expected_got(&["Identifier"], ts),
                    ));
                } else {
                    return Err(ParserError::new(
                        Span::from_tokenstream_mark(start, start).into(),
                        ParserErrorKind::expected_got(&["Identifier"], ts),
                    ));
                }
            }
        }
    }
}
