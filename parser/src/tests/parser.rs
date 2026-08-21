use lexer::Lexer;
use slk_c_core::{
    core::Span,
    parser_core::{
        parser_errors::ParserError,
        parser_ir::{
            Constant, Expression, ExpressionKind, Function, Identifier, Program, Statement,
        },
    },
};

use crate::Parser;

#[test]
fn test_parser() {
    let prog: Vec<_> = "int main(void) { return 0; }".chars().collect();
    let valid_tokens = Lexer::new(&prog).lex_until_error().unwrap();

    let parser = Parser::new(&valid_tokens);

    let desired: Result<Program, ParserError> = Ok(Program::new(
        Function::new(
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
        ),
        Span::new(0, 10),
    ));

    let got = parser.parse();

    assert_eq!(got, desired)
}
