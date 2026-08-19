use slk_c_core::{
    core::Span,
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
        let start = ts.mark();
        let func = Function::parse(ts)?;
        let span = Span::from_tokenstream_mark(start, ts.mark());

        Ok(Self::new(func, span))
    }
}
