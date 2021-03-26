use facompiler::{
    backend::{self, traits::Arch},
    frontend, optimizer,
};

use env_logger;

fn main() {
    env_logger::init();

    let file_path = "./examples/hello_world.cl";

    let content = std::fs::read_to_string(file_path).unwrap();

    // Parse the content to IR
    let mut ir = frontend::parse(content);

    // Optimize it
    optimizer::optimize(&mut ir);

    // Generate Assembly from the IR
    let asm = backend::archs::x86_64::X86_64::generate_asm_string(ir);

    std::fs::write("./test.asm", asm).expect("Unable to write asm file");
}
