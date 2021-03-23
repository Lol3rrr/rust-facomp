use crate::frontend::{self, ir::IRFunction};

use std::collections::BTreeMap;

// Structure
//
// Local-Variables:
// Local variables are stored on the stack
// reference: https://bob.cs.sonoma.edu/IntroCompOrg-RPi/sec-varstack.html

pub mod asm;
mod builtin;
mod function;

/// Stores the Stack-Offset for every variable
/// in the current Scope
pub type VariableOffsets = BTreeMap<String, u64>;

pub fn generate(ir: Vec<IRFunction>) -> String {
    frontend::ir::pretty_print(&ir);

    println!("Generating Assembly");

    let mut final_asm = Vec::new();

    final_asm.push(asm::Instruction::Section(".text".to_owned()));
    final_asm.push(asm::Instruction::Raw("global _start".to_owned()));

    // First generate all the Builtin code
    final_asm.append(&mut builtin::generate_builtins());

    for func in ir {
        final_asm.append(&mut function::generate_function(&func));
    }

    final_asm.push(asm::Instruction::Label("_start".to_owned()));
    final_asm.push(asm::Instruction::Call("main".to_owned())); // Actually call main
    final_asm.push(asm::Instruction::Move("eax".to_owned(), "60".to_owned()));
    final_asm.push(asm::Instruction::Xor("rdi".to_owned(), "rdi".to_owned()));
    final_asm.push(asm::Instruction::Syscall);

    asm::format(&final_asm)
}
