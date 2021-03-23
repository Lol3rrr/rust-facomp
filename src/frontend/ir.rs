use super::lexer::{BuiltIns, Primitives, Token};

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
}

#[derive(Debug, PartialEq, Clone)]
pub enum IRNode {
    DeclareVariable(IRIdentifier, IRType),
    Assignment(IRIdentifier, IRExpression),
    Call(IRIdentifier, IRExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IRFunction {
    pub name: String,
    pub statements: Vec<Vec<IRNode>>,
}

pub fn parse(tokens: &[Token]) -> Option<Vec<IRFunction>> {
    let mut result = Vec::new();

    let mut current_statement = Vec::with_capacity(3);
    let mut iter = tokens.iter().peekable();
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

                        if let Some(exp) = parse_expression(&mut iter) {
                            current_statement.push(IRNode::Assignment(name.clone(), exp));
                        }
                    }
                    _ => {
                        println!("Unknown operation for identifier: {:?}", next_token);
                    }
                };
            }
            Token::Builtin(ref builtin) => {
                let next_token = iter.peek()?;

                match next_token {
                    Token::OpenParan => {
                        iter.next().unwrap();

                        let inner = parse_expression(&mut iter)?;
                        iter.next().unwrap();

                        let func_name = match builtin {
                            BuiltIns::Print => "print".to_owned(),
                        };

                        current_statement.push(IRNode::Call(func_name, inner));
                    }
                    _ => {
                        println!("Unknown operation for identifier: {:?}", next_token);
                    }
                };
            }
            Token::Semicolon => {
                result.push(current_statement.clone());
                current_statement.clear();
            }
            _ => {
                println!("Unknown: {:?}", token);
            }
        };
    }

    Some(vec![IRFunction {
        name: "main".to_owned(),
        statements: result,
    }])
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
