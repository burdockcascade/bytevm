use bytevm::program::{Function, GlobalEntry, Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;
use std::collections::HashMap;

#[test]
fn test_fib() {

    let input = 20;
    let expected_result = fib(input);

    let mut globals = HashMap::new();
    globals.insert(String::from("main"), GlobalEntry::UserDefinedFunction {
        index: 1,
        arity: 0
    });

    let fib_func_name = String::from("fib");
    globals.insert(fib_func_name.clone(), GlobalEntry::UserDefinedFunction {
        index: 0,
        arity: 2
    });

    let program = Program {
        globals,
        functions: vec![
            Function {
                name: String::from("fib"),
                arity: 2,
                instructions: vec![
                    Instruction::GetLocal(0),
                    Instruction::Push(Variant::Integer(1)),
                    Instruction::LessEqual,
                    Instruction::JumpIfFalse(7),
                    Instruction::GetLocal(0),
                    Instruction::Return,
                    Instruction::Jump(7),
                    Instruction::Push(Variant::Identifier(fib_func_name.clone())),
                    Instruction::GetLocal(0),
                    Instruction::Push(Variant::Integer(1)),
                    Instruction::Sub,
                    Instruction::FunctionCall(1),
                    Instruction::Push(Variant::Identifier(fib_func_name.clone())),
                    Instruction::GetLocal(0),
                    Instruction::Push(Variant::Integer(2)),
                    Instruction::Sub,
                    Instruction::FunctionCall(1),
                    Instruction::Add,
                    Instruction::Return,
                ]
            },
            Function {
                name: String::from("main"),
                arity: 0,
                instructions: vec![
                    Instruction::Push(Variant::Identifier(fib_func_name.clone())),
                    Instruction::Push(Variant::Integer(input)),
                    Instruction::FunctionCall(1),
                    Instruction::Return,
                ]
            }
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap();
    
    assert_eq!(result.result.unwrap(), Variant::Integer(expected_result));
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}