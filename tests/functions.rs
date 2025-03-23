use std::collections::HashMap;
use bytevm::program::{Symbol, Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_user_defined_function() {

    let mut functions = HashMap::new();
    functions.insert(String::from("main"), Symbol::UserDefinedFunction {
        address: 0,
        arity: 0
    });
    functions.insert(String::from("add"), Symbol::UserDefinedFunction {
        address: 9,
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
            Instruction::PushIdentifier(String::from("add")),
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::FunctionCall(2),
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

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(3));
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
            Instruction::PushIdentifier(String::from("add")),
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::FunctionCall(2),
            Instruction::Halt
        ]
    };

    let mut vm = Vm::new(program, VmOptions::default());
    vm.register_native_function(String::from("add"), |args: Vec<Variant>| {
        let a = args[0].clone();
        let b = args[1].clone();
        Some(a + b)
    });

    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(3));
}