use crate::variant::Variant;
use std::collections::HashMap;
use crate::builder::ProgramBuilder;

#[derive(Clone, Debug, PartialEq)]
pub enum Instruction {

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
    GetDictionaryKeys,

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

#[derive(Clone, Debug, PartialEq)]
pub enum SymbolEntry {
    NativeFunction {
        arity: usize
    },
    UserDefinedFunction {
        index: usize,
        arity: usize
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub instructions: Vec<Instruction>
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Program {
    pub symbol_table: HashMap<String, SymbolEntry>,
    pub functions: Vec<Function>
}

impl Program {

    pub fn builder() -> ProgramBuilder {
        ProgramBuilder::default()
    }
    
}