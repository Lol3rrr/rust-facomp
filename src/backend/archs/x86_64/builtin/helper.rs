use crate::backend::archs::x86_64::asm::{Instruction, Register};

pub fn help_func(name: &str, instr: &mut Vec<Instruction>, mut content: Vec<Instruction>) {
    instr.push(Instruction::Label(name.to_owned()));

    instr.push(Instruction::Push(Register::RBP.to_string()));
    instr.push(Instruction::Move(
        Register::RBP.to_string(),
        Register::RSP.to_string(),
    ));

    for content_item in content.drain(..) {
        instr.push(content_item);
    }

    instr.push(Instruction::Pop(Register::RBP.to_string()));

    instr.push(Instruction::Return);
}
