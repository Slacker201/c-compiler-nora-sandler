use slk_c_core::{
    lexer_core::lex_tokens::LexToken,
    parser_core::{
        parser_errors::ParserError,
        parser_ir::{Expression, Statement},
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

impl Parse for Statement {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let expr = Expression::parse(ts)?;

        let span = expr.span();

        Ok(Self::new(expr, span))
    }
}
