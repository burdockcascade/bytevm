use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_create_array() {
    let program = Program {
        instructions: vec![
            // Create array [1, 2, 3]
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateArray(3),
            Instruction::Return
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
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
        instructions: vec![
            // Create array [1, 2, 3]
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateArray(3),

            // Get array[1]
            Instruction::Push(Variant::Integer(1)),
            Instruction::GetArrayItem,

            // Return
            Instruction::Return
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(2));
}

#[test]
fn test_set_array_element() {
    let program = Program {
        instructions: vec![
            // Create array [1, 2, 3]
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateArray(3),
            Instruction::SetLocal(0),

            // Set array[1] = 4
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(4)),
            Instruction::SetArrayItem,

            // Get array[1]
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::GetArrayItem,

            // Return
            Instruction::Return
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(4));
}
