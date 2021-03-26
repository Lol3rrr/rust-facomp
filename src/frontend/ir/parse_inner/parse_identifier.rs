use std::iter::Peekable;

use crate::frontend::{
    ir::{parse_expression, parse_passed_args, IRExpression, IRNode},
    lexer::Token,
};

pub fn parse<'a, I>(name: &str, iter: &mut Peekable<I>) -> Option<IRNode>
where
    I: Iterator<Item = &'a Token>,
{
    let next_token = match iter.peek() {
        Some(n) => n,
        None => return None,
    };

    match next_token {
        Token::Assignment => {
            // Advance the iterator
            iter.next().unwrap();

            match parse_expression(iter) {
                Some(exp) => Some(IRNode::Assignment(name.to_owned(), exp)),
                _ => {
                    log::error!("Parsing Expression");
                    None
                }
            }
        }
        Token::OpenParan => {
            let inner = parse_passed_args::parse(iter)?;

            Some(IRNode::SingleExpression(IRExpression::Call(
                name.to_owned(),
                inner,
            )))
        }
        _ => {
            log::error!(
                "Unknown operation for identifier('{}'): {:?}",
                name,
                next_token
            );
            None
        }
    }
}
