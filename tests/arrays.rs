use std::collections::HashMap;
use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::Vm;

#[test]
fn test_create_array() {
    let program = Program {
        symbols: HashMap::new(),
        instructions: vec![
            // Create array [1, 2, 3]
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::PushInteger(3),
            Instruction::CreateArray(3),
            Instruction::Return
        ]
    };

    let vm = Vm::new(program);
    let result = vm.run(None).unwrap().result.unwrap();
    match result {
        Variant::Array(array) => {
            assert_eq!(array.borrow().len(), 3);
            assert_eq!(array.borrow().get(0), Some(&Variant::Integer(1)));
            assert_eq!(array.borrow().get(1), Some(&Variant::Integer(2)));
            assert_eq!(array.borrow().get(2), Some(&Variant::Integer(3)));
        }
        _ => panic!("Expected array")
    }
}

#[test]
fn test_get_array_element() {
    let program = Program {
        symbols: HashMap::new(),
        instructions: vec![
            // Create array [1, 2, 3]
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::PushInteger(3),
            Instruction::CreateArray(3),

            // Get array[1]
            Instruction::PushInteger(1),
            Instruction::GetArrayItem,

            // Return
            Instruction::Return
        ]
    };

    let vm = Vm::new(program);
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(2));
}

#[test]
fn test_set_array_element() {
    let program = Program {
        symbols: HashMap::new(),
        instructions: vec![
            // Create array [1, 2, 3]
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::PushInteger(3),
            Instruction::CreateArray(3),
            Instruction::SetLocal(0),

            // Set array[1] = 4
            Instruction::GetLocal(0),
            Instruction::PushInteger(1),
            Instruction::PushInteger(4),
            Instruction::SetArrayItem,

            // Get array[1]
            Instruction::GetLocal(0),
            Instruction::PushInteger(1),
            Instruction::GetArrayItem,

            // Return
            Instruction::Return
        ]
    };

    let vm = Vm::new(program);
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(4));
}
