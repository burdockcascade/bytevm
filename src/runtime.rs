use crate::program::{Function, SymbolEntry, Instruction, Program};
use crate::variant::Variant;
use log::{debug, trace};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Default, Debug, PartialEq)]
pub struct VmExecutionResult {
    pub result: Option<Variant>,
    pub run_time: Duration,
}

#[derive(Clone, Debug, PartialEq)]
pub enum VmError {
    RuntimeError {
        message: String,
    },
    RuntimeWarning {
        message: String,
    }
}


#[derive(Clone, Default, Debug, PartialEq)]
struct StackFrame {
    locals: Vec<Variant>,
    operands: Vec<Variant>
}

impl StackFrame {

    fn pop_operand(&mut self) -> Variant {
        self.operands.pop().expect("Operand stack should not be empty")
    }

    fn push_operand(&mut self, operand: Variant) {
        self.operands.push(operand);
    }

    fn get_local(&self, index: usize) -> Variant {
        self.locals.get(index).cloned().unwrap_or(Variant::Null)
    }

    fn set_local(&mut self, index: usize, value: Variant) {
        if index >= self.locals.len() {
            self.locals.resize(index + 1, Variant::Null);
        }
        self.locals[index] = value;
    }

}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Vm {
    functions: Vec<Function>,
    symbols: HashMap<String, SymbolEntry>,
    native_functions: HashMap<String, fn(Vec<Variant>) -> Option<Variant>>
}


impl Vm {

    pub fn register_native_function(&mut self, name: String, function: fn(Vec<Variant>) -> Option<Variant>) {
        self.native_functions.insert(name.clone(), function);
        self.symbols.insert(name.clone(), SymbolEntry::NativeFunction {
            arity: 0
        });
    }
    
    pub fn load_program(&mut self, program: Program) {

        debug!("Loaded program");
        trace!("Globals: {:?}", program.symbol_table);
        trace!("Functions: {:?}", program.functions);

        self.functions.extend(program.functions);
        self.symbols.extend(program.symbol_table.into_iter());
    }

    pub fn run(&mut self, entry_point: Option<String>) -> Result<VmExecutionResult, VmError> {

        let function_index = match entry_point {
            Some(label) => match self.symbols.get(&label) {
                Some(symbol) => match symbol {
                    SymbolEntry::UserDefinedFunction { index: address, .. } => *address,
                    _ => return Err(VmError::RuntimeError {
                        message: format!("Entry point is not a function: {}", label)
                    })
                },
                None => return Err(VmError::RuntimeError {
                    message: format!("Entry point not found: {}", label)
                })
            },
            None => match self.symbols.get("main") {
                Some(symbol) => match symbol {
                    SymbolEntry::UserDefinedFunction { index: address, .. } => *address,
                    _ => return Err(VmError::RuntimeError {
                        message: "Main function not found".to_string()
                    })
                },
                None => return Err(VmError::RuntimeError {
                    message: "Main function not found".to_string()
                })
            }
        };

        let result = match self.functions.get(function_index) {
            Some(f) => {
                self.execute(&f, vec![])
            },
            None => Err(VmError::RuntimeError {
                message: format!("Function not found: {}", function_index)
            })
        };

        result
    }

