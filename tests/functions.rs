#![allow(unused_imports)]
#![allow(unused_variables)]

use std::collections::HashMap;
use bytevm::program::{Symbol, Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm;

#[test]
fn test_user_defined_function() {

    let mut functions = HashMap::new();
    functions.insert(String::from("main"), Symbol::UserDefinedFunction {
        address: 0,
        arity: 0
    });
    functions.insert(String::from("add"), Symbol::UserDefinedFunction {
        address: 8,
        arity: 2
    });

    let program = Program {
        symbols: functions,
        instructions: vec![
            // main
            Instruction::PushInteger(1),
            Instruction::SetLocal(0),
            Instruction::PushInteger(2),
            Instruction::SetLocal(1),
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::FunctionCall(String::from("add")),
            Instruction::Halt,

            // add
            Instruction::SetLocal(0),
            Instruction::SetLocal(1),
            Instruction::GetLocal(1),
            Instruction::GetLocal(0),
            Instruction::Add,
            Instruction::Return
        ]
    };

    let result = bytevm::run(program).unwrap();
    assert_eq!(result.result.unwrap(), Variant::Integer(3));
}

#[test]
fn test_builtin_function() {

    let mut functions = HashMap::new();
    functions.insert(String::from("main"), Symbol::UserDefinedFunction {
        address: 0,
        arity: 0
    });
    functions.insert(String::from("add"), Symbol::NativeFunction {
        arity: 2
    });

    let program = Program {
        symbols: functions,
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::FunctionCall(String::from("add")),
            Instruction::Halt
        ]
    };

    let mut vm = vm::Vm::new(program);
    vm.register_native_function(String::from("add"), |args: Vec<Variant>| {
        let a = args[0].clone();
        let b = args[1].clone();
        Some(a + b)
    });
    let result = vm.run(None);

    assert_eq!(result.unwrap().result.unwrap(), Variant::Integer(3));
}