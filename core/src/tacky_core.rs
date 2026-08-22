use std::sync::atomic::{AtomicU64, Ordering};

use crate::parser_core::parser_ir::Constant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TackyInstruction {
    Label(Label),
    Ret,
    LoadVariable(VariableId, Value),
    CopyValue(VariableId, VariableId),
    BitwiseNegate(VariableId, VariableId),
    Negate(VariableId, VariableId),
}

static VARIABLE_COUNTER: AtomicU64 = AtomicU64::new(0);
static FUNCTION_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VariableId {
    EAX,
    Variable { id: u64 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    I32(i32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Label {
    id: u64,
}

impl VariableId {
    pub fn new_id() -> Self {
        Self::Variable {
            id: VARIABLE_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }
}

impl Label {
    pub fn new_id() -> Self {
        Self { id: FUNCTION_COUNTER.fetch_add(1, Ordering::Relaxed) }
    }
}


impl Value {
    pub fn from_ast_constant(constant: &Constant) -> Self {
        match constant {
            Constant::I32 { value, span: _ } => Self::I32(*value),
        }
    }
}
