use crate::{
    backend::{
        archs::x86_64::asm::{Instruction, Register},
        VariableOffsets,
    },
    frontend::ir::IRComparison,
};

use super::expression;

/// Generates the instructions to jump to the Target
/// when the condition is false / the opposite
pub fn generate_inverse_jump(
    cond: &IRComparison,
    jump_target: String,
    variables: &VariableOffsets,
) -> Vec<Instruction> {
    let mut result = Vec::new();

    match cond {
        IRComparison::Equals(left, right) | IRComparison::GreaterThan(left, right) => {
            result.append(&mut expression::generate(&left, variables));
            result.push(Instruction::Push(Register::RAX.to_string()));
            result.append(&mut expression::generate(&right, variables));
            result.push(Instruction::Pop(Register::RBX.to_string()));

            result.push(Instruction::Cmp(
                Register::RBX.to_string(),
                Register::RAX.to_string(),
            ));

            let jump_instr = match cond {
                IRComparison::Equals(_, _) => Instruction::Jne(jump_target),
                IRComparison::GreaterThan(_, _) => Instruction::Jle(jump_target),
            };
            result.push(jump_instr);
        }
    };

    result
}

/// Generates the instructions to jump to the Target
/// when the condition is false / the opposite
pub fn generate_jump(
    cond: &IRComparison,
    jump_target: String,
    variables: &VariableOffsets,
) -> Vec<Instruction> {
    let mut result = Vec::new();

    match cond {
        IRComparison::Equals(left, right) | IRComparison::GreaterThan(left, right) => {
            result.append(&mut expression::generate(&left, variables));
            result.push(Instruction::Push(Register::RAX.to_string()));
            result.append(&mut expression::generate(&right, variables));
            result.push(Instruction::Pop(Register::RBX.to_string()));

            result.push(Instruction::Cmp(
                Register::RBX.to_string(),
                Register::RAX.to_string(),
            ));

            let jump_instr = match cond {
                IRComparison::Equals(_, _) => Instruction::Je(jump_target),
                IRComparison::GreaterThan(_, _) => Instruction::Jg(jump_target),
            };
            result.push(jump_instr);
        }
    };

    result
}

#[cfg(test)]
mod tests {
    use crate::frontend::ir::{IRExpression, IRValue};

    use super::*;

    #[test]
    fn simple_inverse_equal() {
        let comparison = IRComparison::Equals(
            IRExpression::Value(IRValue::Number(1)),
            IRExpression::Value(IRValue::Number(2)),
        );
        let target = "test_target".to_string();
        let variables = VariableOffsets::new();

        let expected = vec![
            Instruction::Move(Register::RAX.to_string(), "1".to_string()),
            Instruction::Push(Register::RAX.to_string()),
            Instruction::Move(Register::RAX.to_string(), "2".to_string()),
            Instruction::Pop(Register::RBX.to_string()),
            Instruction::Cmp(Register::RBX.to_string(), Register::RAX.to_string()),
            Instruction::Jne(target.clone()),
        ];

        assert_eq!(
            expected,
            generate_inverse_jump(&comparison, target, &variables)
        );
    }

    #[test]
    fn simple_inverse_greater_than() {
        let comparison = IRComparison::GreaterThan(
            IRExpression::Value(IRValue::Number(1)),
            IRExpression::Value(IRValue::Number(2)),
        );
        let target = "test_target".to_string();
        let variables = VariableOffsets::new();

        let expected = vec![
            Instruction::Move(Register::RAX.to_string(), "1".to_string()),
            Instruction::Push(Register::RAX.to_string()),
            Instruction::Move(Register::RAX.to_string(), "2".to_string()),
            Instruction::Pop(Register::RBX.to_string()),
            Instruction::Cmp(Register::RBX.to_string(), Register::RAX.to_string()),
            Instruction::Jle(target.clone()),
        ];

        assert_eq!(
            expected,
            generate_inverse_jump(&comparison, target, &variables)
        );
    }

    #[test]
    fn simple_equal() {
        let comparison = IRComparison::Equals(
            IRExpression::Value(IRValue::Number(1)),
            IRExpression::Value(IRValue::Number(2)),
        );
        let target = "test_target".to_string();
        let variables = VariableOffsets::new();

        let expected = vec![
            Instruction::Move(Register::RAX.to_string(), "1".to_string()),
            Instruction::Push(Register::RAX.to_string()),
            Instruction::Move(Register::RAX.to_string(), "2".to_string()),
            Instruction::Pop(Register::RBX.to_string()),
            Instruction::Cmp(Register::RBX.to_string(), Register::RAX.to_string()),
            Instruction::Je(target.clone()),
        ];

        assert_eq!(expected, generate_jump(&comparison, target, &variables));
    }

    #[test]
    fn simple_greater_than() {
        let comparison = IRComparison::GreaterThan(
            IRExpression::Value(IRValue::Number(1)),
            IRExpression::Value(IRValue::Number(2)),
        );
        let target = "test_target".to_string();
        let variables = VariableOffsets::new();

        let expected = vec![
            Instruction::Move(Register::RAX.to_string(), "1".to_string()),
            Instruction::Push(Register::RAX.to_string()),
            Instruction::Move(Register::RAX.to_string(), "2".to_string()),
            Instruction::Pop(Register::RBX.to_string()),
            Instruction::Cmp(Register::RBX.to_string(), Register::RAX.to_string()),
            Instruction::Jg(target.clone()),
        ];

        assert_eq!(expected, generate_jump(&comparison, target, &variables));
    }
}
