#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;
use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::Vm;

#[test]
fn test_jump() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::Jump(4),
            Instruction::PushInteger(2),
            Instruction::Return,
            Instruction::PushInteger(3),
            Instruction::Return
        ],
        ..Default::default()
    };

    let vm = Vm::new(program);
    assert_eq!(vm.run(None).unwrap().result.unwrap(), Variant::Integer(3));
}

#[test]
fn test_jump_if_false() {
    let program = Program {
        instructions: vec![
            Instruction::PushBoolean(false),
            Instruction::JumpIfFalse(4),
            Instruction::PushInteger(1),
            Instruction::Return,
            Instruction::PushInteger(2),
            Instruction::Return
        ],
        ..Default::default()
    };

    let vm = Vm::new(program);
    assert_eq!(vm.run(None).unwrap().result.unwrap(), Variant::Integer(2));
}

#[test]
fn test_dont_jump_if_true() {
    let program = Program {
        instructions: vec![
            Instruction::PushBoolean(true),
            Instruction::JumpIfFalse(4),
            Instruction::PushInteger(1),
            Instruction::Return,
            Instruction::PushInteger(2),
            Instruction::Return
        ],
        ..Default::default()
    };

    let vm = Vm::new(program);
    assert_eq!(vm.run(None).unwrap().result.unwrap(), Variant::Integer(1));
}

#[test]
fn test_for_i_loop() {
    let program = Program {
        instructions: vec![
            // Set i = 0
            Instruction::PushInteger(0),
            Instruction::SetLocal(0),

            // Set max = 10
            Instruction::PushInteger(10),
            Instruction::SetLocal(1),

            // Evaluate i < max
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::LessThan,

            // Jump if false
            Instruction::JumpIfFalse(13),

            // Increment i
            Instruction::GetLocal(0),
            Instruction::PushInteger(1),
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

    let vm = Vm::new(program);
    assert_eq!(vm.run(None).unwrap().result.unwrap(), Variant::Integer(10));
}