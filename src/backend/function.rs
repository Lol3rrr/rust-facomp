use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::frontend::ir::{IRComparison, IRFunction, IRNode};

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
                for tmp_exp in exp.iter().rev() {
                    result.append(&mut expression::generate(tmp_exp, variables));
                    result.push(Instruction::Push("rax".to_owned()));
                }
                result.push(Instruction::Call(func_name.clone()));
                for _ in exp.iter() {
                    result.push(Instruction::Add("rsp".to_owned(), "8".to_owned()));
                }
            }
            &IRNode::Conditional(ref cond, ref nodes) => {
                let rand_string: String = thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(10)
                    .map(char::from)
                    .collect();

                let end_target = format!("{}_END", rand_string);

                match cond {
                    IRComparison::Equals(left, right) => {
                        result.append(&mut expression::generate(&left, variables));
                        result.push(Instruction::Push("rax".to_owned()));
                        result.append(&mut expression::generate(&right, variables));
                        result.push(Instruction::Pop("rbx".to_owned()));
                        result.push(Instruction::Cmp("rbx".to_owned(), "rax".to_owned()));
                        result.push(Instruction::Jne(end_target.clone()));
                    }
                };

                for cond_statements in nodes.iter() {
                    result.append(&mut generate_statement(&cond_statements, variables));
                }

                result.push(Instruction::Label(end_target));
            }
        };
    }

    result
}

pub fn generate_function(func: &IRFunction) -> Vec<Instruction> {
    let mut final_asm = Vec::new();

    let (vars, var_offset) = variables::generate_offsets(func);

    // TODO
    // Add support for actually generating code to deal with params

    final_asm.push(Instruction::Label(func.name.clone()));
    final_asm.push(Instruction::Push("rbp".to_owned()));
    final_asm.push(Instruction::Move("rbp".to_owned(), "rsp".to_owned()));
    final_asm.push(Instruction::Sub(
        "rsp".to_owned(),
        format!("{}", var_offset),
    ));

    // Actual code
    for statement in func.statements.iter() {
        final_asm.append(&mut generate_statement(statement, &vars));
    }

    final_asm.push(Instruction::Add(
        "rsp".to_owned(),
        format!("{}", var_offset),
    ));
    final_asm.push(Instruction::Pop("rbp".to_owned()));
    final_asm.push(Instruction::Return);

    final_asm
}
