use slk_c_core::lexer_core::lex_tokens::Symbol;
use slk_tokenstream::TokenStream;

const OPEN_PAREN: char = '(';
const CLOSE_PAREN: char = ')';
const OPEN_BRACKET: char = '{';
const CLOSE_BRACKET: char = '}';
const SEMICOLON: char = ';';
const TILDA: char = '~';
const MINUS: char = '-';

pub(crate) fn try_get_symbol(ts: &mut TokenStream<'_, char>) -> Option<Symbol> {
    let symbol = match *ts.peek()? {
        OPEN_PAREN => Symbol::OpenParen,
        CLOSE_PAREN => Symbol::CloseParen,
        OPEN_BRACKET => Symbol::OpenBracket,
        CLOSE_BRACKET => Symbol::CloseBracket,
        SEMICOLON => Symbol::SemiColon,
        TILDA => Symbol::Tilda,
        MINUS => {
            if let Some('-') = ts.peek_offset(1) {
                ts.skip();
                Symbol::Decrement
            } else {
                Symbol::Minus
            }
        }
        _ => return None,
    };
    ts.skip();

    Some(symbol)
}
