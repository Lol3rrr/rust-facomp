use rand::{distributions::Alphanumeric, thread_rng, Rng};

use crate::frontend::ir::{IRComparison, IRFunction, IRNode};

use super::{asm::Instruction, VariableOffsets};

mod expression;
mod variables;

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
                let target_offset = match variables.get(var_name) {
                    Some(v) => v,
                    None => {
                        panic!("Cant find variable: {:?}", var_name);
                    }
                };
                let target = format!("[rbp - {}]", target_offset);

                result.append(&mut expression::generate(exp, variables));

                result.push(Instruction::Move(target, "eax".to_owned()));
            }
            &IRNode::DeclareVariable(_, _) => {}
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
                    result.append(&mut generate_statement(
                        &cond_statements,
                        variables,
                        pre_return,
                    ));
                }

                result.push(Instruction::Label(end_target));
            }
            IRNode::Return(ref raw_exp) => {
                if let Some(exp) = raw_exp {
                    result.append(&mut expression::generate(exp, variables));
                }

                pre_return(&mut result);
                result.push(Instruction::Return);
            }
            IRNode::SingleExpression(ref exp) => {
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
    final_asm.push(Instruction::Push("rbp".to_owned()));
    final_asm.push(Instruction::Move("rbp".to_owned(), "rsp".to_owned()));
    final_asm.push(Instruction::Sub(
        "rsp".to_owned(),
        format!("{}", var_offset),
    ));

    let pre_return = |instr: &mut Vec<Instruction>| {
        instr.push(Instruction::Move("rsp".to_owned(), "rbp".to_owned()));
        instr.push(Instruction::Pop("rbp".to_owned()));
    };

    // Actual code
    for statement in func.statements.iter() {
        final_asm.append(&mut generate_statement(statement, &vars, &pre_return));
    }

    pre_return(&mut final_asm);
    final_asm.push(Instruction::Return);

    final_asm
}
