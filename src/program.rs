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

    // Dictionaries
    CreateDictionary(usize),
    GetDictionaryItem,
    SetDictionaryItem,

    // Functions
    FunctionCall(String),
    Return,

    // Stack operations
    PushInteger(i64),
    PushFloat(f64),
    PushString(String),
    PushBoolean(bool),
    PushNull,

    // Arithmetic
    Pop,
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
    pub symbols: HashMap<String, Symbol>,
    pub instructions: Vec<Instruction>
}

impl Default for Program {
    fn default() -> Self {
        Program {
            symbols: HashMap::new(),
            instructions: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    NativeFunction {
        arity: usize
    },
    UserDefinedFunction {
        address: usize,
        arity: usize
    }
}