    // Executes a function with the given parameters.
    fn execute(&self, f: &Function, parameters: Vec<Variant>) -> Result<VmExecutionResult, VmError> {
        
        trace!("=== Executing function: {} ====", f.name);
        trace!("Parameters: {:?}", parameters);
        trace!("Instructions: {:?}", f.instructions);

        let mut return_value= None;
        let mut pc = 0;
        let mut frame = StackFrame {
            locals: parameters,
            operands: Vec::with_capacity(8),
        };

        let start = std::time::Instant::now();

        loop {

            let Some(instruction) = f.instructions.get(pc) else {
                return Err(VmError::RuntimeError {
                    message: format!("Invalid instruction pointer {} in function {}", pc, f.name)
                })
            };
            
            trace!("Program Counter: {}", pc);
            trace!("Executing instruction: {:?}", instruction);
            trace!("Frame Locals: {:?}", frame.locals);
            trace!("Frame Operands: {:?}", frame.operands);

            match instruction {

                // Function calls

                Instruction::FunctionCall(arg_count) => {

                    let mut args = Vec::with_capacity(*arg_count);
                    for _ in 0..*arg_count {
                        args.push(frame.pop_operand());
                    }
                    args.reverse();

                    match frame.pop_operand() {
                        Variant::SymbolReference(name) => {
                            match self.symbols.get(name.as_str()) {
                                Some(SymbolEntry::UserDefinedFunction { index, .. }) => {
                                    match self.functions.get(*index) {
                                        Some(f) => {
                                            if let Some(result) = self.execute(f, args)?.result {
                                                frame.push_operand(result);
                                            }
                                        },
                                        None => return Err(VmError::RuntimeError {
                                            message: format!("Function not found: {}", index)
                                        })
                                    };
                                },
                                Some(SymbolEntry::NativeFunction { .. }) => {
                                    let func = match self.native_functions.get(name.as_str()) {
                                        Some(func) => func,
                                        None => return Err(VmError::RuntimeError {
                                            message: format!("Native function not found: {}", name)
                                        })
                                    };
                                    match func(args) {
                                        Some(value) => frame.push_operand(value),
                                        None => {}
                                    }
                                }
                                None => return Err(VmError::RuntimeError {
                                    message: format!("Function not found: {}", name)
                                })
                            }
                        },
                        Variant::FunctionPointer(address) => {
                            // Check if the function is a user-defined function
                            match self.functions.get(address) {
                                Some(func) => {
                                    if let Some(result) = self.execute(func, args)?.result {
                                        frame.push_operand(result);
                                    }
                                }
                                None => return Err(VmError::RuntimeError {
                                    message: format!("Function not found: {}", address)
                                })
                            }
                        }
                        _ => return Err(VmError::RuntimeError {
                            message: format!("Function call must either be to a global function or a function pointer, but got {:?}", instruction)
                        })
                    }

                    pc += 1;
                },

                Instruction::Return => {
                    return_value = frame.operands.pop();
                    break
                }

                // Operands

                Instruction::Push(value) => {
                    frame.push_operand(value.clone());
                    pc += 1;
                },

                Instruction::Pop => {
                    frame.pop_operand();
                    pc += 1;
                },

                // Local variables

                Instruction::SetLocal(index) => {
                    let value = frame.pop_operand();
                    frame.set_local(*index, value);
                    pc += 1;
                },

                Instruction::GetLocal(index) => {
                    let value = frame.get_local(*index);
                    frame.push_operand(value);
                    pc += 1;
                },

                // Arrays

                Instruction::CreateArray(size) => {
                    let mut array = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        array.push(frame.pop_operand());
                    }
                    array.reverse();
                    frame.push_operand(Variant::Array(Rc::new(RefCell::new(array))));
                    pc += 1;
                },

                Instruction::GetArrayItem => {
                    let index = frame.pop_operand();
                    let array = frame.pop_operand();
                    let value = match array {
                        Variant::Array(array) => {
                            let array = array.borrow();
                            let index: usize = index.into();
                            match array.get(index) {
                                Some(value) => value.clone(),
                                None => return Err(VmError::RuntimeError {
                                    message: "Index out of bounds".to_string()
                                })
                            }
                        },
                        _ => return Err(VmError::RuntimeError {
                            message: format!("Expected an array but got {:?}", array)
                        })
                    };
                    frame.push_operand(value);
                    pc += 1;
                }

                Instruction::SetArrayItem => {
                    let value = frame.pop_operand();
                    let index = frame.pop_operand();
                    let array = frame.pop_operand();
                    match array {
                        Variant::Array(array) => {
                            let mut array = array.borrow_mut();
                            let index: usize = index.into();
                            array[index] = value;
                        },
                        _ => return Err(VmError::RuntimeError {
                            message: format!("Expected an array but got {:?}", array)
                        })
                    }
                    pc += 1;
                },

                Instruction::GetArrayLength => {
                    let array = frame.pop_operand();
                    let length = match array {
                        Variant::Array(array) => {
                            let array = array.borrow();
                            array.len()
                        },
                        _ => return Err(VmError::RuntimeError {
                            message: format!("Expected an array but got {:?}", array)
                        })
                    };
                    frame.push_operand(Variant::Integer(length as i64));
                    pc += 1;
                },

                // Dictionaries

                Instruction::CreateDictionary(size) => {
                    let mut table = HashMap::new();
                    for _ in 0..*size {
                        let value = frame.pop_operand();
                        let key = frame.pop_operand();
                        table.insert(key, value);
                    }
                    frame.push_operand(Variant::Dictionary(Rc::new(RefCell::new(table))));
                    pc += 1;
                },

                Instruction::GetDictionaryItem => {
                    let key = frame.pop_operand();
                    let table = frame.pop_operand();
                    let value = match table {
                        Variant::Dictionary(table) => {
                            let table = table.borrow();
                            match table.get(&key) {
                                Some(value) => value.clone(),
                                None => return Err(VmError::RuntimeError {
                                    message: format!("Key not found: {:?}", key)
                                })
                            }
                        },
                        _ => return Err(VmError::RuntimeError {
                            message: format!("Expected a dictionary but got {:?}", table)
                        })
                    };
                    frame.push_operand(value);
                    pc += 1;
                }

                Instruction::SetDictionaryItem => {
                    let value = frame.pop_operand();
                    let key = frame.pop_operand();
                    let table = frame.pop_operand();
                    match table {
                        Variant::Dictionary(table) => {
                            let mut table = table.borrow_mut();
                            table.insert(key, value);
                        },
                        _ => return Err(VmError::RuntimeError {
                            message: format!("Expected a dictionary but got {:?}", table)
                        })
                    }
                    pc += 1;
                },

                Instruction::GetDictionaryKeys => {
                    let table = frame.pop_operand();
                    let keys = match table {
                        Variant::Dictionary(table) => {
                            let table = table.borrow();
                            table.keys().cloned().collect::<Vec<Variant>>()
                        },
                        _ => return Err(VmError::RuntimeError {
                            message: format!("Expected a dictionary but got {:?}", table)
                        })
                    };
                    frame.push_operand(Variant::Array(Rc::new(RefCell::new(keys))));
                    pc += 1;
                },

                // Jump instructions

                Instruction::Jump(address) => {
                    pc = *address;
                },

                Instruction::JumpIfFalse(address) => {
                    let value: bool = frame.pop_operand().into();
                    if !value {
                        pc = *address;
                    } else {
                        pc += 1;
                    }
                },

                // Binary Operations

                Instruction::Add => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(a + b);
                    pc += 1;
                },

                Instruction::Sub => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(a - b);
                    pc += 1;
                },

