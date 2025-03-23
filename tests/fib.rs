use std::collections::HashMap;
use bytevm::program::{Instruction, Program, Symbol};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_user_defined_function() {

    let input = 20;
    let expected_result = fib(input);

    let mut functions = HashMap::new();
    functions.insert(String::from("main"), Symbol::UserDefinedFunction {
        address: 0,
        arity: 0
    });

    let fib_func_name = String::from("fib");
    functions.insert(fib_func_name.clone(), Symbol::UserDefinedFunction {
        address: 4,
        arity: 2
    });

    let program = Program {
        symbols: functions,
        instructions: vec![

            // main
            Instruction::PushIdentifier(fib_func_name.clone()),
            Instruction::PushInteger(input),
            Instruction::FunctionCall(1),
            Instruction::Return,

            // fib
            Instruction::GetLocal(0),
            Instruction::PushInteger(1),
            Instruction::LessEqual,
            Instruction::JumpIfFalse(11),
            Instruction::GetLocal(0),
            Instruction::Return,
            Instruction::Jump(11),
            Instruction::PushIdentifier(fib_func_name.clone()),
            Instruction::GetLocal(0),
            Instruction::PushInteger(1),
            Instruction::Sub,
            Instruction::FunctionCall(1),
            Instruction::PushIdentifier(fib_func_name.clone()),
            Instruction::GetLocal(0),
            Instruction::PushInteger(2),
            Instruction::Sub,
            Instruction::FunctionCall(1),
            Instruction::Add,
            Instruction::Return,

            // Halt
            Instruction::Halt
        ]
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(expected_result));
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}