use std::iter::Peekable;

use crate::frontend::lexer::{Primitives, Token};

use super::{IRParameter, IRType};

// TODO
// Add support for more than one parameter
pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Vec<IRParameter>
where
    I: Iterator<Item = &'a Token>,
{
    let mut result = Vec::new();

    while let Some(peeked) = iter.peek() {
        if let Token::ClosingParan = peeked {
            break;
        }

        let next = iter.next().unwrap();
        match next {
            Token::Identifier(ref name) => {
                match iter.peek() {
                    Some(Token::Colon) => {
                        iter.next().unwrap();
                    }
                    _ => break,
                };

                match iter.peek() {
                    Some(Token::Primitive(_)) => {}
                    _ => break,
                };

                let param_type = match iter.next().unwrap() {
                    Token::Primitive(ref prim) => match prim {
                        Primitives::Number => IRType::Number,
                    },
                    _ => break,
                };

                result.push(IRParameter {
                    name: name.to_owned(),
                    param_type,
                });
            }
            Token::Comma => {}
            _ => {
                break;
            }
        };
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::frontend::lexer::Primitives;

    #[test]
    fn no_params() {
        let tokens = vec![Token::ClosingParan, Token::OpenCurly];

        let expected: Vec<IRParameter> = vec![];

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn one_param() {
        let tokens = vec![
            Token::Identifier("test1".to_owned()),
            Token::Colon,
            Token::Primitive(Primitives::Number),
            Token::ClosingParan,
            Token::OpenCurly,
        ];

        let expected = vec![IRParameter {
            name: "test1".to_owned(),
            param_type: IRType::Number,
        }];

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }

    #[test]
    fn two_params() {
        let tokens = vec![
            Token::Identifier("test1".to_owned()),
            Token::Colon,
            Token::Primitive(Primitives::Number),
            Token::Comma,
            Token::Identifier("test2".to_owned()),
            Token::Colon,
            Token::Primitive(Primitives::Number),
            Token::ClosingParan,
            Token::OpenCurly,
        ];

        let expected = vec![
            IRParameter {
                name: "test1".to_owned(),
                param_type: IRType::Number,
            },
            IRParameter {
                name: "test2".to_owned(),
                param_type: IRType::Number,
            },
        ];

        assert_eq!(expected, parse(&mut tokens.iter().peekable()));
    }
}
