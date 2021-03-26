use crate::{
    backend::{asm::Instruction, VariableOffsets},
    frontend::ir::{IRExpression, IROperation, IRValue},
};

// The Result of an evaluated expression will always be placed 'eax'
pub fn generate(exp: &IRExpression, vars: &VariableOffsets) -> Vec<Instruction> {
    let mut result = Vec::new();

    let target = "eax".to_owned();
    match exp {
        &IRExpression::Value(ref ir_value) => match ir_value {
            IRValue::Number(ref value) => {
                result.push(Instruction::Move(target, format!("{}", value)));
            }
        },
        &IRExpression::Variable(ref var_name) => {
            let source_offset = vars.get(var_name).unwrap();
            let source = format!("[rbp - {}]", source_offset);
            result.push(Instruction::Move(target, source));
        }
        &IRExpression::Operation(ref operation, ref other_exp) => {
            let first = other_exp.get(0).unwrap();
            let second = other_exp.get(1).unwrap();

            result.append(&mut generate(first, vars));
            result.push(Instruction::Push("rax".to_owned()));
            result.append(&mut generate(second, vars));

            result.push(Instruction::Pop("rbx".to_owned()));

            match operation {
                &IROperation::Add => {
                    result.push(Instruction::Add("eax".to_owned(), "ebx".to_owned()));
                }
                &IROperation::Sub => {
                    result.push(Instruction::Sub("eax".to_owned(), "ebx".to_owned()));
                }
                _ => {
                    println!("Unknown OP: {:?}", operation);
                }
            };
        }
        &IRExpression::Call(ref func_name, ref exp) => {
            for tmp_exp in exp.iter().rev() {
                result.append(&mut generate(tmp_exp, vars));
                result.push(Instruction::Push("rax".to_owned()));
            }
            result.push(Instruction::Call(func_name.clone()));
            for _ in exp.iter() {
                result.push(Instruction::Add("rsp".to_owned(), "8".to_owned()));
            }
        }
        &IRExpression::Noop => {}
    };

    result
}
