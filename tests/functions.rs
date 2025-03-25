use std::collections::HashMap;
use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_user_defined_function() {

    let program = Program {
        instructions: vec![
            // main
            Instruction::Push(Variant::Integer(1)),
            Instruction::SetLocal(0),
            Instruction::Push(Variant::Integer(2)),
            Instruction::SetLocal(1),
            Instruction::Push(Variant::FunctionPointer(9)),
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::FunctionCall(2),
            Instruction::Halt,

            // add
            Instruction::GetLocal(1),
            Instruction::GetLocal(0),
            Instruction::Add,
            Instruction::Return
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run().unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(3));
}