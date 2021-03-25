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
    Call(IRIdentifier, Vec<IRExpression>),
    Conditional(IRComparison, Vec<Vec<IRNode>>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct IRParameter {
    pub name: IRIdentifier,
    pub param_type: IRType,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IRFunction {
    pub name: String,
    pub parameters: Vec<IRParameter>,
    pub statements: Vec<Vec<IRNode>>,
}

pub fn parse(tokens: &[Token]) -> Option<std::collections::HashMap<String, IRFunction>> {
    let mut result = std::collections::HashMap::new();

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
                    name: name.clone(),
                    parameters: arguments,
                    statements: inner,
                };
                result.insert(name, func);
            }
            _ => {
                log::error!("Unexpected: {:?}", current);
            }
        };
    }

    Some(result)
}
