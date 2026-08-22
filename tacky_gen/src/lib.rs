use slk_c_core::{parser_core::parser_ir::Program, tacky_core::TackyInstruction};

use crate::generator::TackyGen;

mod generator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct TackyGenerator {
    program: Program,
}

impl TackyGenerator {
    pub fn new(program: Program) -> Self {
        Self { program }
    }

    pub fn generate_tacky(self) -> Vec<TackyInstruction> {
        self.program.generate_tacky()
    }
}
