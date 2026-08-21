use slk_c_core::{
    parser_core::parser_ir::{Expression, ExpressionKind, UnaryOp},
    tacky_gen::{TackyInstruction, Value, VariableId},
};

use crate::generator::TackyGen;

impl TackyGen<(VariableId, Vec<TackyInstruction>)> for Expression {
    fn generate_tacky(&self) -> (VariableId, Vec<TackyInstruction>) {
        match self.kind() {
            ExpressionKind::Constant(constant) => {
                let mut instructions = vec![];
                let variable_id = VariableId::new_id();
                instructions.push(TackyInstruction::LoadVariable(
                    variable_id,
                    Value::from_ast_constant(constant),
                ));

                return (variable_id, instructions);
            }
            ExpressionKind::Unary(unary_op, expression) => match unary_op {
                UnaryOp::Negate => {
                    let (final_location, mut instructions) = expression.generate_tacky();
                    let var_id = VariableId::new_id();
                    let instr = TackyInstruction::Negate(var_id, final_location);

                    instructions.push(instr);
                    return (var_id, instructions);
                }
                UnaryOp::BitInvert => {
                    let (final_location, mut instructions) = expression.generate_tacky();
                    let var_id = VariableId::new_id();
                    let instr = TackyInstruction::BitwiseNegate(var_id, final_location);

                    instructions.push(instr);
                    return (var_id, instructions);
                }
            },
        }
    }
}
