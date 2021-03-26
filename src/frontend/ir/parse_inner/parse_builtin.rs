use std::iter::Peekable;

use crate::frontend::{
    ir::{parse_expression::parse_expression, IRExpression},
    lexer::{BuiltIns, Token},
};

pub fn parse<'a, I>(builtin: &BuiltIns, iter: &mut Peekable<I>) -> Option<IRExpression>
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

            Some(IRExpression::Call(func_name, vec![inner]))
        }
        _ => {
            log::error!("Unknown operation for identifier: {:?}", next_token);
            None
        }
    }
}
