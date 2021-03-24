use std::iter::Peekable;

use crate::frontend::{
    ir::{parse_expression, IRComparison, IRNode},
    lexer::{Comparisons, Token},
};

use super::inner_parse;

pub fn parse<'a, I>(iter: &mut Peekable<I>) -> Option<IRNode>
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
