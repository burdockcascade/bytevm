use bytevm::program::{Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;

#[test]
fn test_user_defined_function() {
    
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, vec![
            // main
            Instruction::Push(Variant::Integer(1)),
            Instruction::SetLocal(0),
            Instruction::Push(Variant::Integer(2)),
            Instruction::SetLocal(1),
            Instruction::Push(Variant::Identifier(String::from("add"))),
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::FunctionCall(2),
            Instruction::Return
        ]
    );
    
    program.add_function(String::from("add"), 2, vec![
            Instruction::GetLocal(1),
            Instruction::GetLocal(0),
            Instruction::Add,
            Instruction::Return
        ]
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}

#[test]
fn test_builtin_function() {
    
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, vec![
            Instruction::Push(Variant::Identifier(String::from("add"))),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::FunctionCall(2),
            Instruction::Return
        ]
    );

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