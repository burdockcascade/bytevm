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

macro_rules! stack_pop {
    ($stack:expr) => {
        $stack.pop().expect("Operand stack should not be empty")
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
    function_index: usize,
    pc: usize,
    stack_base_pointer: usize,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Vm {
    functions: Vec<Function>,
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

    /// Executes the program with the given entry point and parameters.
    /// If no entry point is provided, it defaults to "main".
    pub fn run(&mut self, entry_point: Option<String>, _parameters: Option<Vec<Variant>>) -> Result<VmExecutionResult, VmError> {
        
        let timer = std::time::Instant::now();

        // frames
        let frames = &mut Vec::with_capacity(32);

        // stack
        let mut stack = Vec::with_capacity(512);
        
        // use entry point or default to main
        let entry_point = entry_point.unwrap_or_else(|| String::from("main"));

        // Get the function to execute
        let function_index = match self.symbols.get(entry_point.as_str()) {
            Some(SymbolEntry::UserDefinedFunction { index, .. }) => {
                match self.functions.get(*index) {
                    Some(_) => *index,
                    None => return runtime_error!("Function not found: {}", entry_point)
                }
            },
            Some(SymbolEntry::NativeFunction { .. }) => {
                return runtime_error!("Cannot execute native function as entry point: {}", entry_point);
            },
            _ => return runtime_error!("Entry point not found: {}", entry_point)
        };

        // Initialize the function's local variables
        let mut current_function = match self.functions.get(function_index) {
            Some(func) => func,
            None => return runtime_error!("Function not found: {}", function_index)
        };

        // Initialize the stack frame
        let mut current_frame = StackFrame {
            function_index,
            pc: 0,
            stack_base_pointer: 0,
        };
        stack.resize(current_function.local_count, Variant::Null);
        
        debug!("Starting execution of function: {}", current_function.name);
        let mut result = None;

        loop  {

            let Some(instruction) = current_function.instructions.get(current_frame.pc) else {
                return runtime_error!("Program counter out of bounds: {} >= {}", current_frame.pc, current_function.instructions.len());
            };

            // trace!("========================================");
            // trace!("Frame[{}]: Executing instruction[{}]: {:?}", self.frames.len(), current_frame.pc, instruction);
            // trace!("Frame[{}]: Stack: {:?}", self.frames.len(), stack);
            // trace!("Frame[{}]: Base pointer: {}", self.frames.len(), current_frame.stack_base_pointer);
            // trace!("Frame[{}]: Local Count: {}", self.frames.len(), current_function.local_count);
            // trace!("Frame[{}]: Locals: {:?}", self.frames.len(), &stack[current_frame.stack_base_pointer .. current_frame.stack_base_pointer + current_function.local_count]);
            // trace!("Frame[{}]: Operands: {:?}", self.frames.len(), &stack[current_frame.stack_base_pointer + current_function.local_count..]);

            match instruction {

                // Operands

                Instruction::Push(value) => {
                    stack.push(value.clone());
                    current_frame.pc += 1;
                },

                // Local variables

                Instruction::SetLocal(index) => {
                    let value = stack_pop!(stack);
                    let variable_index = current_frame.stack_base_pointer + *index;
                    let stack_len = stack.len();
                    stack.get_mut(variable_index)
                        .expect(format!("Local variable index out of bounds: {} >= {}", variable_index, stack_len).as_str())
                        .clone_from(&value);
                    current_frame.pc += 1;
                },

                Instruction::GetLocal(index) => {
                    let value = stack[current_frame.stack_base_pointer + *index].clone();
                    stack.push(value);
                    current_frame.pc += 1;
                },

                // Jump instructions

                Instruction::Jump(address) => {
                    current_frame.pc = *address;
                },

                Instruction::JumpIfFalse(address) => {
                    let var = stack_pop!(stack);
                    match var {
                        Variant::Boolean(value) => {
                            if !value {
                                current_frame.pc = *address;
                            } else {
                                current_frame.pc += 1;
                            }
                        },
                        v => return runtime_error!("Expected a boolean but got {:?}", v)
                    }
                },


                // Function calls

                Instruction::FunctionCall(target) => {

                    let function_index = match target {
                        CallTarget::Name(name) if self.native_functions.contains_key(name) => {
                            let arity = match self.symbols.get(name.as_str()) {
                                Some(SymbolEntry::NativeFunction { arity }) => *arity,
                                _ => return runtime_error!("Native function not found: {}", name)
                            };
                            let func = match self.native_functions.get(name.as_str()) {
                                Some(func) => func,
                                None => return runtime_error!("Native function not found: {}", name)
                            };
                            if let Some(value) = func(get_function_call_args(&mut stack, arity)) {
                                stack.push(value);
                            }
                            current_frame.pc += 1;
                            continue;
                        }
                        CallTarget::Name(name) => {
                            // User defined function
                            match self.symbols.get(name.as_str()) {
                                Some(SymbolEntry::UserDefinedFunction { index, .. }) => *index,
                                _ => return runtime_error!("Function not found: {}", name)
                            }
                        },
                        CallTarget::Index(index) => *index
                    };

                    match self.functions.get(function_index) {
                        Some(next_function) => {

                            current_frame.pc += 1;
                            frames.push(current_frame);

                            // Update the current function to the next function
                            current_function = next_function;

                            // Get arguments for the function call
                            let args = get_function_call_args(&mut stack, current_function.arity);

                            // Create a new stack frame for the function call
                            current_frame = StackFrame {
                                function_index,
                                pc: 0,
                                stack_base_pointer: stack.len()
                            };

                            // Extend the stack with the arguments
                            stack.extend(args);
                            stack.resize(current_frame.stack_base_pointer + current_function.local_count, Variant::Null);
                        },
                        None => return runtime_error!("Function not found: {}", function_index)
                    };
                },

                Instruction::Return => {
                    let Some(returning_value) = stack.pop() else {
                        return runtime_error!("Return instruction without value");
                    };

                    if let Some(parent_frame) = frames.pop() {
                        stack.resize(current_frame.stack_base_pointer, Variant::Null);
                        current_frame = parent_frame;
                        stack.push(returning_value);
                        current_function = match self.functions.get(current_frame.function_index) {
                            Some(func) => func,
                            None => return runtime_error!("Function not found: {}", current_frame.function_index)
                        };
                    } else {
                        result = Some(returning_value);
                        break;
                    }
                }

                Instruction::EndFunction => {
                    if let Some(parent_frame) = frames.pop() {
                        debug!("Returning from function {}", current_function.name);
                        stack.resize(current_frame.stack_base_pointer, Variant::Null);
                        current_frame = parent_frame;
                        current_function = match self.functions.get(current_frame.function_index) {
                            Some(func) => func,
                            None => return runtime_error!("Function not found: {}", current_frame.function_index)
                        };
                    } else {
                        break;
                    }
                }

                // Binary Operations

                Instruction::Add => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a + b);
                    current_frame.pc += 1;
                },

                Instruction::Sub => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a - b);
                    current_frame.pc += 1;
                },

                Instruction::Mul => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a * b);
                    current_frame.pc += 1;
                },

                Instruction::Div => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a / b);
                    current_frame.pc += 1;
                },

                Instruction::Mod => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a % b);
                    current_frame.pc += 1;
                },

                Instruction::Pow => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a.pow(&b));
                    current_frame.pc += 1;
                },

                // Unary Operations

                Instruction::Equal => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a == b));
                    current_frame.pc += 1;
                },

                Instruction::GreaterThan => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a > b));
                    current_frame.pc += 1;
                }

                Instruction::LessThan => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a < b));
                    current_frame.pc += 1;
                },

                Instruction::LessEqual => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a <= b));
                    current_frame.pc += 1;
                },

                Instruction::GreaterEqual => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a >= b));
                    current_frame.pc += 1;
                },

                Instruction::NotEqual => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a != b));
                    current_frame.pc += 1;
                },

                Instruction::Or => {
                    let b = stack_pop!(stack).into();
                    let a = stack_pop!(stack).into();
                    stack.push(Variant::Boolean(a || b));
                    current_frame.pc += 1;
                },

                Instruction::And => {
                    let b = stack_pop!(stack).into();
                    let a = stack_pop!(stack).into();
                    stack.push(Variant::Boolean(a && b));
                    current_frame.pc += 1;
                },

                Instruction::Not => {
                    let a = stack_pop!(stack);
                    stack.push(!a);
                    current_frame.pc += 1;
                },

                Instruction::Negate => {
                    let a = stack_pop!(stack);
                    stack.push(-a);
                    current_frame.pc += 1;
                },

                // Arrays

                Instruction::CreateArray(size) => {
                    let mut array = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        array.push(stack_pop!(stack));
                    }
                    array.reverse();
                    stack.push(Variant::Array(Rc::new(RefCell::new(array))));
                    current_frame.pc += 1;
                },

                Instruction::GetArrayItem => {

                    let index = match stack_pop!(stack) {
                        Variant::Index(index) => index,
                        v => return runtime_error!("Expected an index but got {:?}", v)
                    };

                    let array = stack_pop!(stack);
                    let value = match array {
                        Variant::Array(array) => {
                            let array = array.borrow();
                            let index: usize = index;
                            match array.get(index) {
                                Some(value) => value.clone(),
                                None => return runtime_error!("Array index out of bounds: {} >= {}", index, array.len())
                            }
                        },
                        _ => return runtime_error!("Expected an array but got {:?}", array)
                    };
                    stack.push(value);
                    current_frame.pc += 1;
                }

                Instruction::SetArrayItem => {

                    let value = stack_pop!(stack);

                    let index = match stack_pop!(stack) {
                        Variant::Index(index) => index,
                        v => return runtime_error!("Expected an index but got {:?}", v)
                    };

                    let varray = stack_pop!(stack);
                    match varray {
                        Variant::Array(ref array) => {
                            let mut array = array.borrow_mut();
                            let index: usize = index;
                            array[index] = value;
                            stack.push(varray.clone());
                        },
                        _ => return runtime_error!("Expected an array but got {:?}", varray)
                    }
                    current_frame.pc += 1;
                },

                Instruction::GetArrayLength => {
                    let array = stack_pop!(stack);
                    let length = match array {
                        Variant::Array(array) => {
                            let array = array.borrow();
                            array.len()
                        },
                        _ => return runtime_error!("Expected an array but got {:?}", array)
                    };
                    stack.push(Variant::Integer(length as i64));
                    current_frame.pc += 1;
                },

                // Dictionaries

                Instruction::CreateDictionary(size) => {
                    let mut table = HashMap::new();
                    for _ in 0..*size {
                        let value = stack_pop!(stack);
                        let key = stack_pop!(stack);
                        table.insert(key, value);
                    }
                    stack.push(Variant::Dictionary(Rc::new(RefCell::new(table))));
                    current_frame.pc += 1;
                },

                Instruction::GetDictionaryItem => {
                    let key = stack_pop!(stack);
                    let table = stack_pop!(stack);
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
                    stack.push(value);
                    current_frame.pc += 1;
                }

                Instruction::SetDictionaryItem => {
                    let value = stack_pop!(stack);
                    let key = stack_pop!(stack);
                    let table = stack_pop!(stack);
                    match table {
                        Variant::Dictionary(table) => {
                            let mut table = table.borrow_mut();
                            table.insert(key, value);
                        },
                        _ => return runtime_error!("Expected an dictionary but got {:?}", table)
                    }
                    current_frame.pc += 1;
                },

                Instruction::GetDictionaryKeys => {
                    let table = stack_pop!(stack);
                    let keys = match table {
                        Variant::Dictionary(table) => {
                            let table = table.borrow();
                            table.keys().cloned().collect::<Vec<Variant>>()
                        },
                        _ => return runtime_error!("Expected an dictionary but got {:?}", table)
                    };
                    stack.push(Variant::Array(Rc::new(RefCell::new(keys))));
                    current_frame.pc += 1;
                },

                Instruction::Pop => {
                    stack_pop!(stack);
                    current_frame.pc += 1;
                },

                // Output
                Instruction::Print => {
                    let value = stack_pop!(stack);
                    println!("{}", value);
                    current_frame.pc += 1;
                },

                Instruction::Halt => {
                    break;
                },

                Instruction::Panic => {
                    let value = stack_pop!(stack);
                    return runtime_error!("Panic: {}", value);
                },

            }

        };

        Ok(VmExecutionResult {
            result,
            run_time: timer.elapsed()
        })

    }
    
}

fn get_function_call_args(stack: &mut Vec<Variant>, arity: usize) -> Vec<Variant> {
    let mut args = Vec::with_capacity(arity);
    for _ in 0..arity {
        args.push(stack_pop!(stack));
    }
    args.reverse();
    args
}

