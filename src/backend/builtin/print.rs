use crate::backend::asm::Instruction;
use crate::backend::builtin::helper;

// Calling print, prints out the value in eax
pub fn generate(instr: &mut Vec<Instruction>) {
    helper::help_func(
        "print",
        instr,
        vec![
            Instruction::Move("rax".to_owned(), "[rbp + 16]".to_owned()),
            Instruction::Move("[rbp - 1]".to_owned(), "al".to_owned()),
            Instruction::Move("eax".to_owned(), "1".to_owned()),
            Instruction::Move("edi".to_owned(), "eax".to_owned()),
            Instruction::Lea("rsi".to_owned(), "[rbp - 1]".to_owned()),
            Instruction::Move("edx".to_owned(), "1".to_owned()),
            Instruction::Syscall,
        ],
    );
}
