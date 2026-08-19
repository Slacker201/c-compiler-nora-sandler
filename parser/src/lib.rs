use slk_c_core::{lexer_core::lex_tokens::LexToken, parser_core::parser_errors::ParserError};
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
