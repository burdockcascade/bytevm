use bytevm::program::{Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;

#[test]
fn test_fib() {

    let input = 20;
    let expected_result = fib(input);

    let mut program  = Program::default();

    program.add_function(String::from("main"), 0, vec![
        Instruction::Push(Variant::Integer(input)),
        Instruction::SetLocal(0),
        Instruction::Push(Variant::FunctionPointer(1)),
        Instruction::GetLocal(0),
        Instruction::FunctionCall(1),
        Instruction::Return,
    ]);
    
    program.add_function(String::from("fib"), 2, vec![
        Instruction::GetLocal(0),
        Instruction::Push(Variant::Integer(1)),
        Instruction::LessEqual,
        Instruction::JumpIfFalse(7),
        Instruction::GetLocal(0),
        Instruction::Return,
        Instruction::Jump(7),
        Instruction::Push(Variant::FunctionPointer(1)),
        Instruction::GetLocal(0),
        Instruction::Push(Variant::Integer(1)),
        Instruction::Sub,
        Instruction::FunctionCall(1),
        Instruction::Push(Variant::FunctionPointer(1)),
        Instruction::GetLocal(0),
        Instruction::Push(Variant::Integer(2)),
        Instruction::Sub,
        Instruction::FunctionCall(1),
        Instruction::Add,
        Instruction::Return,
    ]);

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