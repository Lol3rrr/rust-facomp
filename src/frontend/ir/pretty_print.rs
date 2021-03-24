use super::{IRExpression, IRFunction, IRNode};

fn print_expression(prefix: &str, exp: &IRExpression) {
    let next_prefix = format!("{}  ", prefix);
    match exp {
        &IRExpression::Value(ref value) => {
            println!("{}Value: '{:?}'", prefix, value);
        }
        &IRExpression::Variable(ref name) => {
            println!("{}Variable: '{:?}'", prefix, name);
        }
        &IRExpression::Operation(ref op, ref exps) => {
            println!("{}Operation-'{:?}':", prefix, op);
            for exp in exps {
                print_expression(&next_prefix, exp);
            }
        }
    };
}

fn get_next_prefix(current: &str) -> String {
    format!("{}  ", current)
}

fn print_node(prefix: &str, node: &IRNode) {
    let next_prefix = get_next_prefix(prefix);
    match node {
        &IRNode::Call(ref name, ref exp) => {
            println!("{}Call-'{}':", prefix, name);
            print_expression(&next_prefix, exp);
        }
        &IRNode::Assignment(ref name, ref exp) => {
            println!("{}Assignment-'{}':", prefix, name);
            print_expression(&next_prefix, exp);
        }
        &IRNode::DeclareVariable(ref name, ref exp) => {
            println!("{}DeclareVariable-'{}':", prefix, name);
            println!("{}{:?}", next_prefix, exp);
        }
        &IRNode::Conditional(ref comparison, ref nodes) => {
            println!("{}Conditional:", prefix);
            println!("{}{:?}", next_prefix, comparison);
            let n_next_prefix = get_next_prefix(&next_prefix);
            for tmp in nodes {
                print_nodes(&n_next_prefix, tmp);
            }
        }
    };
}

fn print_nodes(prefix: &str, nodes: &[IRNode]) {
    for node in nodes.iter() {
        print_node(prefix, node);
    }
}

pub fn pretty_print(ir: &[IRFunction]) {
    for func in ir {
        println!("Function-'{}':", func.name);
        for statement in func.statements.iter() {
            println!("  Statement:");
            print_nodes("    ", statement);
        }
    }
}
