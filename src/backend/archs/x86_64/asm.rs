#[derive(Debug, PartialEq)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    /// Stack-Pointer
    RSP,
    /// Stack-Base-Pointer
    RBP,
    RSI,
    RDI,
}

impl Register {
    pub fn to_string(&self) -> String {
        match self {
            Register::RAX => "rax".to_string(),
            Register::RBX => "rbx".to_string(),
            Register::RCX => "rcx".to_string(),
            Register::RDX => "rdx".to_string(),
            Register::RSP => "rsp".to_string(),
            Register::RBP => "rbp".to_string(),
            Register::RSI => "rsi".to_string(),
            Register::RDI => "rdi".to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Comment(String),
    Section(String),
    Label(String),
    Add(String, String),
    Sub(String, String),
    Xor(String, String),
    Move(String, String),
    Return,
    Push(String),
    Pop(String),
    Call(String),
    Cmp(String, String),
    Je(String),
    Jne(String),
    Jle(String),
    Jg(String),
    Syscall,
    Int(String),
    Lea(String, String),
    Raw(String),
}

fn format_asm(instr: &Instruction) -> String {
    match instr {
        Instruction::Comment(a1) => format!("    ; {}", a1),
        Instruction::Section(a1) => format!("section {}", a1),
        Instruction::Label(a1) => format!("  {}:", a1),
        Instruction::Add(a1, a2) => format!("    add {}, {}", a1, a2),
        Instruction::Sub(a1, a2) => format!("    sub {}, {}", a1, a2),
        Instruction::Xor(a1, a2) => format!("    xor {}, {}", a1, a2),
        Instruction::Move(a1, a2) => format!("    mov {}, {}", a1, a2),
        Instruction::Return => format!("    ret"),
        Instruction::Push(a1) => format!("    push {}", a1),
        Instruction::Pop(a1) => format!("    pop {}", a1),
        Instruction::Call(a1) => format!("    call {}", a1),
        Instruction::Cmp(a1, a2) => format!("    cmp {}, {}", a1, a2),
        Instruction::Je(a1) => format!("    je {}", a1),
        Instruction::Jne(a1) => format!("    jne {}", a1),
        Instruction::Jle(a1) => format!("    jle {}", a1),
        Instruction::Jg(a1) => format!("    jg {}", a1),
        Instruction::Syscall => format!("    syscall"),
        Instruction::Int(a1) => format!("    int {}", a1),
        Instruction::Lea(a1, a2) => format!("    lea {},{}", a1, a2),
        Instruction::Raw(a1) => format!("{}", a1),
    }
}
pub fn pretty_print(instructions: &[Instruction]) {
    for instr in instructions.iter() {
        println!("{}", format_asm(instr));
    }
}

pub fn format(instructions: &[Instruction]) -> String {
    let mut result = String::new();

    for instr in instructions.iter() {
        result.push_str(&format_asm(instr));
        result.push('\n');
    }

    result
}
