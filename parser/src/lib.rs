use slk_c_core::{
    lexer_core::lex_tokens::LexToken,
    parser_core::{parser_errors::ParserError, parser_ir::Program},
};
use slk_tokenstream::TokenStream;

pub mod parse_impl;

#[cfg(test)]
mod tests;

pub trait Parse {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError>
    where
        Self: Sized;
}

pub struct Parser<'a> {
    tokens: TokenStream<'a, LexToken>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [LexToken]) -> Self {
        Self {
            tokens: TokenStream::new(tokens),
        }
    }

    pub fn parse(mut self) -> Result<Program, ParserError> {
        Program::parse(&mut self.tokens)
    }
}
