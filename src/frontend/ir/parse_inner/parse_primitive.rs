use std::iter::Peekable;

use crate::frontend::{
    ir::{IRNode, IRType},
    lexer::{Primitives, Token},
};

pub fn parse<'a, I>(prim: &Primitives, iter: &mut Peekable<I>) -> Option<IRNode>
where
    I: Iterator<Item = &'a Token>,
{
    let ir_type = match prim {
        Primitives::Number => IRType::Number,
    };

    let next_token = iter.peek()?;

    match next_token {
        Token::Identifier(ref name) => Some(IRNode::DeclareVariable(name.clone(), ir_type)),
        _ => {
            log::error!("Expected Identifier");
            None
        }
    }
}
