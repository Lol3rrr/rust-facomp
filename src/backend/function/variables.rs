use crate::{
    backend::VariableOffsets,
    frontend::ir::{IRFunction, IRNode, IRType},
};

pub fn generate_offsets(func: &IRFunction) -> (VariableOffsets, u64) {
    let mut vars = VariableOffsets::new();
    let mut offset = 0;

    for statement in func.statements.iter() {
        for part in statement {
            if let &IRNode::DeclareVariable(ref name, ref var_type) = part {
                let size = match var_type {
                    IRType::Number => 8,
                };

                vars.insert(name.clone(), offset);
                offset += size;
            };
        }
    }

    (vars, offset)
}
