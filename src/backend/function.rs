use crate::frontend::ir::{IRFunction, IRNode};

use super::{asm::Instruction, VariableOffsets};

mod expression;
mod variables;

fn generate_statement(statement: &[IRNode], variables: &VariableOffsets) -> Vec<Instruction> {
    let mut result = Vec::new();

    for step in statement.iter() {
        match step {
            &IRNode::Assignment(ref var_name, ref exp) => {
                let target_offset = variables.get(var_name).unwrap();
                let target = format!("[rbp - {}]", target_offset);

                result.append(&mut expression::generate(exp, variables));

                result.push(Instruction::Move(target, "eax".to_owned()));
            }
            &IRNode::DeclareVariable(_, _) => {}
            &IRNode::Call(ref func_name, ref exp) => {
                result.append(&mut expression::generate(exp, variables));
                result.push(Instruction::Call(func_name.clone()));
            }
        };
    }

    result
}

pub fn generate_function(func: &IRFunction) -> Vec<Instruction> {
    let mut final_asm = Vec::new();

    let vars = variables::generate_offsets(func);

    final_asm.push(Instruction::Label(func.name.clone()));
    final_asm.push(Instruction::Push("rbp".to_owned()));
    final_asm.push(Instruction::Move("rbp".to_owned(), "rsp".to_owned()));

    // Actual code
    for statement in func.statements.iter() {
        final_asm.append(&mut generate_statement(statement, &vars));
    }

    final_asm.push(Instruction::Pop("rbp".to_owned()));
    final_asm.push(Instruction::Return);

    final_asm
}
