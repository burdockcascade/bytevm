use bytevm::program::{GlobalEntry, Instruction, Program};
use bytevm::variant::Variant;
use bytevm::runtime::Vm;
use std::collections::HashMap;

#[test]
fn test_user_defined_function() {

    let mut globals = HashMap::new();
    globals.insert(String::from("main"), GlobalEntry::UserDefinedFunction {
        address: 0,
        arity: 0
    });
    globals.insert(String::from("add"), GlobalEntry::UserDefinedFunction {
        address: 9,
        arity: 2
    });

    let program = Program {
        globals,
        instructions: vec![
            // main
            Instruction::Push(Variant::Integer(1)),
            Instruction::SetLocal(0),
            Instruction::Push(Variant::Integer(2)),
            Instruction::SetLocal(1),
            Instruction::Push(Variant::Identifier(String::from("add"))),
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::FunctionCall(2),
            Instruction::Halt,

            // add
            Instruction::GetLocal(1),
            Instruction::GetLocal(0),
            Instruction::Add,
            Instruction::Return
        ]
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}

#[test]
fn test_builtin_function() {

    let mut functions = HashMap::new();
    functions.insert(String::from("main"), GlobalEntry::UserDefinedFunction {
        address: 0,
        arity: 0
    });
    functions.insert(String::from("add"), GlobalEntry::NativeFunction {
        arity: 2
    });

    let program = Program {
        globals: functions,
        instructions: vec![
            Instruction::Push(Variant::Identifier(String::from("add"))),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::FunctionCall(2),
            Instruction::Halt
        ]
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    vm.register_native_function(String::from("add"), |args: Vec<Variant>| {
        let a = args[0].clone();
        let b = args[1].clone();
        Some(a + b)
    });

    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(3));
}