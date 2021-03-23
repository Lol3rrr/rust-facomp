use crate::backend::asm::Instruction;

// Calling print, prints out the value in eax
pub fn generate(instr: &mut Vec<Instruction>) {
    instr.push(Instruction::Label("print".to_owned()));

    instr.push(Instruction::Move("[rsp - 1]".to_owned(), "al".to_owned())); // Putting the value to be printed on the Stack
    instr.push(Instruction::Move("eax".to_owned(), "1".to_owned()));
    instr.push(Instruction::Move("edi".to_owned(), "eax".to_owned()));
    instr.push(Instruction::Lea("rsi".to_owned(), "[rsp - 1]".to_owned()));
    instr.push(Instruction::Move("edx".to_owned(), "1".to_owned()));
    instr.push(Instruction::Syscall);

    instr.push(Instruction::Return);
}
