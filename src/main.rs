use facompiler::{backend, frontend};

fn main() {
    let file_path = "./examples/hello_world.cl";

    let content = std::fs::read_to_string(file_path).unwrap();

    // Parse the content to IR
    let ir = frontend::parse(content);

    // Generate Assembly from the IR
    let asm = backend::generate(ir);

    std::fs::write("./test.asm", asm).expect("Unable to write asm file");
}
