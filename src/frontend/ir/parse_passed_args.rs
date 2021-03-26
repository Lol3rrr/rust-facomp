use std::iter::Peekable;

use crate::frontend::lexer::Token;

use super::{IRExpression, IRValue};

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<IRExpression>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    match iter.peek() {
        Some(Token::OpenParan) => {}
        _ => return None,
    };
    iter.next().unwrap();

    while let Some(peeked) = iter.peek() {
        match peeked {
            Token::ClosingParan => {
                iter.next().unwrap();
                break;
            }
            Token::ValueNumber(value) => {
                iter.next().unwrap();
                result.push(IRExpression::Value(IRValue::Number(*value)));
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

    Some(result)
}
