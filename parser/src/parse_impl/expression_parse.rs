use slk_c_core::{
    core::Span,
    lexer_core::lex_tokens::{LexToken, LexTokenKind, Symbol},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::{
            Constant, Expression,
            ExpressionKind::{self},
            UnaryOp,
        },
    },
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
            .consume_if(|t| *t.kind() == LexTokenKind::Symbol(Symbol::OpenParen))
            .is_some()
        {
            let mut expr = Expression::parse(ts).map_err(|mut e| {
                e.replace_desired(VALID_EXPRESSION_TOKENS);
                ts.reset(&start);
                e
            })?;

            match ts.consume_if_else_err(|t| *t.kind() == LexTokenKind::Symbol(Symbol::CloseParen))
            {
                Ok(_) => {
                    let new_span = Span::from_tokenstream_mark(start, ts.mark());

                    expr.set_span(new_span);

                    return Ok(expr);
                }
                Err(t) => {
                    if let Some(t) = t {
                        let e = Err(ParserError::new(
                            t.span(),
                            ParserErrorKind::expected_got(&[")"], ts),
                        ));

                        ts.reset(&start);
                        return e;
                    } else {
                        let e = Err(ParserError::new(
                            Span::from_tokenstream_mark(ts.mark(), ts.mark()),
                            ParserErrorKind::expected_got(&[")"], ts),
                        ));
                        ts.reset(&start);
                        return e;
                    }
                }
            }
        }

        let op = UnaryOp::parse(ts).map_err(|mut e| {
            e.replace_desired(VALID_EXPRESSION_TOKENS);
            e
        })?;

        let expr = Expression::parse(ts).map_err(|mut e| {
            e.replace_desired(VALID_EXPRESSION_TOKENS);
            ts.reset(&start);
            e
        })?;

        let kind = ExpressionKind::Unary(op, Box::new(expr));

        let span = Span::from_tokenstream_mark(start, ts.mark());

        Ok(Expression::new(kind, span))
    }
}