                Instruction::Mul => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(a * b);
                    pc += 1;
                },

                Instruction::Div => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(a / b);
                    pc += 1;
                },

                Instruction::Mod => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(a % b);
                    pc += 1;
                },

                Instruction::Pow => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(a.pow(&b));
                    pc += 1;
                },

                // Unary Operations

                Instruction::Equal => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a == b));
                    pc += 1;
                },

                Instruction::GreaterThan => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a > b));
                    pc += 1;
                }

                Instruction::LessThan => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a < b));
                    pc += 1;
                },

                Instruction::LessEqual => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a <= b));
                    pc += 1;
                },

                Instruction::GreaterEqual => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a >= b));
                    pc += 1;
                },

                Instruction::NotEqual => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a != b));
                    pc += 1;
                },

                Instruction::Or => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a.into() || b.into()));
                    pc += 1;
                },

                Instruction::And => {
                    let b = frame.pop_operand();
                    let a = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(a.into() && b.into()));
                    pc += 1;
                },

                Instruction::Not => {
                    let a = frame.pop_operand();
                    frame.push_operand(!a);
                    pc += 1;
                },

                Instruction::Negate => {
                    let a = frame.pop_operand();
                    frame.push_operand(-a);
                    pc += 1;
                },


                // Output
                Instruction::Print => {
                    let value = frame.pop_operand();
                    println!("{}", value);
                    pc += 1;
                },

                Instruction::Halt => {
                    break;
                },

                Instruction::Panic => {
                    return match frame.pop_operand() {
                        Variant::String(message) => Err(VmError::RuntimeError {
                            message: message.clone()
                        }),
                        _ => Err(VmError::RuntimeError {
                            message: "Panic message must be a string".to_string()
                        })
                    };
                },

            }

        }
        
        trace!("=== Finished executing function: {} ====", f.name);

        Ok(VmExecutionResult {
            result: return_value,
            run_time: start.elapsed()
        })

    }
    
}

