use lexer::Lexer;
use slk_c_core::{
    core::Span,
    lexer_core::lex_tokens::{Identifier, LexTokenKind},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::{Constant, Expression, ExpressionKind, Statement},
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

#[test]
fn statement_accept_valid() {
    let prog: Vec<_> = "return 0;".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    let statement = Statement::parse(&mut ts);

    assert_eq!(
        statement,
        Ok(Statement::new(
            Expression::new(
                ExpressionKind::Constant(Constant::I32 {
                    value: 0,
                    span: Span::new(1, 2)
                }),
                Span::new(1, 2)
            ),
            Span::new(0, 3)
        ))
    )
}

#[test]
fn statement_reject_invalid() {
    let prog: Vec<_> = "scoff".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    let statement = Statement::parse(&mut ts);

    assert_eq!(
        statement,
        Err(ParserError::new(
            Span::new(0, 0),
            ParserErrorKind::ExpectedFound {
                expected: &["return"],
                got: LexTokenKind::Identifier(Identifier::new("scoff".to_string()))
            }
        ))
    )
}

#[test]
fn statement_reject_invalid_const() {
    let prog: Vec<_> = "return const;".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    let statement = Statement::parse(&mut ts);

    assert_eq!(
        statement,
        Err(ParserError::new(
            Span::new(1, 1),
            ParserErrorKind::ExpectedFound {
                expected: &["-", "~", "(", "Constant"],
                got: LexTokenKind::Identifier(Identifier::new("const".to_string()))
            }
        ))
    )
}

#[test]
fn statement_reject_no_semicolon() {
    let prog: Vec<_> = "return 0".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    let statement = Statement::parse(&mut ts);

    assert_eq!(
        statement,
        Err(ParserError::new(
            Span::new(2, 2),
            ParserErrorKind::ExpectedFound {
                expected: &[";"],
                got: LexTokenKind::EOF
            }
        ))
    )
}

#[test]
fn statement_accept_valid_trailing() {
    let prog: Vec<_> = "return 0; trailing".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    let statement = Statement::parse(&mut ts);

    assert_eq!(
        statement,
        Ok(Statement::new(
            Expression::new(
                ExpressionKind::Constant(Constant::I32 {
                    value: 0,
                    span: Span::new(1, 2)
                }),
                Span::new(1, 2)
            ),
            Span::new(0, 3)
        ))
    )
}
