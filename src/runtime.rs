use crate::program::{Function, SymbolEntry, Instruction, Program, CallTarget};
use crate::variant::Variant;
use log::{debug, trace};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Duration;

macro_rules! runtime_error {
    ($($arg:tt)*) => {
        Err(VmError::RuntimeError {
            message: format!($($arg)*)
        })
    };
}

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
        self.locals[index].clone()
    }

    fn set_local(&mut self, index: usize, value: Variant) {
        self.locals[index] = value;
    }

}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Vm {
    functions: Vec<Rc<Function>>,
    symbols: HashMap<String, SymbolEntry>,
    native_functions: HashMap<String, fn(Vec<Variant>) -> Option<Variant>>
}

impl Vm {

    pub fn register_native_function(&mut self, name: String, arity: usize, function: fn(Vec<Variant>) -> Option<Variant>) {
        self.native_functions.insert(name.clone(), function);
        self.symbols.insert(name.clone(), SymbolEntry::NativeFunction {
            arity
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
        
        // use entry point or default to main
        let entry_point = entry_point.unwrap_or_else(|| String::from("main"));

        // Get the function to execute
        let Some(SymbolEntry::UserDefinedFunction { index, .. }) = self.symbols.get(entry_point.as_str()) else {
            return runtime_error!("Entry point not found: {}", entry_point)
        };

        // Get the function
        let Some(f) = self.functions.get(*index).cloned() else {
            return runtime_error!("Function not found: {}", index)
        };

        // Execute the function
        self.execute(f.clone(), vec![])

    }

    // Executes a function with the given parameters.
    fn execute(&self, f: Rc<Function>, parameters: Vec<Variant>) -> Result<VmExecutionResult, VmError> {
        
        trace!("=== Executing function: {} ====", f.name);
        trace!("Parameters: {:?}", parameters);
        trace!("Instructions: {:?}", f.instructions);

        let mut return_value= None;
        let mut pc = 0;
        let mut frame = StackFrame {
            locals: parameters,
            operands: Vec::with_capacity(8),
        };

        // Expand the operand stack to hold the local variables
        frame.locals.resize(f.local_count, Variant::Null);

        let start = std::time::Instant::now();

        loop {

            let Some(instruction) = f.instructions.get(pc) else {
                return runtime_error!("Invalid instruction pointer {} in function {}", pc, f.name)
            };
            
            trace!("Program Counter: {}", pc);
            trace!("Executing instruction: {:?}", instruction);
            trace!("Frame Locals: {:?}", frame.locals);
            trace!("Frame Operands: {:?}", frame.operands);

            match instruction {

                // Function calls

                Instruction::FunctionCall(target) => {

                    match target {
                        CallTarget::Name(name) => {
                            match self.symbols.get(name.as_str()) {
                                Some(SymbolEntry::UserDefinedFunction { index, .. }) => {
                                    match self.functions.get(*index) {
                                        Some(f) => {
                                            if let Some(result) = self.execute(f.clone(), get_function_call_args(&mut frame, f.arity))?.result {
                                                frame.push_operand(result);
                                            }
                                        },
                                        None => return runtime_error!("Function not found: {}", index)
                                    };
                                },
                                Some(SymbolEntry::NativeFunction { arity }) => {
                                    let func = match self.native_functions.get(name.as_str()) {
                                        Some(func) => func,
                                        None => return runtime_error!("Native function not found: {}", name)
                                    };
                                    match func(get_function_call_args(&mut frame, *arity)) {
                                        Some(value) => frame.push_operand(value),
                                        None => {}
                                    }
                                }
                                None => return runtime_error!("Native function not found: {}", name)
                            }
                        },
                        CallTarget::Index(index) => {
                            match self.functions.get(*index) {
                                Some(f) => {
                                    if let Some(result) = self.execute(f.clone(), get_function_call_args(&mut frame, f.arity))?.result {
                                        frame.push_operand(result);
                                    }
                                },
                                None => return runtime_error!("Function not found: {}", index)
                            }
                        }
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
                                None => return runtime_error!("Array index out of bounds: {} >= {}", index, array.len())
                            }
                        },
                        _ => return runtime_error!("Expected an array but got {:?}", array)
                    };
                    frame.push_operand(value);
                    pc += 1;
                }

                Instruction::SetArrayItem => {
                    let value = frame.pop_operand();
                    let index = frame.pop_operand();
                    let varray = frame.pop_operand();
                    match varray {
                        Variant::Array(ref array) => {
                            let mut array = array.borrow_mut();
                            let index: usize = index.into();
                            array[index] = value;
                            frame.push_operand(varray.clone());
                        },
                        _ => return runtime_error!("Expected an array but got {:?}", varray)
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
                        _ => return runtime_error!("Expected an array but got {:?}", array)
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
                                None => return runtime_error!("Dictionary key not found: {:?}", key)
                            }
                        },
                        _ => return runtime_error!("Expected an dictionary but got {:?}", table)
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
                        _ => return runtime_error!("Expected an dictionary but got {:?}", table)
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
                        _ => return runtime_error!("Expected an dictionary but got {:?}", table)
                    };
                    frame.push_operand(Variant::Array(Rc::new(RefCell::new(keys))));
                    pc += 1;
                },

                // Jump instructions

                Instruction::Jump(address) => {
                    pc = *address;
                },

                Instruction::JumpIfFalse(address) => {
                    let Variant::Boolean(value) = frame.pop_operand() else {
                        return runtime_error!("Expected a boolean but got {:?}", instruction)
                    };
                    
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
                        _ => runtime_error!("Expected a string but got {:?}", instruction)
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

fn get_function_call_args(frame: &mut StackFrame, arity: usize) -> Vec<Variant> {
    let mut args = Vec::with_capacity(arity);
    for _ in 0..arity {
        args.push(frame.pop_operand());
    }
    args.reverse();
    args
}

