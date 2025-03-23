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
    FunctionCall(u8),
    Return,

    // Stack operations
    PushInteger(i64),
    PushFloat(f64),
    PushString(String),
    PushBoolean(bool),
    PushIdentifier(String),
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

        let mut symbols = HashMap::new();
        symbols.insert(String::from("main"), Symbol::UserDefinedFunction {
            address: 0,
            arity: 0
        });

        Program {
            symbols,
            instructions: Vec::new()
        }
    }
}

#[derive(Debug, Clone)]
pub enum Symbol {
    NativeFunction {
        arity: u8
    },
    UserDefinedFunction {
        address: usize,
        arity: u8
    }
}