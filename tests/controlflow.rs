use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

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

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
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

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
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

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(1));
}

#[test]
fn test_for_i_loop() {
    let program = Program {
        instructions: vec![
            // Set i = 0
            Instruction::Push(Variant::Integer(0)),
            Instruction::SetLocal(0),

            // Set max = 10
            Instruction::Push(Variant::Integer(10)),
            Instruction::SetLocal(1),

            // Evaluate i < max
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

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(10));
}