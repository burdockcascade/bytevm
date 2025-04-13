use crate::program::{CallTarget, Function, Instruction, SymbolEntry};
use crate::variant::Variant;
use std::collections::HashMap;
use crate::prelude::Program;

#[derive(Clone, Debug, Default)]
pub struct ProgramBuilder {
    program: Program,
}

impl ProgramBuilder {

    pub fn add_function(&mut self, function: Function) {
        match self.program.symbol_table.get(&function.name) {
            Some(SymbolEntry::UserDefinedFunction { index }) => {
                self.program.functions[*index] = function;
            }
            None => {
                self.program.symbol_table.insert(function.name.clone(), SymbolEntry::UserDefinedFunction {
                    index: self.program.functions.len()
                });
                self.program.functions.push(function);
            }
            _ => panic!("Cannot redefine function {}", function.name),
        }
    }

    pub fn add_symbol(&mut self, name: String, entry: SymbolEntry) {
        self.program.symbol_table.insert(name, entry);
    }

    pub fn build(mut self) -> Program {

        // Resolve function references with function index
        for function in &mut self.program.functions {
            for instruction in &mut function.instructions {
                if let Instruction::FunctionCall(CallTarget::Name(name)) = instruction {
                    if let Some(SymbolEntry::UserDefinedFunction { index }) = self.program.symbol_table.get(name) {
                        *instruction = Instruction::FunctionCall(CallTarget::Index(*index));
                    }
                }
            }
        }

        self.program
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct FunctionBuilder {
    name: String,
    arity: usize,
    local_count: usize,
    body: Vec<Instruction>,
}

impl FunctionBuilder {

    /// Creates a new function builder with default values.
    pub fn name(&mut self, name: &str) -> &mut Self {
        self.name = name.to_string();
        self
    }

    /// Sets the number of arguments for the function.
    pub fn arity(&mut self, arity: usize) -> &mut Self {
        self.arity = arity;
        self
    }

    /// Sets the body of the function.
    pub fn body(&mut self, body: &mut BlockEncoder) -> &mut Self {
        self.body = body.encode();
        self.local_count = body.next_local_slot;
        self
    }

    /// Builds the function and returns it.
    pub fn build(&mut self) -> Function {
        Function {
            name: self.name.clone(),
            arity: self.arity.clone(),
            local_count: self.local_count,
            instructions: self.body.clone(),
        }
    }

}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct BlockEncoder {
    instructions: Vec<Instruction>,
    variable_names: HashMap<String, usize>,
    next_local_slot: usize,
    labels: HashMap<String, usize>,
    pending_jumps: HashMap<String, usize>,
    known_functions: HashMap<String, usize>,
}

impl BlockEncoder {

    fn push(&mut self, instruction: Instruction) -> &mut Self {
        self.instructions.push(instruction);
        self
    }

    /// Declares a local variable with the given name.
    pub fn declare_local(&mut self, name: &str) -> &mut Self {
        if !self.variable_names.contains_key(name) {
            self.variable_names.insert(name.to_string(), self.next_local_slot);
            self.next_local_slot += 1;
        }
        self
    }

    /// Sets a local variable to a value.
    pub fn set_local(&mut self, name: &str) -> &mut Self {
        if let Some(&index) = self.variable_names.get(name) {
            self.push(Instruction::SetLocal(index))
        } else {
            panic!("Local variable {} not declared", name);
        }
    }

    /// Get value from a local variable.
    pub fn get_local(&mut self, name: &str) -> &mut Self {
        if let Some(&index) = self.variable_names.get(name) {
            self.push(Instruction::GetLocal(index))
        } else {
            panic!("Local variable {} not declared", name);
        }
    }

    /// Adds a label to the instruction list.
    pub fn add_label(&mut self, label: &str) -> &mut Self {
        self.labels.insert(label.to_string(), self.instructions.len());
        self
    }

    /// Jumps to a label. If the label is not found, it will be added to the pending jumps.
    pub fn jump(&mut self, label: &str) -> &mut Self {
        if let Some(&index) = self.labels.get(label) {
            self.push(Instruction::Jump(index))
        } else {
            self.pending_jumps.insert(label.to_string(), self.instructions.len());
            self.push(Instruction::Jump(0))
        }
    }

    /// Jumps to a label if the top of the stack is false. If the label is not found, it will be added to the pending jumps.
    pub fn jump_if_false(&mut self, label: &str) -> &mut Self {
        if let Some(&index) = self.labels.get(label) {
            self.push(Instruction::JumpIfFalse(index))
        } else {
            self.pending_jumps.insert(label.to_string(), self.instructions.len());
            self.push(Instruction::JumpIfFalse(0))
        }
    }

    /// Pushes an integer value onto the stack.
    pub fn push_integer(&mut self, value: i64) -> &mut Self {
        self.push(Instruction::Push(Variant::Integer(value)))
    }

    /// Pushes a float value onto the stack.
    pub fn push_float(&mut self, value: f64) -> &mut Self {
        self.push(Instruction::Push(Variant::Float(value)))
    }

    /// Pushes a string value onto the stack.
    pub fn push_string(&mut self, value: String) -> &mut Self {
        self.push(Instruction::Push(Variant::String(value)))
    }

    /// Pushes a boolean value onto the stack.
    pub fn push_boolean(&mut self, value: bool) -> &mut Self {
        self.push(Instruction::Push(Variant::Boolean(value)))
    }   

    /// Pushes null onto the stack.
    pub fn push_null(&mut self) -> &mut Self {
        self.push(Instruction::Push(Variant::Null))
    }

    /// Pushes a symbol reference onto the stack.
    pub fn push_symbol(&mut self, value: &str) -> &mut Self {
        self.push(Instruction::Push(Variant::SymbolReference(value.to_string())))
    }

    /// Pushes a function pointer onto the stack.
    pub fn push_function_reference(&mut self, index: &str) -> &mut Self {
        self.push_symbol(index)
    }

    /// Add tos and tos-1 and push the result.
    pub fn add(&mut self) -> &mut Self {
        self.push(Instruction::Add)
    }

    /// Subtract tos and tos-1 and push the result.
    pub fn sub(&mut self) -> &mut Self {
        self.push(Instruction::Sub)
    }

    /// Multiply tos and tos-1 and push the result.
    pub fn mul(&mut self) -> &mut Self {
        self.push(Instruction::Mul)
    }

    /// Divide tos and tos-1 and push the result.
    pub fn div(&mut self) -> &mut Self {
        self.push(Instruction::Div)
    }

    /// Modulus the tos and tos-1 and push the result.
    pub fn modulus(&mut self) -> &mut Self {
        self.push(Instruction::Mod)
    }

    /// Exponentiate the tos and tos-1 and push the result.
    pub fn pow(&mut self) -> &mut Self {
        self.push(Instruction::Pow)
    }

    /// Compares the top two values on the stack and pushes the result.
    pub fn equal(&mut self) -> &mut Self {
        self.push(Instruction::Equal)
    }

    /// Negates the top value on the stack and pushes the result.
    pub fn negate(&mut self) -> &mut Self {
        self.push(Instruction::Negate)
    }

    /// Pushes true onto the stack if the top two values are equal.
    pub fn less_than(&mut self) -> &mut Self {
        self.push(Instruction::LessThan)
    }

    pub fn less_than_or_equal(&mut self) -> &mut Self {
        self.push(Instruction::LessEqual)
    }

    pub fn greater_than(&mut self) -> &mut Self {
        self.push(Instruction::GreaterThan)
    }

    pub fn greater_than_or_equal(&mut self) -> &mut Self {
        self.push(Instruction::GreaterEqual)
    }

    pub fn not_equal(&mut self) -> &mut Self {
        self.push(Instruction::NotEqual)
    }

    /// Inverts the top value on the stack and pushes the result.
    pub fn not(&mut self) -> &mut Self {
        self.push(Instruction::Not)
    }

    /// Pushes true onto the stack if both of the top two values on the stack are true.
    pub fn and(&mut self) -> &mut Self {
        self.push(Instruction::And)
    }

    /// Pushes true onto the stack if either of the top two values on the stack are true.
    pub fn or(&mut self) -> &mut Self {
        self.push(Instruction::Or)
    }

    /// Calls a function by its name and pushes the result onto the stack.
    pub fn call_function_by_name(&mut self, name: &str) -> &mut Self {
        self.push(Instruction::FunctionCall(CallTarget::Name(name.to_string())))
    }

    /// Calls a function by its index and pushes the result onto the stack.
    pub fn call_function_by_index(&mut self, index: usize) -> &mut Self {
        self.push(Instruction::FunctionCall(CallTarget::Index(index)))
    }

    pub fn create_array(&mut self, size: usize) -> &mut Self {
        self.push(Instruction::CreateArray(size))
    }

    pub fn get_array_item(&mut self) -> &mut Self {
        self.push(Instruction::GetArrayItem)
    }

    pub fn set_array_item(&mut self) -> &mut Self {
        self.push(Instruction::SetArrayItem)
    }

    /// Gets the length of the array and pushes it onto the stack as an integer.
    pub fn get_array_length(&mut self) -> &mut Self {
        self.push(Instruction::GetArrayLength)
    }

    pub fn create_dictionary(&mut self, size: usize) -> &mut Self {
        self.push(Instruction::CreateDictionary(size))
    }

    pub fn get_dictionary_item(&mut self) -> &mut Self {
        self.push(Instruction::GetDictionaryItem)
    }

    pub fn set_dictionary_item(&mut self) -> &mut Self {
        self.push(Instruction::SetDictionaryItem)
    }

    /// Gets the keys of the dictionary and pushes them onto the stack as an array.
    pub fn get_dictionary_keys(&mut self) -> &mut Self {
        self.push(Instruction::GetDictionaryKeys)
    }

    /// Halts the execution of the function and returns the top of the stack.
    pub fn return_value(&mut self) -> &mut Self {
        self.push(Instruction::Return)
    }

    /// Prints the top of the stack to the console.
    pub fn print(&mut self) -> &mut Self {
        self.push(Instruction::Print)
    }

    /// Halts the execution of the function and returns no value.
    pub fn halt(&mut self) -> &mut Self {
        self.push(Instruction::Halt)
    }

    /// Panics the VM with the top of the stack as the error message.
    pub fn panic(&mut self) -> &mut Self {
        self.push(Instruction::Panic)
    }

    /// Returns the instructions as a vector of Instruction.
    pub fn encode(&mut self) -> Vec<Instruction> {

        // Insert Halt at the end of the block if not already present
        if let Some(last_instruction) = self.instructions.last() {
            match last_instruction {
                Instruction::Return | Instruction::Halt => {}
                _ => {
                    self.push(Instruction::Halt);
                }
            }
        }

        // Resolve pending jumps
        for (label, index) in &self.pending_jumps {
            if let Some(&target_index) = self.labels.get(label) {
                match self.instructions.get_mut(*index) {
                    Some(Instruction::Jump(_)) => {
                        self.instructions[*index] = Instruction::Jump(target_index);
                    }
                    Some(Instruction::JumpIfFalse(_)) => {
                        self.instructions[*index] = Instruction::JumpIfFalse(target_index);
                    }
                    ins => unreachable!("Expected Jump or JumpIfFalse instruction, found {:?}", ins),
                }
            } else {
                panic!("Label {} not found", label);
            }
        }

        self.instructions.clone()
    }
    
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_jump_to_label() {
        let instructions = BlockEncoder::default()
            .declare_local("i")
            .declare_local("max")
            .push_integer(0)
            .set_local("i")
            .push_integer(10)
            .set_local("max")
            .add_label("start")
            .get_local("i")
            .get_local("max")
            .less_than()
            .jump_if_false("end")
            .get_local("i")
            .push_integer(1)
            .add()
            .set_local("i")
            .jump("start")
            .add_label("end")
            .get_local("i")
            .return_value()
            .encode();

        assert_eq!(instructions, vec![
            Instruction::Push(Variant::Integer(0)),
            Instruction::SetLocal(0),
            Instruction::Push(Variant::Integer(10)),
            Instruction::SetLocal(1),
            Instruction::GetLocal(0), // start
            Instruction::GetLocal(1),
            Instruction::LessThan,
            Instruction::JumpIfFalse(13),
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Add,
            Instruction::SetLocal(0),
            Instruction::Jump(4),
            Instruction::GetLocal(0), // end
            Instruction::Return
        ]);
    }

    #[test]
    #[should_panic]
    fn test_undeclared_local() {
        let mut encoder = BlockEncoder::default();
        encoder.declare_local("x");
        encoder.set_local("y"); // y is not declared
        let instructions = encoder.encode();
        assert_eq!(instructions, vec![]);
    }

    #[test]
    #[should_panic]
    fn test_undeclared_label() {
        let mut encoder = BlockEncoder::default();
        encoder.add_label("start");
        encoder.jump("end"); // end is not declared
        let instructions = encoder.encode();
        assert_eq!(instructions, vec![]);
    }
    
}