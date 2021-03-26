use crate::frontend::ir::IRFunction;

pub trait Arch {
    fn generate_asm_string(ir: std::collections::HashMap<String, IRFunction>) -> String;
}
