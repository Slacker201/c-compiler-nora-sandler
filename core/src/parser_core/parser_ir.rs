use crate::{core::Span, lexer_core::lex_tokens};


#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Program {
    function: Function,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Function {
    identifier: Identifier,
    statement: Statement,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Statement {
    expression: Expression,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Expression {
    kind: ExpressionKind,
    span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ExpressionKind {
    Constant(Constant),
    Unary(UnaryOp, Box<Expression>),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Constant {
    I32 {
        value: i32,
        span: Span,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    Negate,
    BitInvert,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Identifier {
    ident: String,
    span: Span,
}


impl Program {
    pub fn new(function: Function, span: Span) -> Self {
        Self { function, span }
    }

    pub fn function(&self) -> &Function {
        &self.function
    }
}

impl Function {
    pub fn new(identifier: Identifier, statement: Statement, span: Span) -> Self {
        Self { identifier, statement, span }
    }

    pub fn identifier(&self) -> &Identifier {
        &self.identifier
    }

    pub fn statement(&self) -> &Statement {
        &self.statement
    }

        pub fn span(&self) -> Span {
        self.span
    }
}

impl Identifier {
    pub fn new(ident: String, span: Span,) -> Self {
        Self { ident, span }
    }

    pub fn value(&self) -> &str {
        &self.ident
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl Statement {
    pub fn new(expression: Expression, span: Span) -> Self {
        Self { expression, span }
    }

    pub fn expression(&self) -> &Expression {
        &self.expression
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl Constant {
    pub fn from_lex_constant(c: lex_tokens::Constant, span: Span) -> Self {
        match c {
            lex_tokens::Constant::I32(value) => Self::I32 { value, span },
        }
    }
}

impl Expression {
    pub fn new(kind: ExpressionKind, span: Span) -> Self {
        Self { kind, span }
    }

    pub fn span(&self) -> Span {
        self.span
    }

    pub fn set_span(&mut self, span: Span) {
        self.span = span;
    }
}

impl UnaryOp {
    pub fn from_symbol(s: lex_tokens::Symbol) -> Self {
        match s {
            lex_tokens::Symbol::Minus => Self::Negate,
            lex_tokens::Symbol::Tilda => Self::BitInvert,
            _ => panic!()
        }
    }

    pub fn try_from_symbol(s: lex_tokens::Symbol) -> Result<Self, ()> {
        let s = match s {
            lex_tokens::Symbol::Minus => Self::Negate,
            lex_tokens::Symbol::Tilda => Self::BitInvert,
            _ => return Err(())
        };

        Ok(s)
    }
}