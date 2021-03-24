use super::lexer::Token;

mod parse_expression;
pub use parse_expression::parse_expression;

mod pretty_print;
pub use pretty_print::pretty_print;

mod parse_arguments;
mod parse_inner;

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

                match iter.peek() {
                    Some(Token::OpenParan) => iter.next(),
                    _ => return None,
                };

                let arguments = parse_arguments::parse(&mut iter);
                log::debug!("Function-Arguments: {:?}", arguments);

                match iter.peek() {
                    Some(Token::ClosingParan) => iter.next(),
                    _ => return None,
                };
                match iter.peek() {
                    Some(Token::OpenCurly) => iter.next(),
                    _ => return None,
                };

                let inner = parse_inner::inner_parse(&mut iter)?;

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

    use crate::frontend::lexer::Primitives;

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
