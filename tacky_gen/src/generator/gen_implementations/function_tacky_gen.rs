use slk_c_core::{
    parser_core::parser_ir::Function,
    tacky_core::{Label, TackyInstruction},
};

use crate::generator::TackyGen;

impl TackyGen<(Label, Vec<TackyInstruction>)> for Function {
    fn generate_tacky(&self) -> (Label, Vec<TackyInstruction>) {
        let label = Label::new_id();

        let mut instr = vec![TackyInstruction::Label(label)];
        instr.extend(self.statement().generate_tacky());

        (label, instr)
    }
}
