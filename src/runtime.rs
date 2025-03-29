use crate::program::{Instruction, Program, GlobalEntry};
use crate::stack::StackFrame;
use crate::variant::Variant;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use log::{debug, trace};

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
    instructions: Vec<Instruction>,
    globals: HashMap<String, GlobalEntry>,
    native_functions: HashMap<String, fn(Vec<Variant>) -> Option<Variant>>,
    pub stack: Vec<StackFrame>,
    pub pc: usize,
}

impl Default for Vm {
    fn default() -> Self {
        Vm {
            instructions: Vec::new(),
            globals: HashMap::new(),
            native_functions: HashMap::new(),
            stack: Vec::with_capacity(8),
            pc: 0,
        }
    }
}

impl Vm {

    pub fn register_native_function(&mut self, name: String, function: fn(Vec<Variant>) -> Option<Variant>) {
        self.native_functions.insert(name, function);
    }
    
    pub fn load_program(&mut self, program: Program) {

        debug!("Loaded program with {} instructions and {} globals", program.instructions.len(), program.globals.len());
        trace!("Instructions: {:?}", program.instructions);
        trace!("Globals: {:?}", program.globals);

        self.instructions = program.instructions;
        self.globals = program.globals;
    }

    pub fn run(mut self, entry_point: Option<String>) -> Result<VmExecutionResult, VmError> {

        let start = std::time::Instant::now();

        if self.instructions.is_empty() {
            return Err(VmError::RuntimeWarning {
                message: "No instructions found".to_string()
            });
        }

        self.pc = match entry_point {
            Some(label) => match self.globals.get(&label) {
                Some(symbol) => match symbol {
                    GlobalEntry::UserDefinedFunction { address, .. } => *address,
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
                    GlobalEntry::UserDefinedFunction { address, .. } => *address,
                    _ => return Err(VmError::RuntimeError {
                        message: "Main function not found".to_string()
                    })
                },
                None => return Err(VmError::RuntimeError {
                    message: "Main function not found".to_string()
                })
            }
        };

        let mut frame = StackFrame::default();

        loop {

            trace!("[Loop Start]");

            let Some(instruction) = &self.instructions.get(self.pc) else {
                return Err(VmError::RuntimeError {
                    message: "Invalid program counter".to_string()
                });
            };

            trace!("Stack Counter: {:?}", self.stack.len());
            trace!("Program Counter: {}", self.pc);
            trace!("Executing instruction: {:?}", instruction);
            trace!("Frame Locals: {:?}", frame.locals);
            trace!("Frame Operands: {:?}", frame.operands);

            match instruction {

                Instruction::Assert => {
                    let value: bool = frame.pop_operand().into();
                    if !value {
                        return Err(VmError::RuntimeError {
                            message: "Assertion failed".to_string()
                        });
                    }
                    self.pc += 1;
                },

                Instruction::Push(value) => {
                    frame.push_operand(value.clone());
                    self.pc += 1;
                },

                Instruction::Pop => {
                    frame.pop_operand();
                    self.pc += 1;
                },

                // Local variables

                Instruction::SetLocal(index) => {
                    let value = frame.pop_operand();
                    frame.set_local(*index, value);
                    self.pc += 1;
                },

                Instruction::GetLocal(index) => {
                    let value = frame.get_local(*index);
                    frame.push_operand(value);
                    self.pc += 1;
                },

                // Arrays

                Instruction::CreateArray(size) => {
                    let mut array = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        array.push(frame.pop_operand());
                    }
                    array.reverse();
                    frame.push_operand(Variant::Array(Rc::new(RefCell::new(array))));
                    self.pc += 1;
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
                    self.pc += 1;
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
                    self.pc += 1;
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
                    self.pc += 1;
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
                    self.pc += 1;
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
                    self.pc += 1;
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
                    self.pc += 1;
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
                    self.pc += 1;
                },

                // Function calls

                Instruction::FunctionCall(arg_count) => {

                    let args = frame.pop_operands(*arg_count);

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
                                    self.pc += 1;
                                },
                                GlobalEntry::UserDefinedFunction { address, .. } => {
                                    let mut new_frame = StackFrame::default();
                                    new_frame.base_address = *address;
                                    new_frame.return_address = Some(self.pc + 1);
                                    new_frame.locals = args;
                                    self.stack.push(frame);
                                    frame = new_frame;
                                    self.pc = *address;
                                }
                            }
                        },
                        None => return Err(VmError::RuntimeError {
                            message: format!("Function not found: {}", name)
                        })
                    }
                },

                Instruction::Return => {
                    match frame.return_address {
                        Some(address) => {

                            if frame.operands.len() > 1 {
                                return Err(VmError::RuntimeError {
                                    message: "Too many items on the stack".to_string()
                                });
                            }

                            let return_value = if frame.operands.is_empty() {
                                Variant::Null
                            } else {
                                frame.pop_operand()
                            };

                            self.pc = address;
                            frame = self.stack.pop().unwrap();

                            frame.push_operand(return_value);
                        },
                        None => break
                    }
                },

                // Jump instructions

                Instruction::Jump(address) => {
                    self.pc = frame.base_address + *address;
                },

                Instruction::JumpIfFalse(address) => {
                    let value: bool = frame.pop_operand().into();
                    if !value {
                        self.pc = frame.base_address + *address;
                    } else {
                        self.pc += 1;
                    }
                },

                // Comparison instructions

                Instruction::Equal => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b == a));
                    self.pc += 1;
                },

                Instruction::Add => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b + a);
                    self.pc += 1;
                },

                Instruction::Sub => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b - a);
                    self.pc += 1;
                },

                Instruction::Mul => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b * a);
                    self.pc += 1;
                },

                Instruction::Div => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b / a);
                    self.pc += 1;
                },

                Instruction::Not => {
                    let a = frame.pop_operand();
                    frame.push_operand(!a);
                    self.pc += 1;
                },

                Instruction::GreaterThan => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b > a));
                    self.pc += 1;
                }

                Instruction::LessThan => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b < a));
                    self.pc += 1;
                },

                Instruction::LessEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b <= a));
                    self.pc += 1;
                },

                Instruction::GreaterEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b >= a));
                    self.pc += 1;
                },

                Instruction::NotEqual => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(Variant::Boolean(b != a));
                    self.pc += 1;
                },

                Instruction::Or => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    let result: bool = b.into() || a.into();
                    frame.push_operand(Variant::Boolean(result));
                    self.pc += 1;
                },

                Instruction::And => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    let result: bool = b.into() && a.into();
                    frame.push_operand(Variant::Boolean(result));
                    self.pc += 1;
                },

                Instruction::Mod => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b % a);
                    self.pc += 1;
                },

                Instruction::Pow => {
                    let a = frame.pop_operand();
                    let b = frame.pop_operand();
                    frame.push_operand(b.pow(&a));
                    self.pc += 1;
                },

                Instruction::Negate => {
                    let a = frame.pop_operand();
                    frame.push_operand(-a);
                    self.pc += 1;
                },

                // Output
                Instruction::Print => {
                    let value = frame.pop_operand();
                    println!("{}", value);
                    self.pc += 1;
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

        Ok(VmExecutionResult {
            result: match frame.operands.pop() {
                Some(result) => Some(result),
                None => None
            },
            run_time: start.elapsed().as_nanos()
        })

    }
}

