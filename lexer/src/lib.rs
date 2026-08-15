use slk_c_core::{
    core::Span, lexer_core::{lex_errors::{LexError, LexErrorKind}, lex_tokens::{Constant, Identifier, LexToken, LexTokenKind}},
};
use slk_tokenstream::{TokenStream, TokenstreamSpan};

use crate::{handle_comment::handle_comment, handle_keyword::try_get_keyword, handle_symbol::try_get_symbol};

mod handle_symbol;
mod handle_keyword;
mod handle_comment;

#[cfg(test)]
mod lexer_tests;

pub struct Lexer<'a> {
    ts: TokenStream<'a, char>,
}

impl<'a> Lexer<'a> {
    pub fn new(backing_vec: &'a [char]) -> Self {
        Self {
            ts: TokenStream::new(backing_vec),
        }
    }

    pub fn next(&mut self) -> Result<Option<LexToken>, LexError> {
        loop {
            self.ts.skip_while(|c| c.is_whitespace());

            let start = self.ts.mark();

            if self.ts.is_eof() {
                return Ok(None);
            }

            if let Some(s) = try_get_symbol(&mut self.ts) {
                return Ok(Some(LexToken::new(
                    Span::new(start.position(), self.ts.cursor()),
                    LexTokenKind::Symbol(s),
                )));
            }

            if self.ts.peek().and_then(|c| Some(c.is_alphabetic() || *c == '_')).unwrap_or(false) {
                self.ts.skip_while(|c| c.is_alphanumeric() || *c == '_');
                let ident_span = self.ts.span_from_marks(start, self.ts.mark());
                let ident: String = self.ts.slice_from_span(&ident_span).iter().collect();
                let span: Span = ident_span.into();


                if let Some(k) = try_get_keyword(&ident) {
                    return Ok(Some(LexToken::new(span, LexTokenKind::KeyWord(k))));
                }
                
                return Ok(Some(
                    LexToken::new(
                        span, 
                    LexTokenKind::Identifier(Identifier::new(ident)
                        )
                    )
                ));
            }

            if self.ts.peek().and_then(|c| Some(c.is_numeric())).unwrap_or(false) {
                self.ts.skip_while(|c| !(c.is_whitespace() || c.is_ascii_punctuation()));
                let constant_span = self.ts.span_from_marks(start, self.ts.mark());
                let c: String = self.ts.slice_from_span(&constant_span).iter().collect();
                let span: Span = constant_span.into();

                if !c.chars().all(|c| c.is_numeric()) {
                    return Err(
                        LexError::new(
                            span, 
                            LexErrorKind::InvalidIdentifier,
                        )
                    );
                }

                let c = if let Ok(c) = i32::from_str_radix(&c, 10) {
                    c
                } else {
                    return Err(
                        LexError::new(
                            span, 
                            LexErrorKind::InvalidNumber,
                        )
                    );
                };

                return Ok(
                    Some(
                        LexToken::new(span, LexTokenKind::Constant(Constant::I32(c)))
                    )
                );

            }

            if !handle_comment(&mut self.ts) {
                self.ts.skip_while(|c| !(c.is_whitespace() || (c.is_ascii_punctuation() && *c != '_')));

                let error_end = self.ts.mark();

                self.ts.reset(&start);

                return Err(LexError::new(TokenstreamSpan::new(start, error_end).into(), LexErrorKind::InvalidIdentifier))
            }
        }
    }

    pub fn lex_until_error(&mut self) -> Result<Vec<LexToken>, (Vec<LexToken>, LexError)> {
        let mut tokens = vec![];
        loop {
            match self.next() {
                Ok(tok) => {
                    if let Some(t) = tok {
                        tokens.push(t);
                    } else {
                        return Ok(tokens);
                    }
                },
                Err(e) => {
                    return Err((tokens, e));
                },
            }
        }
    }

    pub fn lex_and_accumulate_errors(&mut self) -> Vec<Result<LexToken, LexError>> {
        let mut tokens = vec![];
        loop {
            match self.next() {
                Ok(tok) => {
                    if let Some(t) = tok {
                        tokens.push(Ok(t));
                    } else {
                        return tokens;
                    }
                },
                Err(e) => {
                    let len = e.span().len();
                    self.ts.advance(len);
                    tokens.push(Err(e));
                },
            }
        }
    }

}
