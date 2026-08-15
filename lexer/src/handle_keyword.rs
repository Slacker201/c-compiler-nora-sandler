use slk_c_core::lexer_core::lex_tokens::KeyWord;

const INT: &str = "int";
const VOID: &str = "void";
const RETURN: &str = "return";


pub(crate) fn try_get_keyword(ident: &str) -> Option<KeyWord> {
    let k = match ident {
        INT => KeyWord::Int,
        VOID => KeyWord::Void,
        RETURN => KeyWord::Return,
        _ => return None,
    };


    Some(k)
}