use slk_c_core::{
    parser_core::parser_ir::Statement,
    tacky_gen::{TackyInstruction, VariableId},
};

use crate::generator::TackyGen;

impl TackyGen<Vec<TackyInstruction>> for Statement {
    fn generate_tacky(&self) -> Vec<TackyInstruction> {
        let (final_value, mut expression_tacky) = self.expression().generate_tacky();
        expression_tacky.push(TackyInstruction::CopyValue(VariableId::EAX, final_value));
        expression_tacky.push(TackyInstruction::Ret);

        expression_tacky
    }
}
