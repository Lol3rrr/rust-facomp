use std::iter::Peekable;

use crate::frontend::{
    ir::{parse_expression::parse_expression, IRNode},
    lexer::{BuiltIns, Token},
};

pub fn parse<'a, I>(builtin: &BuiltIns, iter: &mut Peekable<I>) -> Option<IRNode>
where
    I: Iterator<Item = &'a Token>,
{
    let next_token = iter.peek()?;

    match next_token {
        Token::OpenParan => {
            iter.next().unwrap();

            let inner = parse_expression(iter)?;
            iter.next().unwrap();

            let func_name = match builtin {
                BuiltIns::Print => "print".to_owned(),
            };

            Some(IRNode::Call(func_name, inner))
        }
        _ => {
            log::error!("Unknown operation for identifier: {:?}", next_token);
            None
        }
    }
}
