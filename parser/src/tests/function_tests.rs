use lexer::Lexer;
use slk_c_core::{
    core::Span, lexer_core::lex_tokens::{LexTokenKind, Symbol}, parser_core::{
        parser_errors::{ParserError, ParserErrorKind}, parser_ir::{Constant, Expression, ExpressionKind, Function, Identifier, Statement},
    },
};
use slk_tokenstream::TokenStream;

use crate::Parse;

#[test]
fn function_accept_valid() {
    let prog: Vec<_> = "int main(void) { return 0; }".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    let desired: Result<Function, ParserError> = Ok(Function::new(
        Identifier::new("main".to_string(), Span::new(1, 2)),
        Statement::new(
            Expression::new(
                ExpressionKind::Constant(Constant::I32 {
                    value: 0,
                    span: Span::new(7, 8),
                }),
                Span::new(7, 8),
            ),
            Span::new(6, 9),
        ),
        Span::new(0, 10),
    ));

    let got = Function::parse(&mut ts);

    assert_eq!(got, desired)
}

#[test]
fn expression_reject_malformed() {
    let prog: Vec<_> = "int main() { return 0; }".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let mut ts = TokenStream::new(&valid_tokens);

    let desired: Result<Function, ParserError> = Err(
        ParserError::new(Span::new(3, 3), 
        ParserErrorKind::ExpectedFound { expected: &["void"], got: LexTokenKind::Symbol(Symbol::CloseParen) }
    )
    );

    let got = Function::parse(&mut ts);

    assert_eq!(got, desired)
}
