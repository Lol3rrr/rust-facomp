use rand::{thread_rng, Rng};

use crate::backend::VariableOffsets;
use crate::frontend::ir::{IRFunction, IRNode};

use super::asm::{Instruction, Register};

mod conditionals;
mod expression;
mod variables;

fn generate_random_string(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

    let mut rng = thread_rng();
    (0..length)
        .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())] as char)
        .collect()
}

fn generate_statement<F>(
    statement: &[IRNode],
    variables: &VariableOffsets,
    pre_return: &F,
) -> Vec<Instruction>
where
    F: Fn(&mut Vec<Instruction>),
{
    let mut result = Vec::new();

    for step in statement.iter() {
        match step {
            &IRNode::Assignment(ref var_name, ref exp) => {
                result.push(Instruction::Comment("Assignment".to_string()));
                let target_offset = match variables.get(var_name) {
                    Some(v) => v,
                    None => {
                        panic!("Cant find variable: {:?}", var_name);
                    }
                };
                let target = format!("[rbp - {}]", target_offset);

                result.append(&mut expression::generate(exp, variables));

                result.push(Instruction::Move(target, Register::RAX.to_string()));
            }
            &IRNode::DeclareVariable(_, _) => {}
            &IRNode::Conditional(ref cond, ref nodes) => {
                result.push(Instruction::Comment("Conditional".to_string()));
                let end_target = format!("{}_END", generate_random_string(10));

                result.append(&mut conditionals::generate_inverse_jump(
                    cond,
                    end_target.clone(),
                    variables,
                ));

                for cond_statements in nodes.iter() {
                    result.append(&mut generate_statement(
                        &cond_statements,
                        variables,
                        pre_return,
                    ));
                }

                result.push(Instruction::Label(end_target));
            }
            &IRNode::Loop(ref cond, ref nodes) => {
                result.push(Instruction::Comment("Loop".to_string()));
                let random_id = generate_random_string(10);
                let top_target = format!("{}_TOP", random_id);
                let end_target = format!("{}_END", random_id);

                // Skip the loop if the condition is FALSE
                result.append(&mut conditionals::generate_inverse_jump(
                    cond,
                    end_target.clone(),
                    variables,
                ));

                // The start of the loop
                result.push(Instruction::Label(top_target.clone()));

                // Generate all the actual loop code
                for cond_statements in nodes.iter() {
                    result.append(&mut generate_statement(
                        &cond_statements,
                        variables,
                        pre_return,
                    ));
                }

                // Generate the Comparison and jump to top to actually loop
                result.append(&mut conditionals::generate_jump(
                    cond, top_target, variables,
                ));

                // The end of the Loop
                // if this is reached the loop is done
                result.push(Instruction::Label(end_target));
            }
            IRNode::Return(ref raw_exp) => {
                result.push(Instruction::Comment("Return".to_string()));
                if let Some(exp) = raw_exp {
                    result.append(&mut expression::generate(exp, variables));
                }

                pre_return(&mut result);
                result.push(Instruction::Return);
            }
            IRNode::SingleExpression(ref exp) => {
                result.push(Instruction::Comment("Single-Expression".to_string()));
                result.append(&mut expression::generate(exp, variables));
            }
        };
    }

    result
}

pub fn generate_function(func: &IRFunction) -> Vec<Instruction> {
    let mut final_asm = Vec::new();

    let (vars, var_offset) = variables::generate_offsets(func);

    final_asm.push(Instruction::Label(func.name.clone()));
    final_asm.push(Instruction::Push(Register::RBP.to_string()));
    final_asm.push(Instruction::Move(
        Register::RBP.to_string(),
        Register::RSP.to_string(),
    ));
    final_asm.push(Instruction::Sub(
        Register::RSP.to_string(),
        format!("{}", var_offset),
    ));

    let pre_return = |instr: &mut Vec<Instruction>| {
        instr.push(Instruction::Move(
            Register::RSP.to_string(),
            Register::RBP.to_string(),
        ));
        instr.push(Instruction::Pop(Register::RBP.to_string()));
    };

    // Actual code
    for statement in func.statements.iter() {
        final_asm.append(&mut generate_statement(statement, &vars, &pre_return));
    }

    pre_return(&mut final_asm);
    final_asm.push(Instruction::Return);

    final_asm
}
