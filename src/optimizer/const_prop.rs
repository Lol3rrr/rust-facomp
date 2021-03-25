use crate::frontend::ir::{IRExpression, IRFunction, IRNode, IROperation, IRValue};

fn combine_two(op: &IROperation, v1: &IRValue, v2: &IRValue) -> IRExpression {
    match (v1, v2) {
        (IRValue::Number(n1), IRValue::Number(n2)) => {
            let result = match op {
                IROperation::Add => n1 + n2,
                IROperation::Sub => n1 - n2,
                IROperation::Divide => n1 / n2,
                IROperation::Multiply => n1 * n2,
            };

            IRExpression::Value(IRValue::Number(result))
        }
    }
}

fn propagate_exp(exp: &IRExpression) -> IRExpression {
    match exp {
        IRExpression::Operation(ref op, ref expressions) => {
            let first = expressions.get(0).unwrap();
            let second = expressions.get(1).unwrap();
            match (first, second) {
                (IRExpression::Value(v1), IRExpression::Value(v2)) => combine_two(op, v1, v2),
                (IRExpression::Value(v1), IRExpression::Operation(_, _)) => {
                    let v2 = match propagate_exp(second) {
                        IRExpression::Value(t) => t,
                        _ => return exp.clone(),
                    };

                    combine_two(op, v1, &v2)
                }
                (IRExpression::Operation(_, _), IRExpression::Value(v2)) => {
                    let v1 = match propagate_exp(first) {
                        IRExpression::Value(t) => t,
                        _ => return exp.clone(),
                    };

                    combine_two(op, &v1, v2)
                }
                (IRExpression::Operation(_, _), IRExpression::Operation(_, _)) => {
                    let v1 = match propagate_exp(first) {
                        IRExpression::Value(t) => t,
                        _ => return exp.clone(),
                    };

                    let v2 = match propagate_exp(second) {
                        IRExpression::Value(t) => t,
                        _ => return exp.clone(),
                    };

                    combine_two(op, &v1, &v2)
                }
                (_, IRExpression::Operation(_, _)) => {
                    let value = match propagate_exp(second) {
                        IRExpression::Value(t) => t,
                        _ => return exp.clone(),
                    };

                    IRExpression::Operation(
                        op.clone(),
                        vec![first.clone(), IRExpression::Value(value)],
                    )
                }
                (IRExpression::Operation(_, _), _) => {
                    let value = match propagate_exp(first) {
                        IRExpression::Value(t) => t,
                        _ => return exp.clone(),
                    };

                    IRExpression::Operation(
                        op.clone(),
                        vec![IRExpression::Value(value), second.clone()],
                    )
                }
                (_, _) => exp.clone(),
            }
        }
        _ => exp.clone(),
    }
}

pub fn propagate(ir: &mut IRFunction) {
    for statement in ir.statements.iter_mut() {
        for part in statement.iter_mut() {
            match part {
                IRNode::Assignment(_, exp) => {
                    let n_exp = propagate_exp(exp);
                    drop(std::mem::replace(exp, n_exp));
                }
                IRNode::Call(_, exp) => {
                    let mut n_exp = Vec::new();
                    for tmp in exp.iter() {
                        n_exp.push(propagate_exp(tmp));
                    }

                    drop(std::mem::replace(exp, n_exp));
                }
                IRNode::DeclareVariable(_, _) => {}
                IRNode::Conditional(_, _) => {}
            };
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one_propagate() {
        let exp = IRExpression::Operation(
            IROperation::Add,
            vec![
                IRExpression::Value(IRValue::Number(1)),
                IRExpression::Value(IRValue::Number(2)),
            ],
        );
        let expected = IRExpression::Value(IRValue::Number(3));

        assert_eq!(expected, propagate_exp(&exp));
    }

    #[test]
    fn nested_propagate_as_second() {
        let exp = IRExpression::Operation(
            IROperation::Add,
            vec![
                IRExpression::Value(IRValue::Number(1)),
                IRExpression::Operation(
                    IROperation::Add,
                    vec![
                        IRExpression::Value(IRValue::Number(2)),
                        IRExpression::Value(IRValue::Number(3)),
                    ],
                ),
            ],
        );
        let expected = IRExpression::Value(IRValue::Number(6));

        assert_eq!(expected, propagate_exp(&exp));
    }
    #[test]
    fn nested_propagate_as_first() {
        let exp = IRExpression::Operation(
            IROperation::Add,
            vec![
                IRExpression::Operation(
                    IROperation::Add,
                    vec![
                        IRExpression::Value(IRValue::Number(2)),
                        IRExpression::Value(IRValue::Number(3)),
                    ],
                ),
                IRExpression::Value(IRValue::Number(1)),
            ],
        );
        let expected = IRExpression::Value(IRValue::Number(6));

        assert_eq!(expected, propagate_exp(&exp));
    }

    #[test]
    fn two_expressions() {
        let exp = IRExpression::Operation(
            IROperation::Add,
            vec![
                IRExpression::Operation(
                    IROperation::Add,
                    vec![
                        IRExpression::Value(IRValue::Number(2)),
                        IRExpression::Value(IRValue::Number(5)),
                    ],
                ),
                IRExpression::Operation(
                    IROperation::Add,
                    vec![
                        IRExpression::Value(IRValue::Number(1)),
                        IRExpression::Value(IRValue::Number(2)),
                    ],
                ),
            ],
        );
        let expected = IRExpression::Value(IRValue::Number(10));

        assert_eq!(expected, propagate_exp(&exp));
    }
}
