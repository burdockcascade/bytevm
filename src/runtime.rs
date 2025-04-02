use crate::program::{Function, GlobalEntry, Instruction, Program};
use crate::stack::StackFrame;
use crate::variant::Variant;
use log::{debug, trace};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct VmExecutionResult {
    pub result: Option<Variant>,
    pub run_time: u128,
}

#[derive(Debug)]
pub enum VmError {
    RuntimeError {
        message: String,
    },
    RuntimeWarning {
        message: String,
    }
}

pub struct Vm {
    functions: Vec<Function>,
    globals: HashMap<String, GlobalEntry>,
    native_functions: HashMap<String, fn(Vec<Variant>) -> Option<Variant>>
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            functions: Default::default(),
            globals: HashMap::new(),
            native_functions: HashMap::new()
        }
    }
}

impl Vm {

    pub fn register_native_function(&mut self, name: String, function: fn(Vec<Variant>) -> Option<Variant>) {
        self.native_functions.insert(name.clone(), function);
        self.globals.insert(name.clone(), GlobalEntry::NativeFunction {
            arity: 0
        });
    }
    
    pub fn load_program(&mut self, program: Program) {

        debug!("Loaded program");
        trace!("Globals: {:?}", program.globals);
        trace!("Functions: {:?}", program.functions);

        self.functions.extend(program.functions);
        self.globals.extend(program.globals.into_iter());
    }

    pub fn run(&mut self, entry_point: Option<String>) -> Result<VmExecutionResult, VmError> {

        let function_index = match entry_point {
            Some(label) => match self.globals.get(&label) {
                Some(symbol) => match symbol {
                    GlobalEntry::UserDefinedFunction { index: address, .. } => *address,
                    _ => return Err(VmError::RuntimeError {
                        message: format!("Entry point is not a function: {}", label)
                    })
                },
                None => return Err(VmError::RuntimeError {
                    message: format!("Entry point not found: {}", label)
                })
            },
            None => match self.globals.get("main") {
                Some(symbol) => match symbol {
                    GlobalEntry::UserDefinedFunction { index: address, .. } => *address,
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
        
        let mut pc = 0;
        let mut frame = StackFrame {
            locals: parameters,
            operands: Vec::with_capacity(8),
        };

        let start = std::time::Instant::now();

        while let Some(instruction) = f.instructions.get(pc) {
            
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

                    // Get the function name from the stack
                    let name = match frame.pop_operand() {
                        Variant::Identifier(name) => name,
                        _ => return Err(VmError::RuntimeError {
                            message: "Function name must be a string".to_string()
                        })
                    };

                    match self.globals.get(&name) {
                        Some(func) => {
                            match func {
                                GlobalEntry::NativeFunction { .. } => {
                                    let function = self.native_functions.get(&name).unwrap();
                                    let result = function(args);
                                    if let Some(result) = result {
                                        frame.push_operand(result);
                                    }

                                },
                                GlobalEntry::UserDefinedFunction { index: address, .. } => {
                                    match self.functions.get(*address) {
                                        Some(func) => {
                                            match self.execute(func, args) {
                                                Ok(result) => {
                                                    match result.result {
                                                        Some(value) => frame.push_operand(value),
                                                        None => {}
                                                    }
                                                },
                                                Err(err) => {
                                                    return Err(err);
                                                }
                                            }
                                        }
                                        None => return Err(VmError::RuntimeError {
                                            message: format!("Function not found: {}", name)
                                        })
                                    }
                                }
                            }
                        },
                        None => return Err(VmError::RuntimeError {
                            message: format!("Function not found: {}", name)
                        })
                    }

                    pc += 1;
                },

                Instruction::Return => {
                    trace!("=== Returning from function: {} ====", f.name);
                    return Ok(VmExecutionResult {
                        result: frame.operands.pop(),
                        run_time: start.elapsed().as_nanos()
                    });
                }

                // Assert

                Instruction::Assert => {
                    let value: bool = frame.pop_operand().into();
                    if !value {
                        return Err(VmError::RuntimeError {
                            message: "Assertion failed".to_string()
                        });
                    }
                    pc += 1;
                },

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

                // Comparison instructions

                Instruction::Equal => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b == a));
                    pc += 1;
                },

                Instruction::Add => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b + a);
                    pc += 1;
                },

                Instruction::Sub => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b - a);
                    pc += 1;
                },

                Instruction::Mul => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b * a);
                    pc += 1;
                },

                Instruction::Div => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b / a);
                    pc += 1;
                },

                Instruction::Not => {
                    let a = frame.pop_operand();
                    frame.push_operand(!a);
                    pc += 1;
                },

                Instruction::GreaterThan => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b > a));
                    pc += 1;
                }

                Instruction::LessThan => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b < a));
                    pc += 1;
                },

                Instruction::LessEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b <= a));
                    pc += 1;
                },

                Instruction::GreaterEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b >= a));
                    pc += 1;
                },

                Instruction::NotEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b != a));
                    pc += 1;
                },

                Instruction::Or => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    let result: bool = b.into() || a.into();
                    frame.push_operand(Variant::Boolean(result));
                    pc += 1;
                },

                Instruction::And => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    let result: bool = b.into() && a.into();
                    frame.push_operand(Variant::Boolean(result));
                    pc += 1;
                },

                Instruction::Mod => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b % a);
                    pc += 1;
                },

                Instruction::Pow => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b.pow(&a));
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
            result: None,
            run_time: start.elapsed().as_nanos()
        })

    }
}

