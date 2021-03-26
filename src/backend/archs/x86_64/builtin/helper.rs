use crate::backend::archs::x86_64::asm::Instruction;

pub fn help_func(name: &str, instr: &mut Vec<Instruction>, mut content: Vec<Instruction>) {
    instr.push(Instruction::Label(name.to_owned()));

    instr.push(Instruction::Push("rbp".to_owned()));
    instr.push(Instruction::Move("rbp".to_owned(), "rsp".to_owned()));

    for content_item in content.drain(..) {
        instr.push(content_item);
    }

    instr.push(Instruction::Pop("rbp".to_owned()));

    instr.push(Instruction::Return);
}
