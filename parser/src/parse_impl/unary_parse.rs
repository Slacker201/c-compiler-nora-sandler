use slk_c_core::{
    core::Span,
    lexer_core::lex_tokens::{LexToken, LexTokenKind, Symbol},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::UnaryOp,
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

impl Parse for UnaryOp {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError> {
        let start = ts.mark();

        match ts.consume_if_else_err(|t| {
            matches!(
                t.kind(),
                LexTokenKind::Symbol(Symbol::Minus) | LexTokenKind::Symbol(Symbol::Tilda)
            )
        }) {
            Ok(tok) => {
                if let LexTokenKind::Symbol(s) = tok.kind() {
                    return Ok(Self::from_symbol(*s));
                } else {
                    unreachable!()
                }
            }
            Err(e) => {
                if let Some(t) = e {
                    return Err(ParserError::new(
                        t.span(),
                        ParserErrorKind::expected_got(&["-", "~"], ts),
                    ));
                } else {
                    return Err(ParserError::new(
                        Span::from_tokenstream_mark(start, start).into(),
                        ParserErrorKind::expected_got(&["-", "~"], ts),
                    ));
                }
            }
        }
    }
}
