use std::iter::Peekable;

use super::{IRExpression, IRNode};
use crate::frontend::lexer::Token;

mod parse_builtin;
mod parse_identifier;
mod parse_if;
mod parse_primitive;
mod parse_while;

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
                current_statement.push(IRNode::SingleExpression(parsed));
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
            Token::While => {
                let parsed = parse_while::parse(iter)?;
                current_statement.push(parsed);

                result.push(current_statement.clone());
                current_statement.clear();
            }
            Token::Return => {
                let ret_exp = match iter.peek() {
                    Some(Token::Identifier(ref name)) => {
                        Some(IRExpression::Variable(name.to_owned()))
                    }
                    Some(Token::Semicolon) => None,
                    _ => return None,
                };

                if ret_exp.is_some() {
                    iter.next().unwrap();
                }

                current_statement.push(IRNode::Return(ret_exp));
            }
            Token::ClosingCurly => return Some(result),
            _ => {
                log::error!("Unknown: {:?}", token);
            }
        };
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::ir::{IRType, IRValue};
    use crate::frontend::lexer::Primitives;

    #[test]
    fn example_1() {
        let tokens = vec![
            Token::Primitive(Primitives::Number),
            Token::Identifier("test".to_owned()),
            Token::Assignment,
            Token::ValueNumber(12),
            Token::Semicolon,
        ];

        let expected = vec![vec![
            IRNode::DeclareVariable("test".to_owned(), IRType::Number),
            IRNode::Assignment("test".to_owned(), IRExpression::Value(IRValue::Number(12))),
        ]];

        assert_eq!(Some(expected), inner_parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn example_2() {
        let tokens = vec![
            Token::Identifier("test".to_owned()),
            Token::OpenParan,
            Token::ValueNumber(1),
            Token::Comma,
            Token::ValueNumber(5),
            Token::ClosingParan,
            Token::Semicolon,
        ];

        let expected = vec![vec![IRNode::SingleExpression(IRExpression::Call(
            "test".to_owned(),
            vec![
                IRExpression::Value(IRValue::Number(1)),
                IRExpression::Value(IRValue::Number(5)),
            ],
        ))]];

        assert_eq!(Some(expected), inner_parse(&mut tokens.iter().peekable()));
    }
}
