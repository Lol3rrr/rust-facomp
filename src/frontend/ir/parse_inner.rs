use std::iter::Peekable;

use super::IRNode;
use crate::frontend::lexer::Token;

mod parse_builtin;
mod parse_identifier;
mod parse_if;
mod parse_primitive;

pub fn inner_parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<Vec<IRNode>>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    let mut current_statement = Vec::with_capacity(3);
    while let Some(token) = iter.next() {
        match token {
            Token::Primitive(ref prim) => {
                let parsed = parse_primitive::parse(prim, iter)?;
                current_statement.push(parsed);
            }
            Token::Identifier(ref name) => {
                let parsed = parse_identifier::parse(name, iter)?;
                current_statement.push(parsed);
            }
            Token::Builtin(ref builtin) => {
                let parsed = parse_builtin::parse(builtin, iter)?;
                current_statement.push(parsed);
            }
            Token::Semicolon => {
                result.push(current_statement.clone());
                current_statement.clear();
            }
            Token::If => {
                let parsed = parse_if::parse(iter)?;
                current_statement.push(parsed);

                result.push(current_statement.clone());
                current_statement.clear();
            }
            Token::ClosingCurly => return Some(result),
            _ => {
                log::error!("Unknown: {:?}", token);
            }
        };
    }

    Some(result)
}
