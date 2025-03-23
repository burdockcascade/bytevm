use crate::program::{Instruction, Program, Symbol};
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

pub struct VmOptions {
    pub stack_size: usize,
}

impl Default for VmOptions {
    fn default() -> Self {
        VmOptions {
            stack_size: 16,
        }
    }
}

pub struct Vm {
    pub program: Program,
    native_functions: HashMap<String, fn(Vec<Variant>) -> Option<Variant>>,
    pub stack: Vec<StackFrame>,
    options: VmOptions,
    pub pc: usize,
}

impl Vm {

    pub fn new(program: Program, vm_options: VmOptions) -> Vm {
        Vm {
            program,
            native_functions: HashMap::new(),
            stack: Vec::with_capacity(vm_options.stack_size),
            options: vm_options,
            pc: 0,
        }
    }

    pub fn register_native_function(&mut self, name: String, function: fn(Vec<Variant>) -> Option<Variant>) {
        self.native_functions.insert(name, function);
    }

    pub fn run(mut self, entry_point: Option<String>) -> Result<VmExecutionResult, VmError> {

        let start = std::time::Instant::now();

        if self.program.instructions.is_empty() {
            return Err(VmError::RuntimeWarning {
                message: "No instructions found".to_string()
            });
        }

        trace!("Executing instructions: {:?}", self.program.instructions);
        trace!("Symbols: {:?}", self.program.symbols);

        self.pc = match entry_point {
            Some(label) => match self.program.symbols.get(&label) {
                Some(symbol) => match symbol {
                    Symbol::UserDefinedFunction { address, .. } => *address,
                    _ => return Err(VmError::RuntimeError {
                        message: format!("Entry point is not a function: {}", label)
                    })
                },
                None => return Err(VmError::RuntimeError {
                    message: format!("Entry point not found: {}", label)
                })
            },
            None => {
                debug!("No entry point specified, using default 0");
                0
            }
        };

        let mut frame = StackFrame::new(0);

        loop {

            let Some(instruction) = &self.program.instructions.get(self.pc) else {
                return Err(VmError::RuntimeError {
                    message: "Invalid program counter".to_string()
                });
            };

            trace!("=== [PC: {} / {:?}] ===", self.pc, instruction);
            trace!("Frame locals -> {:?}", frame.locals);
            trace!("Stack operands -> {:?}", frame.operands);

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

                Instruction::PushInteger(value) => {
                    frame.push_operand(Variant::Integer(*value));
                    self.pc += 1;
                },

                Instruction::PushFloat(value) => {
                    frame.push_operand(Variant::Float(*value));
                    self.pc += 1;
                },

                Instruction::PushBoolean(value) => {
                    frame.push_operand(Variant::Boolean(*value));
                    self.pc += 1;
                },

                Instruction::PushString(value) => {
                    frame.push_operand(Variant::String(value.clone()));
                    self.pc += 1;
                },

                Instruction::PushIdentifier(value) => {
                    frame.push_operand(Variant::String(value.clone()));
                    self.pc += 1;
                },

                Instruction::PushNull => {
                    frame.push_operand(Variant::Null);
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
                                    message: "Key not found".to_string()
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

                // Function calls

                Instruction::FunctionCall(arg_count) => {

                    // Get the function name from the stack
                    let name = match frame.pop_operand() {
                        Variant::String(name) => name,
                        _ => return Err(VmError::RuntimeError {
                            message: "Function name must be a string".to_string()
                        })
                    };

                    match self.program.symbols.get(&name) {
                        Some(func) => {
                            match func {
                                Symbol::NativeFunction { arity } => {

                                    // Get the function arguments
                                    let mut args = Vec::with_capacity(*arity as usize);
                                    for _ in 0..*arity {
                                        args.push(frame.pop_operand());
                                    }

                                    // pad with nulls if the function expects more arguments
                                    for _ in 0..(*arity - *arg_count) {
                                        args.push(Variant::Null);
                                    }

                                    args.reverse();

                                    let function = self.native_functions.get(&name).unwrap();
                                    let result = function(args);
                                    if let Some(result) = result {
                                        frame.push_operand(result);
                                    }
                                    self.pc += 1;
                                },
                                Symbol::UserDefinedFunction { address, arity } => {
                                    let pc = self.pc + 1;
                                    let mut new_frame = StackFrame::new(frame.id + 1);
                                    new_frame.return_address = Some(pc);

                                    // Set the function arguments
                                    for _ in 0..*arg_count {
                                        let value = frame.pop_operand();
                                        new_frame.push_operand(value);
                                    }

                                    // pad with nulls if the function expects more arguments
                                    for _ in 0..(*arity - *arg_count) {
                                        new_frame.push_operand(Variant::Null);
                                    }

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
                    self.pc = *address;
                },

                Instruction::JumpIfFalse(address) => {
                    let value: bool = frame.pop_operand().into();
                    if !value {
                        self.pc = *address;
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

            trace!("Frame locals <- {:?}", frame.locals);
            trace!("Frame operands <- {:?}", frame.operands);

        }

        debug!("Program halted");

        Ok(VmExecutionResult {
            result: match frame.operands.pop() {
                Some(result) => Some(result),
                None => None
            },
            run_time: start.elapsed().as_nanos()
        })

    }
}

