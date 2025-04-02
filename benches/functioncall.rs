use bytevm::program::{Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;
use criterion::{criterion_group, criterion_main, Criterion};

// Your function to benchmark
fn test_function_call() {
    let mut program = Program::default();
    
    program.add_main_function(vec![
        Instruction::Push(Variant::Integer(1)),
        Instruction::SetLocal(0),
        Instruction::Push(Variant::Integer(2)),
        Instruction::SetLocal(1),
        Instruction::Push(Variant::Identifier(String::from("add"))),
        Instruction::GetLocal(0),
        Instruction::GetLocal(1),
        Instruction::FunctionCall(2),
        Instruction::Return,
    ]);
    
    program.add_function(String::from("add"), 2, vec![
        Instruction::GetLocal(0),
        Instruction::GetLocal(1),
        Instruction::Add,
        Instruction::Return
    ]);

    let mut vm = Vm::default();
    vm.load_program(program);
    let _ = vm.run(None);
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("function_call", |b| b.iter(|| test_function_call()));
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);