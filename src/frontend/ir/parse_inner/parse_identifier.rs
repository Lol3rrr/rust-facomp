use std::iter::Peekable;

use crate::frontend::{
    ir::{parse_expression, IRExpression, IRNode, IRValue},
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

            let mut inner = vec![];

            while let Some(peeked) = iter.peek() {
                match peeked {
                    Token::ClosingParan => {
                        iter.next().unwrap();
                        break;
                    }
                    Token::ValueNumber(value) => {
                        iter.next().unwrap();
                        inner.push(IRExpression::Value(IRValue::Number(*value)));
                    }
                    Token::Comma => {
                        iter.next().unwrap();
                    }
                    _ => {
                        log::error!("Unexpected: {:?}", peeked);
                        unimplemented!("Calling custom functions with params not supported yet");
                    }
                };
            }

            Some(IRNode::Call(name.to_owned(), inner))
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
