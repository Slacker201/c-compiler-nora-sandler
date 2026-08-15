use slk_c_core::lexer_core::lex_tokens::{Constant, Identifier, KeyWord, LexTokenKind, Symbol};

use crate::Lexer;



#[test]
fn chapter_1() {
    let prog: Vec<_> = "int main(void) { return 0; }".chars().collect();
    let mut lexer = Lexer::new(&prog);

    let r = lexer.lex_until_error().unwrap();
    let desired_kinds = vec![
        LexTokenKind::KeyWord(KeyWord::Int),
        LexTokenKind::Identifier(Identifier::new("main".to_string())),
        LexTokenKind::Symbol(Symbol::OpenParen),
        LexTokenKind::KeyWord(KeyWord::Void),
        LexTokenKind::Symbol(Symbol::CloseParen),
        LexTokenKind::Symbol(Symbol::OpenBracket),
        LexTokenKind::KeyWord(KeyWord::Return),
        LexTokenKind::Constant(Constant::I32(0)),
        LexTokenKind::Symbol(Symbol::SemiColon),
        LexTokenKind::Symbol(Symbol::CloseBracket),
    ];


    for item in r.iter().zip(desired_kinds.iter()) {
        assert_eq!(item.0.kind(), item.1);
    }
    assert_eq!(r.last().unwrap().span().end(), prog.len());
}


#[test]
fn chapter_2() {
    let prog: Vec<_> = "int main(void) { return -(~(-0)); }".chars().collect();
    let mut lexer = Lexer::new(&prog);

    let r = lexer.lex_until_error().unwrap();
    let desired_kinds = vec![
        LexTokenKind::KeyWord(KeyWord::Int),
        LexTokenKind::Identifier(Identifier::new("main".to_string())),
        LexTokenKind::Symbol(Symbol::OpenParen),
        LexTokenKind::KeyWord(KeyWord::Void),
        LexTokenKind::Symbol(Symbol::CloseParen),
        LexTokenKind::Symbol(Symbol::OpenBracket),
        LexTokenKind::KeyWord(KeyWord::Return),
        LexTokenKind::Symbol(Symbol::Minus),
        LexTokenKind::Symbol(Symbol::OpenParen),
        LexTokenKind::Symbol(Symbol::Tilda),
        LexTokenKind::Symbol(Symbol::OpenParen),
        LexTokenKind::Symbol(Symbol::Minus),
        LexTokenKind::Constant(Constant::I32(0)),
        LexTokenKind::Symbol(Symbol::CloseParen),
        LexTokenKind::Symbol(Symbol::CloseParen),
        LexTokenKind::Symbol(Symbol::SemiColon),
        LexTokenKind::Symbol(Symbol::CloseBracket),
    ];


    for item in r.iter().zip(desired_kinds.iter()) {
        assert_eq!(item.0.kind(), item.1);
    }
    assert_eq!(r.last().unwrap().span().end(), prog.len());
}


#[test]
fn test_test() {
    let prog: Vec<_> = "int main(void) { 0twelve return -(~(-0)); }".chars().collect();

    let thing = Lexer::new(&prog).lex_and_accumulate_errors();

    println!("{:?}", thing);
}