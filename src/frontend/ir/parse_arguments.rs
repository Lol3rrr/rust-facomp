use std::iter::Peekable;

use crate::frontend::lexer::Token;

// TODO
pub fn parse<'a, I>(iter: &mut Peekable<I>)
where
    I: Iterator<Item = &'a Token>,
{
}
