use std::sync::atomic::{AtomicU64, Ordering};


#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TackyInstruction {
    Ret,
    LoadVariable(VariableId, Value),
}

static VARIABLE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VariableId {
    EAX,
    Variable {
        id: u64,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Value {
    I32(i32),
}

impl VariableId {
    pub fn new_id() -> Self {
        Self::Variable { id: VARIABLE_COUNTER.fetch_add(1, Ordering::Relaxed) }
    }
}