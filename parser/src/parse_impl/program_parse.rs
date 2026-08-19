use slk_c_core::{
    lexer_core::lex_tokens::LexToken,
    parser_core::{
        parser_errors::ParserError,
        parser_ir::{Function, Program},
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

impl Parse for Program {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let func = Function::parse(ts)?;
        let span = func.span();

        Ok(Self::new(func, span))
    }
}
