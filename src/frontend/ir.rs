use std::iter::Peekable;

use super::lexer::{BuiltIns, Comparisons, Primitives, Token};

mod parse_expression;
pub use parse_expression::parse_expression;

mod pretty_print;
pub use pretty_print::pretty_print;

pub type IRIdentifier = String;

#[derive(Debug, PartialEq, Clone)]
pub enum IRType {
    Number,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IRValue {
    Number(u64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IROperation {
    Add,
    Sub,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IRExpression {
    Operation(IROperation, Vec<IRExpression>),
    Value(IRValue),
    Variable(IRIdentifier),
    Noop,
}

#[derive(Debug, PartialEq, Clone)]
pub enum IRComparison {
    Equals(IRExpression, IRExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum IRNode {
    DeclareVariable(IRIdentifier, IRType),
    Assignment(IRIdentifier, IRExpression),
    Call(IRIdentifier, IRExpression),
    Conditional(IRComparison, Vec<Vec<IRNode>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IRFunction {
    pub name: String,
    pub statements: Vec<Vec<IRNode>>,
}

fn inner_parse<'a, I>(iter: &mut Peekable<I>) -> Option<Vec<Vec<IRNode>>>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    let mut current_statement = Vec::with_capacity(3);
    while let Some(token) = iter.next() {
        match token {
            Token::Primitive(ref prim) => {
                let ir_type = match prim {
                    Primitives::Number => IRType::Number,
                };

                let next_token = match iter.peek() {
                    Some(n) => n,
                    None => return None,
                };

                match next_token {
                    Token::Identifier(ref name) => {
                        current_statement.push(IRNode::DeclareVariable(name.clone(), ir_type));
                    }
                    _ => return None,
                };
            }
            Token::Identifier(ref name) => {
                let next_token = match iter.peek() {
                    Some(n) => n,
                    None => return None,
                };

                match next_token {
                    Token::Assignment => {
                        // Advance the iterator
                        iter.next().unwrap();

                        if let Some(exp) = parse_expression(iter) {
                            current_statement.push(IRNode::Assignment(name.clone(), exp));
                        }
                    }
                    Token::OpenParan => {
                        iter.next().unwrap();

                        match iter.peek() {
                            Some(Token::ClosingParan) => {
                                iter.next().unwrap();

                                current_statement
                                    .push(IRNode::Call(name.to_owned(), IRExpression::Noop));
                            }
                            _ => {
                                unimplemented!("Calling functions with params not supported yet");
                            }
                        };
                    }
                    _ => {
                        log::error!(
                            "Unknown operation for identifier('{}'): {:?}",
                            name,
                            next_token
                        );
                    }
                };
            }
            Token::Builtin(ref builtin) => {
                let next_token = iter.peek()?;

                match next_token {
                    Token::OpenParan => {
                        iter.next().unwrap();

                        let inner = parse_expression(iter)?;
                        iter.next().unwrap();

                        let func_name = match builtin {
                            BuiltIns::Print => "print".to_owned(),
                        };

                        current_statement.push(IRNode::Call(func_name, inner));
                    }
                    _ => {
                        log::error!("Unknown operation for identifier: {:?}", next_token);
                    }
                };
            }
            Token::Semicolon => {
                result.push(current_statement.clone());
                current_statement.clear();
            }
            Token::If => {
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

                current_statement.push(IRNode::Conditional(comp, inner_scope));
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

// TODO
fn parse_arguments<'a, I>(iter: &mut Peekable<I>)
where
    I: Iterator<Item = &'a Token>,
{
}

pub fn parse(tokens: &[Token]) -> Option<Vec<IRFunction>> {
    let mut result = Vec::new();

    let mut iter = tokens.iter().peekable();
    while let Some(current) = iter.next() {
        match current {
            Token::Function => {
                let name = match iter.peek() {
                    Some(Token::Identifier(name)) => {
                        iter.next().unwrap();
                        name.clone()
                    }
                    _ => return None,
                };

                log::debug!("Parsing-Function: {:?}", name);

                match iter.peek() {
                    Some(Token::OpenParan) => iter.next(),
                    _ => return None,
                };

                let arguments = parse_arguments(&mut iter);
                log::debug!("Function-Arguments: {:?}", arguments);

                match iter.peek() {
                    Some(Token::ClosingParan) => iter.next(),
                    _ => return None,
                };
                match iter.peek() {
                    Some(Token::OpenCurly) => iter.next(),
                    _ => return None,
                };

                let inner = inner_parse(&mut iter)?;

                let func = IRFunction {
                    name,
                    statements: inner,
                };
                result.push(func);
            }
            _ => {
                log::error!("Unexpected: {:?}", current);
            }
        };
    }

    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_initialize() {
        let tokens = vec![
            Token::Primitive(Primitives::Number),
            Token::Identifier("test".to_owned()),
            Token::Assignment,
            Token::ValueNumber(1),
            Token::Semicolon,
        ];

        let expected = vec![IRFunction {
            name: "main".to_owned(),
            statements: vec![vec![
                IRNode::DeclareVariable("test".to_owned(), IRType::Number),
                IRNode::Assignment("test".to_owned(), IRExpression::Value(IRValue::Number(1))),
            ]],
        }];

        assert_eq!(Some(expected), parse(&tokens));
    }

    #[test]
    fn initialize_with_simple_expression() {
        let tokens = vec![
            Token::Primitive(Primitives::Number),
            Token::Identifier("test".to_owned()),
            Token::Assignment,
            Token::ValueNumber(1),
            Token::Plus,
            Token::Identifier("other".to_owned()),
            Token::Semicolon,
        ];

        let expected = vec![IRFunction {
            name: "main".to_owned(),
            statements: vec![vec![
                IRNode::DeclareVariable("test".to_owned(), IRType::Number),
                IRNode::Assignment(
                    "test".to_owned(),
                    IRExpression::Operation(
                        IROperation::Add,
                        vec![
                            IRExpression::Value(IRValue::Number(1)),
                            IRExpression::Variable("other".to_owned()),
                        ],
                    ),
                ),
            ]],
        }];

        assert_eq!(Some(expected), parse(&tokens));
    }
}
