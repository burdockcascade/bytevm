use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {

    Assert,

    // Variables
    SetLocal(usize),
    GetLocal(usize),

    CreateArray(usize),
    GetArrayItem,
    SetArrayItem,

    CreateDictionary(usize),
    GetDictionaryItem,
    SetDictionaryItem,

    FunctionCall(String),

    // Stack operations
    PushInteger(i64),
    PushFloat(f64),
    PushString(String),
    PushBoolean(bool),
    PushNull,

    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Equals,
    Not,
    Return,
    Negate,
    LessThan,
    LessEqual,
    GreaterThan,
    GreaterEqual,
    NotEqual,
    Or,
    And,

    Jump(usize),
    JumpIfFalse(usize),

    Halt,
    Panic(String)
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