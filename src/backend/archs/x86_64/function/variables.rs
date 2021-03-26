use crate::{
    backend::VariableOffsets,
    frontend::ir::{IRFunction, IRNode, IRType},
};

pub fn generate_offsets(func: &IRFunction) -> (VariableOffsets, i64) {
    let mut vars = VariableOffsets::new();
    let mut param_offset = -16;
    let mut local_offset = 8;

    for param in func.parameters.iter() {
        vars.insert(param.name.clone(), param_offset);

        match param.param_type {
            IRType::Number => {
                param_offset -= 8;
            }
        };
    }

    for statement in func.statements.iter() {
        for part in statement {
            if let &IRNode::DeclareVariable(ref name, ref var_type) = part {
                let size = match var_type {
                    IRType::Number => 8,
                };

                vars.insert(name.clone(), local_offset);
                local_offset += size;
            };
        }
    }

    (vars, local_offset)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ir::{IRExpression, IRParameter};

    #[test]
    fn no_variables() {
        let function = IRFunction {
            name: "test".to_owned(),
            return_type: None,
            parameters: vec![],
            statements: vec![vec![IRNode::SingleExpression(IRExpression::Call(
                "test_func".to_owned(),
                vec![],
            ))]],
        };

        let expected_vars = VariableOffsets::new();
        let expected_offset = 0;

        let (result_vars, result_offset) = generate_offsets(&function);

        assert_eq!(expected_vars, result_vars);
        assert_eq!(expected_offset, result_offset);
    }

    #[test]
    fn one_variable() {
        let function = IRFunction {
            name: "test".to_owned(),
            return_type: None,
            parameters: vec![],
            statements: vec![vec![IRNode::DeclareVariable(
                "test_var".to_owned(),
                IRType::Number,
            )]],
        };

        let mut expected_vars = VariableOffsets::new();
        expected_vars.insert("test_var".to_owned(), 0);
        let expected_offset = 8;

        let (result_vars, result_offset) = generate_offsets(&function);

        assert_eq!(expected_vars, result_vars);
        assert_eq!(expected_offset, result_offset);
    }

    #[test]
    fn one_param() {
        let function = IRFunction {
            name: "test".to_owned(),
            return_type: None,
            parameters: vec![IRParameter {
                name: "test_param".to_owned(),
                param_type: IRType::Number,
            }],
            statements: vec![],
        };

        let mut expected_vars = VariableOffsets::new();
        expected_vars.insert("test_param".to_owned(), -16);
        let expected_offset = 0;

        let (result_vars, result_offset) = generate_offsets(&function);

        assert_eq!(expected_vars, result_vars);
        assert_eq!(expected_offset, result_offset);
    }
    #[test]
    fn two_params() {
        let function = IRFunction {
            name: "test".to_owned(),
            return_type: None,
            parameters: vec![
                IRParameter {
                    name: "test_param1".to_owned(),
                    param_type: IRType::Number,
                },
                IRParameter {
                    name: "test_param2".to_owned(),
                    param_type: IRType::Number,
                },
            ],
            statements: vec![],
        };

        let mut expected_vars = VariableOffsets::new();
        expected_vars.insert("test_param1".to_owned(), -16);
        expected_vars.insert("test_param2".to_owned(), -24);
        let expected_offset = 0;

        let (result_vars, result_offset) = generate_offsets(&function);

        assert_eq!(expected_vars, result_vars);
        assert_eq!(expected_offset, result_offset);
    }
}
