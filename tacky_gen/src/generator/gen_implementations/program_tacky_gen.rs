use slk_c_core::{parser_core::parser_ir::Program, tacky_core::TackyInstruction};

use crate::generator::TackyGen;



impl TackyGen<Vec<TackyInstruction>> for Program {
    fn generate_tacky(&self) -> Vec<TackyInstruction> {
        self.function().generate_tacky().1
    }
}