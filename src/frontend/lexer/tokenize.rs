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
        "while" => Some(Token::While),
        "==" => Some(Token::Comparison(Comparisons::Equal)),
        ">" => Some(Token::Comparison(Comparisons::GreaterThan)),
        "func" => Some(Token::Function),
        "->" => Some(Token::Arrow),
        "return" => Some(Token::Return),
        _ if part.len() > 0 => {
            if let Ok(v) = part.parse() {
                return Some(Token::ValueNumber(v));
            }

            Some(Token::Identifier(part.to_owned()))
        }
        _ => None,
    }
}

fn parse_seperator(chars: &str) -> Option<Token> {
    match chars {
        ";" => Some(Token::Semicolon),
        ":" => Some(Token::Colon),
        "," => Some(Token::Comma),
        "(" => Some(Token::OpenParan),
        ")" => Some(Token::ClosingParan),
        "{" => Some(Token::OpenCurly),
        "}" => Some(Token::ClosingCurly),
        _ => None,
    }
}

pub fn tokenize(content: String) -> Vec<Token> {
    let mut result = Vec::new();

    let seperators = &[' ', '\n', ':', ';', ',', '(', ')', '{', '}'];

    let mut searching = &content[..];
    while let Some(index) = searching.find(&seperators[..]) {
        let raw_part = &searching[..index];
        let part = raw_part.trim_start();
        if let Some(t) = parse(part) {
            result.push(t);
        }
        if let Some(t) = parse_seperator(searching.get(index..index + 1).unwrap()) {
            result.push(t);
        }

        searching = &searching[index + 1..];
    }

    result
}
