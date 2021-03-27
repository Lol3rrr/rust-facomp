use crate::backend::archs::x86_64::{
    asm::{Instruction, Register},
    builtin::helper,
};

// Calling print, prints out the value in eax
pub fn generate(instr: &mut Vec<Instruction>) {
    helper::help_func(
        "print",
        instr,
        vec![
            Instruction::Move(Register::RAX.to_string(), "[rbp + 16]".to_owned()),
            Instruction::Move("[rbp - 1]".to_owned(), "al".to_owned()),
            Instruction::Move(Register::RAX.to_string(), "1".to_owned()),
            Instruction::Move(Register::RDI.to_string(), Register::RAX.to_string()),
            Instruction::Lea(Register::RSI.to_string(), "[rbp - 1]".to_owned()),
            Instruction::Move(Register::RDX.to_string(), "1".to_owned()),
            Instruction::Syscall,
        ],
    );
}
