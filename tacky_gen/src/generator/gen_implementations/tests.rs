use lexer::Lexer;
use parser::Parse;
use slk_c_core::{parser_core::parser_ir::{Expression, Function, Statement}, tacky_core::{Label, TackyInstruction, Value, VariableId}};
use slk_tokenstream::TokenStream;

use crate::generator::TackyGen;



#[test]
fn statement_correct_minimal_expr() {
    let expr: Vec<_> = "return 0;".chars().collect();
    let tokens = Lexer::new(& expr).lex_until_error().unwrap();
    let mut ts = TokenStream::new(&tokens);
    let expr = Statement::parse(&mut ts).unwrap();

    let desired_tacky = vec![
        TackyInstruction::LoadVariable(VariableId::Variable { id: 0 }, Value::I32(0)),
        TackyInstruction::CopyValue(VariableId::EAX, VariableId::Variable { id: 0 }),
        TackyInstruction::Ret,
    ];

    assert_eq!(desired_tacky, expr.generate_tacky());
}

#[test]
fn statement_correct_three_token_expr() {
    let expr: Vec<_> = "return ~-0;".chars().collect();
    let tokens = Lexer::new(& expr).lex_until_error().unwrap();
    let mut ts = TokenStream::new(&tokens);
    let expr = Statement::parse(&mut ts).unwrap();

    let desired_tacky = vec![
        TackyInstruction::LoadVariable(VariableId::Variable { id: 0 }, Value::I32(0)),
        TackyInstruction::Negate(VariableId::Variable { id: 1 }, VariableId::Variable { id: 0 }),
        TackyInstruction::BitwiseNegate(VariableId::Variable { id: 2 }, VariableId::Variable { id: 1 }),
        TackyInstruction::CopyValue(VariableId::EAX, VariableId::Variable { id: 2 }),
        TackyInstruction::Ret,
    ];

    assert_eq!(desired_tacky, expr.generate_tacky());
}

#[test]
fn statement_correct_three_token_nested_expr() {
    let expr: Vec<_> = "return ~(-(0));".chars().collect();
    let tokens = Lexer::new(& expr).lex_until_error().unwrap();
    let mut ts = TokenStream::new(&tokens);
    let expr = Statement::parse(&mut ts).unwrap();

    let desired_tacky = vec![
        TackyInstruction::LoadVariable(VariableId::Variable { id: 0 }, Value::I32(0)),
        TackyInstruction::Negate(VariableId::Variable { id: 1 }, VariableId::Variable { id: 0 }),
        TackyInstruction::BitwiseNegate(VariableId::Variable { id: 2 }, VariableId::Variable { id: 1 }),
        TackyInstruction::CopyValue(VariableId::EAX, VariableId::Variable { id: 2 }),
        TackyInstruction::Ret,
    ];

    assert_eq!(desired_tacky, expr.generate_tacky());
}

#[test]
fn function_correct_three_token_nested_expr() {
    let expr: Vec<_> = "int main(void) { return ~(-(0)); }".chars().collect();
    let tokens = Lexer::new(& expr).lex_until_error().unwrap();
    let mut ts = TokenStream::new(&tokens);
    let expr = Function::parse(&mut ts).unwrap();

    let desired_tacky = vec![
        TackyInstruction::Label(Label::new(0)),
        TackyInstruction::LoadVariable(VariableId::Variable { id: 0 }, Value::I32(0)),
        TackyInstruction::Negate(VariableId::Variable { id: 1 }, VariableId::Variable { id: 0 }),
        TackyInstruction::BitwiseNegate(VariableId::Variable { id: 2 }, VariableId::Variable { id: 1 }),
        TackyInstruction::CopyValue(VariableId::EAX, VariableId::Variable { id: 2 }),
        TackyInstruction::Ret,
    ];

    assert_eq!((Label::new(0), desired_tacky), expr.generate_tacky());
}