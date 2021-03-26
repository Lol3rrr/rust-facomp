use super::*;
use crate::frontend::lexer::Token;
use std::iter::Peekable;

fn parse_single(token: &Token) -> Option<IRExpression> {
    match token {
        Token::ValueNumber(value) => Some(IRExpression::Value(IRValue::Number(*value))),
        Token::Identifier(value) => Some(IRExpression::Variable(value.clone())),
        _ => None,
    }
}

pub fn parse_expression<'a, I>(iter: &mut Peekable<I>) -> Option<IRExpression>
where
    I: Iterator<Item = &'a Token>,
{
    let first = iter.next()?;
    let second = iter.peek()?;

    match second {
        Token::Plus | Token::Minus | Token::Multiply => {
            let op = match second {
                Token::Plus => IROperation::Add,
                Token::Minus => IROperation::Sub,
                Token::Multiply => IROperation::Multiply,
                _ => return None,
            };

            // Advance the iterator
            iter.next().unwrap();

            let first_part = match parse_single(first) {
                Some(f) => f,
                None => return None,
            };

            let other_part = match parse_expression(iter) {
                Some(o) => o,
                None => return None,
            };

            return Some(IRExpression::Operation(op, vec![first_part, other_part]));
        }
        Token::Comparison(_) => {
            return parse_single(first);
        }
        Token::Semicolon | Token::ClosingParan | Token::Comma => {
            return parse_single(first);
        }
        Token::OpenParan => {
            let name = match first {
                Token::Identifier(ref name) => name.clone(),
                _ => return None,
            };

            let inner = parse_passed_args::parse(iter)?;

            return Some(IRExpression::Call(name.to_owned(), inner));
        }
        _ => {
            log::error!("Unexpected: {:?}", second);
        }
    };

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_value() {
        let tokens = vec![Token::ValueNumber(1), Token::Semicolon];
        let expected = IRExpression::Value(IRValue::Number(1));

        assert_eq!(
            Some(expected),
            parse_expression(&mut tokens.iter().peekable())
        );
    }
    #[test]
    fn simple_variable() {
        let tokens = vec![Token::Identifier("test".to_owned()), Token::Semicolon];
        let expected = IRExpression::Variable("test".to_owned());

        assert_eq!(
            Some(expected),
            parse_expression(&mut tokens.iter().peekable())
        );
    }
    #[test]
    fn simple_expression() {
        let tokens = vec![
            Token::ValueNumber(1),
            Token::Plus,
            Token::ValueNumber(1),
            Token::Semicolon,
        ];
        let expected = IRExpression::Operation(
            IROperation::Add,
            vec![
                IRExpression::Value(IRValue::Number(1)),
                IRExpression::Value(IRValue::Number(1)),
            ],
        );

        assert_eq!(
            Some(expected),
            parse_expression(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn complex_expression() {
        let tokens = vec![
            Token::ValueNumber(1),
            Token::Plus,
            Token::ValueNumber(1),
            Token::Plus,
            Token::ValueNumber(1),
            Token::Semicolon,
        ];
        let expected = IRExpression::Operation(
            IROperation::Add,
            vec![
                IRExpression::Value(IRValue::Number(1)),
                IRExpression::Operation(
                    IROperation::Add,
                    vec![
                        IRExpression::Value(IRValue::Number(1)),
                        IRExpression::Value(IRValue::Number(1)),
                    ],
                ),
            ],
        );

        assert_eq!(
            Some(expected),
            parse_expression(&mut tokens.iter().peekable())
        );
    }

    #[test]
    fn parse_call_no_param() {
        let tokens = vec![
            Token::Identifier("test_func".to_owned()),
            Token::OpenParan,
            Token::ClosingParan,
            Token::Semicolon,
        ];

        let expected = IRExpression::Call("test_func".to_owned(), vec![]);

        assert_eq!(
            Some(expected),
            parse_expression(&mut tokens.iter().peekable())
        );
    }
    #[test]
    fn parse_call_one_param() {
        let tokens = vec![
            Token::Identifier("test_func".to_owned()),
            Token::OpenParan,
            Token::ValueNumber(1),
            Token::ClosingParan,
            Token::Semicolon,
        ];

        let expected = IRExpression::Call(
            "test_func".to_owned(),
            vec![IRExpression::Value(IRValue::Number(1))],
        );

        assert_eq!(
            Some(expected),
            parse_expression(&mut tokens.iter().peekable())
        );
    }
    #[test]
    fn parse_call_two_params() {
        let tokens = vec![
            Token::Identifier("test_func".to_owned()),
            Token::OpenParan,
            Token::ValueNumber(1),
            Token::Comma,
            Token::ValueNumber(2),
            Token::ClosingParan,
            Token::Semicolon,
        ];

        let expected = IRExpression::Call(
            "test_func".to_owned(),
            vec![
                IRExpression::Value(IRValue::Number(1)),
                IRExpression::Value(IRValue::Number(2)),
            ],
        );

        assert_eq!(
            Some(expected),
            parse_expression(&mut tokens.iter().peekable())
        );
    }
}
