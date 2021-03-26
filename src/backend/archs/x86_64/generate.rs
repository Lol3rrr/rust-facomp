use crate::backend::archs::x86_64::{asm, builtin, function};
use crate::{backend::traits, frontend::ir::IRFunction};

pub type X86_64 = ();

impl traits::Arch for X86_64 {
    fn generate_asm_string(ir: std::collections::HashMap<String, IRFunction>) -> String {
        let mut final_asm = Vec::new();

        final_asm.push(asm::Instruction::Section(".text".to_owned()));
        final_asm.push(asm::Instruction::Raw("global _start".to_owned()));

        // First generate all the Builtin code
        final_asm.append(&mut builtin::generate_builtins());

        for (_, func) in ir {
            final_asm.append(&mut function::generate_function(&func));
        }

        final_asm.push(asm::Instruction::Label("_start".to_owned()));
        final_asm.push(asm::Instruction::Call("main".to_owned())); // Actually call main
        final_asm.push(asm::Instruction::Move("eax".to_owned(), "60".to_owned()));
        final_asm.push(asm::Instruction::Xor("rdi".to_owned(), "rdi".to_owned()));
        final_asm.push(asm::Instruction::Syscall);

        asm::format(&final_asm)
    }
}
