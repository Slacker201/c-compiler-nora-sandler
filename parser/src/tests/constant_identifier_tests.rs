use lexer::Lexer;
use slk_c_core::{
    core::Span,
    lexer_core::lex_tokens::{self, LexTokenKind},
    parser_core::{
        parser_errors::{ParserError, ParserErrorKind},
        parser_ir::{Constant, Identifier},
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

#[test]
fn constant_accept_valid() {
    let prog: Vec<_> = "0".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Constant::parse(&mut ts),
        Ok(Constant::I32 {
            value: 0,
            span: Span::new(0, 1)
        })
    )
}

#[test]
fn constant_accept_valid_trailing_tokens() {
    let prog: Vec<_> = "0 test_tok".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Constant::parse(&mut ts),
        Ok(Constant::I32 {
            value: 0,
            span: Span::new(0, 1)
        })
    )
}

#[test]
fn constant_reject_invalid() {
    let prog: Vec<_> = "invalid_constant".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Constant::parse(&mut ts),
        Err(ParserError::new(
            Span::new(0, 16),
            ParserErrorKind::ExpectedFound {
                expected: &["Constant"],
                got: LexTokenKind::Identifier(lex_tokens::Identifier::new(
                    "invalid_constant".to_string()
                ))
            }
        ))
    )
}

#[test]
fn identifier_accept_valid() {
    let prog: Vec<_> = "ident".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Identifier::parse(&mut ts),
        Ok(Identifier::new("ident".to_string(), Span::new(0, 5)))
    )
}

#[test]
fn identifier_accept_valid_trailing_tokens() {
    let prog: Vec<_> = "ident 123".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Identifier::parse(&mut ts),
        Ok(Identifier::new("ident".to_string(), Span::new(0, 5)))
    )
}

#[test]
fn identifier_reject_invalid_token() {
    let prog: Vec<_> = "0".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    assert_eq!(
        Identifier::parse(&mut ts),
        Err(ParserError::new(
            Span::new(0, 1),
            ParserErrorKind::ExpectedFound {
                expected: &["Identifier"],
                got: LexTokenKind::Constant(lex_tokens::Constant::I32(0))
            }
        ))
    )
}
