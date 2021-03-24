use std::iter::Peekable;

use super::{parse_expression, IRComparison, IRExpression, IRNode, IRType};
use crate::frontend::lexer::{BuiltIns, Comparisons, Primitives, Token};

fn parse_primitive<'a, I>(prim: &Primitives, iter: &mut Peekable<I>) -> Option<IRNode>
where
    I: Iterator<Item = &'a Token>,
{
    let ir_type = match prim {
        Primitives::Number => IRType::Number,
    };

    let next_token = iter.peek()?;

    match next_token {
        Token::Identifier(ref name) => Some(IRNode::DeclareVariable(name.clone(), ir_type)),
        _ => None,
    }
}

fn parse_identifier<'a, I>(name: &str, iter: &mut Peekable<I>) -> Option<IRNode>
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

fn parse_builtin<'a, I>(builtin: &BuiltIns, iter: &mut Peekable<I>) -> Option<IRNode>
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

fn parse_if<'a, I>(iter: &mut Peekable<I>) -> Option<IRNode>
where
    I: Iterator<Item = &'a Token>,
{
    match iter.peek().unwrap() {
        Token::OpenParan => {
            iter.next().unwrap();
        }
        _ => {
            log::error!("Unknown-Peek: {:?}", iter.peek());
            return None;
        }
    };

    let first_part = parse_expression(iter)?;

    let comparison_token = iter.next().unwrap();

    let second_part = parse_expression(iter)?;

    let comp = match comparison_token {
        Token::Comparison(comp) => match comp {
            Comparisons::Equal => IRComparison::Equals(first_part, second_part),
        },
        _ => return None,
    };

    match iter.next().unwrap() {
        Token::ClosingParan => {}
        _ => return None,
    };

    match iter.peek().unwrap() {
        Token::OpenCurly => {
            iter.next().unwrap();
        }
        _ => return None,
    };

    let inner_scope = inner_parse(iter)?;

    Some(IRNode::Conditional(comp, inner_scope))
}

pub fn inner_parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<Vec<IRNode>>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    let mut current_statement = Vec::with_capacity(3);
    while let Some(token) = iter.next() {
        match token {
            Token::Primitive(ref prim) => {
                let parsed = parse_primitive(prim, iter)?;
                current_statement.push(parsed);
            }
            Token::Identifier(ref name) => {
                let parsed = parse_identifier(name, iter)?;
                current_statement.push(parsed);
            }
            Token::Builtin(ref builtin) => {
                let parsed = parse_builtin(builtin, iter)?;
                current_statement.push(parsed);
            }
            Token::Semicolon => {
                result.push(current_statement.clone());
                current_statement.clear();
            }
            Token::If => {
                let parsed = parse_if(iter)?;
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
