use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::runtime::Vm;

#[test]
fn test_jump() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Jump(4),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Return,
            Instruction::Push(Variant::Integer(3)),
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}

#[test]
fn test_jump_if_false() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Boolean(false)),
            Instruction::JumpIfFalse(4),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Return,
            Instruction::Push(Variant::Integer(2)),
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(2));
}

#[test]
fn test_dont_jump_if_true() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Boolean(true)),
            Instruction::JumpIfFalse(4),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Return,
            Instruction::Push(Variant::Integer(2)),
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(1));
}

#[test]
fn test_for_while_loop() {
    let program = Program {
        instructions: vec![
            // Set i = 0
            Instruction::Push(Variant::Integer(0)),
            Instruction::SetLocal(0),

            // Set max = 10
            Instruction::Push(Variant::Integer(10)),
            Instruction::SetLocal(1),

            // while i < max
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::LessThan,

            // Jump if false
            Instruction::JumpIfFalse(13),

            // Increment i
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Add,
            Instruction::SetLocal(0),

            // Jump to the beginning
            Instruction::Jump(4),

            // Return i
            Instruction::GetLocal(0),
            Instruction::Return

        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(10));
}

#[test]
fn test_foreach_array_loop() {

    let program = Program {
        instructions: vec![
            // Create an array with 3 elements
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateArray(3),
            Instruction::SetLocal(0),

            // Set i = 0
            Instruction::Push(Variant::Integer(0)),
            Instruction::SetLocal(1),

            // Get the length of the array and store it in max
            Instruction::GetLocal(0),
            Instruction::GetArrayLength,
            Instruction::SetLocal(2),

            // while i < max
            Instruction::GetLocal(1), // 10
            Instruction::GetLocal(2),
            Instruction::LessThan,

            // Jump if false
            Instruction::JumpIfFalse(23), // 13

            // Get the array
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::GetArrayItem,
            Instruction::SetLocal(3),

            // Increment i
            Instruction::GetLocal(1),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Add,
            Instruction::SetLocal(1),

            // Jump to the beginning
            Instruction::Jump(10),

            // Return i
            Instruction::GetLocal(1),
            Instruction::Return // 23

        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}
