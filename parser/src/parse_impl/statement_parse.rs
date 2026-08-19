use slk_c_core::{
    core::Span,
    get_or_ret,
    lexer_core::lex_tokens::{KeyWord, LexToken, LexTokenKind, Symbol},
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
        let start = ts.mark();

        get_or_ret!(
            ts,
            start,
            LexTokenKind::KeyWord(KeyWord::Return),
            &["return"]
        );

        let expr = Expression::parse(ts).inspect_err(|_| {
            ts.reset(&start);
        })?;

        get_or_ret!(ts, start, LexTokenKind::Symbol(Symbol::SemiColon), &[";"]);

        let span = Span::from_tokenstream_mark(start, ts.mark());

        Ok(Self::new(expr, span))
    }
}
