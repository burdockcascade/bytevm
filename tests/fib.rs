use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_user_defined_function() {

    let input = 10;
    let expected_result = fib(input);

    let program = Program {
        instructions: vec![

            // main
            Instruction::Push(Variant::FunctionPointer(4)),
            Instruction::Push(Variant::Integer(input)),
            Instruction::FunctionCall(1),
            Instruction::Return,

            // fib
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::LessEqual,
            Instruction::JumpIfFalse(11),
            Instruction::GetLocal(0),
            Instruction::Return,
            Instruction::Jump(11),
            Instruction::Push(Variant::FunctionPointer(4)),
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Sub,
            Instruction::FunctionCall(1),
            Instruction::Push(Variant::FunctionPointer(4)),
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Sub,
            Instruction::FunctionCall(1),
            Instruction::Add,
            Instruction::Return,

            // Halt
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run().unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(expected_result));
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}