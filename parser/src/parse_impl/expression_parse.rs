use slk_c_core::{
    core::Span,
    get_or_ret,
    lexer_core::lex_tokens::{LexToken, LexTokenKind, Symbol},
    parser_core::{
        parser_errors::ParserError,
        parser_ir::{
            Constant, Expression,
            ExpressionKind::{self},
            UnaryOp,
        },
    },
    replace_desired,
};
use slk_tokenstream::TokenStream;

use crate::Parse;

const VALID_EXPRESSION_TOKENS: &[&str] = &["-", "~", "(", "Constant"];

impl Parse for Expression {
    fn parse(ts: &mut TokenStream<'_, LexToken>) -> Result<Self, ParserError>
    where
        Self: Sized,
    {
        let start = ts.mark();

        if let Ok(c) = Constant::parse(ts) {
            return Ok(Self::new(
                ExpressionKind::Constant(c),
                Span::from_tokenstream_mark(start, ts.mark()).into(),
            ));
        }

        if ts
            .consume_if(|c| matches!(c.kind(), LexTokenKind::Symbol(Symbol::OpenParen)))
            .is_some()
        {
            let mut expr = Expression::parse(ts).map_err(|mut e| {
                e.replace_desired(VALID_EXPRESSION_TOKENS);
                ts.reset(&start);
                e
            })?;

            get_or_ret!(ts, start, LexTokenKind::Symbol(Symbol::CloseParen), &[")"]);

            expr.set_span(Span::from_tokenstream_mark(start, ts.mark()));

            return Ok(expr);
        }

        let unary_op = replace_desired!(ts, start, UnaryOp::parse(ts), VALID_EXPRESSION_TOKENS)?;

        let expr = Expression::parse(ts).inspect_err(|_| {
            ts.reset(&start);
        })?;

        return Ok(Expression::new(
            ExpressionKind::Unary(unary_op, Box::new(expr)),
            Span::from_tokenstream_mark(start, ts.mark()),
        ));
    }
}
