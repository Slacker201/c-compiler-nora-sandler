use lexer::Lexer;
use slk_c_core::{
    core::Span,
    lexer_core::lex_tokens::{Identifier, LexTokenKind},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::{Constant, Expression, ExpressionKind, UnaryOp},
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

#[test]
fn expression_accept_constant() {
    let prog: Vec<_> = "0".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Ok(Expression::new(
            ExpressionKind::Constant(Constant::I32 {
                value: 0,
                span: Span::new(0, 1)
            }),
            Span::new(0, 1),
        )),
        Expression::parse(&mut ts)
    )
}

#[test]
fn expression_accept_unary() {
    let prog: Vec<_> = "-0".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Ok(Expression::new(
            ExpressionKind::Unary(
                UnaryOp::Negate,
                Box::new(Expression::new(
                    ExpressionKind::Constant(Constant::I32 {
                        value: 0,
                        span: Span::new(1, 2)
                    }),
                    Span::new(1, 2),
                ))
            ),
            Span::new(0, 2),
        )),
        Expression::parse(&mut ts)
    )
}

#[test]
fn expression_accept_parenthesis() {
    let prog: Vec<_> = "(0)".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Ok(Expression::new(
            ExpressionKind::Constant(Constant::I32 {
                value: 0,
                span: Span::new(1, 2)
            }),
            Span::new(0, 3),
        )),
        Expression::parse(&mut ts)
    )
}

#[test]
fn expression_accept_nested_unary() {
    let prog: Vec<_> = "-(0)".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Ok(Expression::new(
            ExpressionKind::Unary(
                UnaryOp::Negate,
                Box::new(Expression::new(
                    ExpressionKind::Constant(Constant::I32 {
                        value: 0,
                        span: Span::new(2, 3)
                    }),
                    Span::new(1, 4),
                ))
            ),
            Span::new(0, 4),
        )),
        Expression::parse(&mut ts)
    )
}

#[test]
fn expression_accept_constant_trailing_tokens() {
    let prog: Vec<_> = "0 trailing_token".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Ok(Expression::new(
            ExpressionKind::Constant(Constant::I32 {
                value: 0,
                span: Span::new(0, 1)
            }),
            Span::new(0, 1),
        )),
        Expression::parse(&mut ts)
    )
}

#[test]
fn expression_reject_invalid_token() {
    let prog: Vec<_> = "invalid_tok".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Err(ParserError::new(
            Span::new(0, 0),
            ParserErrorKind::ExpectedFound {
                expected: &["-", "~", "(", "Constant"],
                got: LexTokenKind::Identifier(Identifier::new("invalid_tok".to_string()),)
            }
        )),
        Expression::parse(&mut ts)
    )
}

#[test]
fn expression_reject_unclosed_paren() {
    let prog: Vec<_> = "(0".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Err(ParserError::new(
            Span::new(2, 2),
            ParserErrorKind::ExpectedFound {
                expected: &[")"],
                got: LexTokenKind::EOF
            }
        )),
        Expression::parse(&mut ts)
    )
}
