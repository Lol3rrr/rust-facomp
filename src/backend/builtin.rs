use super::asm::Instruction;

mod print;

pub fn generate_builtins() -> Vec<Instruction> {
    let mut result = Vec::new();

    print::generate(&mut result);

    result
}
