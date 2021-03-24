use super::{BuiltIns, Comparisons, Primitives, Token};

fn parse(part: &str) -> Option<Token> {
    match part {
        "=" => Some(Token::Assignment),
        "+" => Some(Token::Plus),
        "-" => Some(Token::Minus),
        "*" => Some(Token::Multiply),
        "number" => Some(Token::Primitive(Primitives::Number)),
        "print" => Some(Token::Builtin(BuiltIns::Print)),
        "if" => Some(Token::If),
        "==" => Some(Token::Comparison(Comparisons::Equal)),
        _ if part.len() > 0 => {
            if let Ok(v) = part.parse() {
                return Some(Token::ValueNumber(v));
            }

            Some(Token::Identifier(part.to_owned()))
        }
        _ => None,
    }
}

pub fn tokenize(content: String) -> Vec<Token> {
    let mut result = Vec::new();

    let mut searching = &content[..];
    while let Some(index) = searching.find(&[' ', '\n', ';', '(', ')', '{', '}'][..]) {
        let raw_part = &searching[..index];
        let part = raw_part.trim_start();
        if let Some(t) = parse(part) {
            result.push(t);
        }
        match searching.get(index..index + 1).unwrap() {
            ";" => {
                result.push(Token::Semicolon);
            }
            "(" => {
                result.push(Token::OpenParan);
            }
            ")" => {
                result.push(Token::ClosingParan);
            }
            "{" => {
                result.push(Token::OpenCurly);
            }
            "}" => {
                result.push(Token::ClosingCurly);
            }
            _ => {}
        };

        searching = &searching[index + 1..];
    }

    result
}
