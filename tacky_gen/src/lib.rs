use slk_c_core::parser_core::parser_ir::Program;

mod generator;

#[derive(Debug, Clone, PartialEq, PartialOrd, Hash)]
pub struct TackyGenerator {
    program: Program,
}
