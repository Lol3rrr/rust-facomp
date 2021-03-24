use std::iter::Peekable;

use crate::frontend::{
    ir::{parse_expression, IRExpression, IRNode},
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
                _ => None,
            }
        }
        Token::OpenParan => {
            iter.next().unwrap();

            match iter.peek() {
                Some(Token::ClosingParan) => {
                    iter.next().unwrap();

                    Some(IRNode::Call(name.to_owned(), IRExpression::Noop))
                }
                _ => {
                    unimplemented!("Calling custom functions with params not supported yet");
                }
            }
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
