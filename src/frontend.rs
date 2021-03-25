use self::ir::IRFunction;

pub mod ir;
mod lexer;

pub fn parse(content: String) -> std::collections::HashMap<String, IRFunction> {
    let tokens = lexer::tokenize(content);

    let ir = ir::parse(&tokens).unwrap();

    ir
}
