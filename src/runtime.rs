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
        let mut stack = Vec::with_capacity(1024);
        
        // use entry point or default to main
        let entry_point = entry_point.unwrap_or_else(|| String::from("main"));

        // Get the function to execute
        let mut function_index = match self.symbols.get(entry_point.as_str()) {
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
        stack.resize(self.functions[function_index].local_count, Variant::Null);

        // Initialize the stack frame
        let mut pc = 0;
        let mut stack_base_pointer = 0;
        
        debug!("Starting execution of function: {}", self.functions[function_index].name);
        let mut result = None;

        loop  {
            
            // trace!("========================================");
            // trace!("Frame[{}]: Stack: {:?}", frames.len(), stack);
            // trace!("Frame[{}]: Base pointer: {}", frames.len(), stack_base_pointer);
            // trace!("Frame[{}]: Local Count: {}", frames.len(), self.functions[function_index].local_count);
            // trace!("Frame[{}]: Locals: {:?}", frames.len(), &stack[stack_base_pointer .. stack_base_pointer + self.functions[function_index].local_count]);
            // trace!("Frame[{}]: Operands: {:?}", frames.len(), &stack[stack_base_pointer + self.functions[function_index].local_count..]);

            let Some(instruction) = self.functions[function_index].instructions.get(pc) else {
                // debug!("Frame[{}]: Instructions {:?}", frames.len(), self.functions[function_index].instructions);
                return runtime_error!("Program counter out of bounds: {} >= {}", pc, self.functions[function_index].instructions.len());
            };
            
            // trace!("Frame[{}]: Executing instruction[{}]: {:?}", frames.len(), pc, instruction);
            
            match instruction {

                // Operands

                Instruction::Push(value) => {
                    stack.push(value.clone());
                    pc += 1;
                },

                // Local variables

                Instruction::SetLocal(index) => {
                    let value = stack_pop!(stack);
                    let variable_index = stack_base_pointer + *index;
                    let stack_len = stack.len();
                    stack.get_mut(variable_index)
                        .expect(format!("Local variable index out of bounds: {} >= {}", variable_index, stack_len).as_str())
                        .clone_from(&value);
                    pc += 1;
                },

                Instruction::GetLocal(index) => {
                    let value = stack[stack_base_pointer + *index].clone();
                    stack.push(value);
                    pc += 1;
                },

                // Jump instructions

                Instruction::Jump(address) => {
                    pc = *address;
                },

                Instruction::JumpIfFalse(address) => {
                    let var = stack_pop!(stack);
                    match var {
                        Variant::Boolean(value) => {
                            if !value {
                                pc = *address;
                            } else {
                                pc += 1;
                            }
                        },
                        v => return runtime_error!("Expected a boolean but got {:?}", v)
                    }
                },

                // Binary Operations

                Instruction::Add => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a + b);
                    pc += 1;
                },

                Instruction::Sub => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a - b);
                    pc += 1;
                },

                Instruction::Mul => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a * b);
                    pc += 1;
                },

                Instruction::Div => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a / b);
                    pc += 1;
                },

                Instruction::Mod => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a % b);
                    pc += 1;
                },

                Instruction::Pow => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(a.pow(&b));
                    pc += 1;
                },

                // Unary Operations

                Instruction::Equal => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a == b));
                    pc += 1;
                },

                Instruction::GreaterThan => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a > b));
                    pc += 1;
                }

                Instruction::LessThan => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a < b));
                    pc += 1;
                },

                Instruction::LessEqual => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a <= b));
                    pc += 1;
                },

                Instruction::GreaterEqual => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a >= b));
                    pc += 1;
                },

                Instruction::NotEqual => {
                    let b = stack_pop!(stack);
                    let a = stack_pop!(stack);
                    stack.push(Variant::Boolean(a != b));
                    pc += 1;
                },

                Instruction::Or => {
                    let b = stack_pop!(stack).into();
                    let a = stack_pop!(stack).into();
                    stack.push(Variant::Boolean(a || b));
                    pc += 1;
                },

                Instruction::And => {
                    let b = stack_pop!(stack).into();
                    let a = stack_pop!(stack).into();
                    stack.push(Variant::Boolean(a && b));
                    pc += 1;
                },

                Instruction::Not => {
                    let a = stack_pop!(stack);
                    stack.push(!a);
                    pc += 1;
                },

                Instruction::Negate => {
                    let a = stack_pop!(stack);
                    stack.push(-a);
                    pc += 1;
                },

                // Arrays

                Instruction::CreateArray(size) => {
                    let mut array = Vec::with_capacity(*size);
                    for _ in 0..*size {
                        array.push(stack_pop!(stack));
                    }
                    array.reverse();
                    stack.push(Variant::Array(Rc::new(RefCell::new(array))));
                    pc += 1;
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
                    pc += 1;
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
                    pc += 1;
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
                    pc += 1;
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
                    pc += 1;
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
                    pc += 1;
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
                    pc += 1;
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
                    pc += 1;
                },

                Instruction::Pop => {
                    stack_pop!(stack);
                    pc += 1;
                },

                // Function calls

                Instruction::FunctionCall(target) => {

                    let next_function_index = match target {
                        CallTarget::Index(index) => *index,
                        CallTarget::Name(name) if self.native_functions.contains_key(name) => {
                            let arity = match self.symbols.get(name.as_str()) {
                                Some(SymbolEntry::NativeFunction { arity }) => *arity,
                                _ => return runtime_error!("Native function not found: {}", name)
                            };
                            let func = match self.native_functions.get(name.as_str()) {
                                Some(func) => func,
                                None => return runtime_error!("Native function not found: {}", name)
                            };
                            
                            let args = stack.drain(stack.len() - arity..).collect::<Vec<_>>();
                            if let Some(value) = func(args) {
                                stack.push(value);
                            }
                            pc += 1;
                            continue;
                        }
                        CallTarget::Name(name) => {
                            // User defined function
                            match self.symbols.get(name.as_str()) {
                                Some(SymbolEntry::UserDefinedFunction { index, .. }) => *index,
                                _ => return runtime_error!("Function not found: {}", name)
                            }
                        },
                    };
                    
                    // Remember the current function frame
                    frames.push(StackFrame {
                        function_index,
                        pc: pc + 1,
                        stack_base_pointer
                    });
                    
                    // Create a new stack frame for the function call
                    pc = 0;
                    
                    // Set the stack base pointer to the current stack length and include the function's arity
                    stack_base_pointer = stack.len() - self.functions[next_function_index].arity;

                    // Extend the stack with the arguments
                    stack.resize(stack_base_pointer + self.functions[next_function_index].local_count, Variant::Null);

                    // Update the current function to the next function
                    function_index = next_function_index;
  
                },

                Instruction::Return => {
                    let Some(returning_value) = stack.pop() else {
                        return runtime_error!("Return instruction without value");
                    };

                    if let Some(parent_frame) = frames.pop() {

                        stack.resize(stack_base_pointer, Variant::Null);

                        pc = parent_frame.pc;
                        stack_base_pointer = parent_frame.stack_base_pointer;

                        stack.push(returning_value);
                        function_index = parent_frame.function_index;
                    } else {
                        result = Some(returning_value);
                        break;
                    }
                }

                Instruction::EndFunction => {
                    if let Some(parent_frame) = frames.pop() {
                        debug!("Returning from function {}", self.functions[function_index].name);
                        stack.resize(stack_base_pointer, Variant::Null);
                        pc = parent_frame.pc;
                        stack_base_pointer = parent_frame.stack_base_pointer;
                        function_index = parent_frame.function_index;   
                    } else {
                        break;
                    }
                }

                // Output
                Instruction::Print => {
                    let value = stack_pop!(stack);
                    println!("{}", value);
                    pc += 1;
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

