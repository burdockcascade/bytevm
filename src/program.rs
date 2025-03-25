use crate::variant::Variant;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {

    Assert,

    // Variables
    SetLocal(usize),
    GetLocal(usize),

    // Arrays
    CreateArray(usize),
    GetArrayItem,
    SetArrayItem,

    // Dictionaries
    CreateDictionary(usize),
    GetDictionaryItem,
    SetDictionaryItem,

    // Functions
    FunctionCall(usize),
    Return,

    // Stack operations
    Push(Variant),
    Pop,

    // Arithmetic
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,

    // Comparison
    Equal,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    NotEqual,

    // Logical
    Or,
    And,

    // Unary
    Not,
    Negate,

    // Jumps
    Jump(usize),
    JumpIfFalse(usize),

    // End of program
    Halt,
    Panic
}

#[derive(Debug, Clone)]
pub struct Program {
    pub entry_point: usize,
    pub instructions: Vec<Instruction>
}

impl Default for Program {
    fn default() -> Self {
        Program {
            entry_point: 0,
            instructions: Vec::new()
        }
    }
}
