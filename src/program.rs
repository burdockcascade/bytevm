use std::collections::HashMap;
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
    GetArrayLength,

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

    // Output
    Print,

    // End of program
    Halt,
    Panic
}

#[derive(Debug, Clone)]
pub struct Program {
    pub globals: HashMap<String, GlobalEntry>,
    pub instructions: Vec<Instruction>
}

impl Default for Program {
    fn default() -> Self {

        let mut symbols = HashMap::new();
        symbols.insert(String::from("main"), GlobalEntry::UserDefinedFunction {
            address: 0,
            arity: 0
        });

        Program {
            globals: symbols,
            instructions: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub enum GlobalEntry {
    NativeFunction {
        arity: usize
    },
    UserDefinedFunction {
        address: usize,
        arity: usize
    }
}