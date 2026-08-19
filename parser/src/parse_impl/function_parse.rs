use slk_c_core::{
    core::Span,
    get_or_ret,
    lexer_core::lex_tokens::{KeyWord, LexToken, LexTokenKind, Symbol},
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

        get_or_ret!(ts, start, LexTokenKind::KeyWord(KeyWord::Int), &["int"]);

        let ident = Identifier::parse(ts).inspect_err(|_| {
            ts.reset(&start);
        })?;

        get_or_ret!(ts, start, LexTokenKind::Symbol(Symbol::OpenParen), &["("]);
        get_or_ret!(ts, start, LexTokenKind::KeyWord(KeyWord::Void), &["void"]);
        get_or_ret!(ts, start, LexTokenKind::Symbol(Symbol::CloseParen), &[")"]);
        get_or_ret!(ts, start, LexTokenKind::Symbol(Symbol::OpenBracket), &["{"]);

        let statement = Statement::parse(ts).inspect_err(|_| {
            ts.reset(&start);
        })?;

        get_or_ret!(
            ts,
            start,
            LexTokenKind::Symbol(Symbol::CloseBracket),
            &["}"]
        );

        let span = Span::from_tokenstream_mark(start, ts.mark());

        Ok(Self::new(ident, statement, span))
    }
}
