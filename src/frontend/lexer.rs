mod tokenize;
pub use tokenize::tokenize;

#[derive(Debug, PartialEq, Clone)]
pub enum Primitives {
    Number,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BuiltIns {
    Print,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Comparisons {
    Equal,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Identifier(String),
    ValueNumber(u64),
    Assignment,
    Plus,
    Minus,
    Multiply,
    Semicolon,
    Colon,
    Comma,
    OpenParan,
    ClosingParan,
    Primitive(Primitives),
    Builtin(BuiltIns),
    If,
    OpenCurly,
    ClosingCurly,
    Comparison(Comparisons),
    Function,
    Return,
    Arrow,
}
