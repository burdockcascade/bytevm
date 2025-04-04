use crate::variant::Variant;
use std::collections::HashMap;

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

#[derive(Debug, Clone)]
pub enum GlobalEntry {
    NativeFunction {
        arity: usize
    },
    UserDefinedFunction {
        index: usize,
        arity: usize
    }
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub arity: usize,
    pub instructions: Vec<Instruction>
}

#[derive(Debug, Clone)]
pub struct Program {
    pub globals: HashMap<String, GlobalEntry>,
    pub functions: Vec<Function>
}

impl Default for Program {
    fn default() -> Self {
        Program {
            globals: Default::default(),
            functions: Default::default(),
        }
    }
}

impl Program {
    
    pub fn add_function(&mut self, name: String, arity: usize, instructions: Vec<Instruction>) {
        self.globals.insert(name.clone(), GlobalEntry::UserDefinedFunction {
            index: self.functions.len(),
            arity
        });
        self.functions.push(Function {
            name,
            arity,
            instructions
        });
    }
    
}