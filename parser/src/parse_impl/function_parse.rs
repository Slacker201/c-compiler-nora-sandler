use slk_c_core::{
    lexer_core::lex_tokens::LexToken,
    parser_core::{
        parser_errors::ParserError,
        parser_ir::{Function, Identifier, Statement},
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

impl Parse for Function {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let start = ts.mark();

        let ident = Identifier::parse(ts)?;

        let statement = Statement::parse(ts).inspect_err(|_| {
            ts.reset(&start);
        })?;

        let span = ident.span().combine(statement.span());

        Ok(Self::new(ident, statement, span))
    }
}
