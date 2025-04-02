use bytevm::program::{Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Your function to benchmark
fn fibonacci(input: i64) -> i64 {

    let mut program = Program::default();
    
    program.add_main_function(vec![
        Instruction::Push(Variant::Integer(input)),
        Instruction::SetLocal(0),
        Instruction::Push(Variant::Identifier(String::from("fib"))),
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
        Instruction::Push(Variant::Identifier(String::from("fib"))),
        Instruction::GetLocal(0),
        Instruction::Push(Variant::Integer(1)),
        Instruction::Sub,
        Instruction::FunctionCall(1),
        Instruction::Push(Variant::Identifier(String::from("fib"))),
        Instruction::GetLocal(0),
        Instruction::Push(Variant::Integer(2)),
        Instruction::Sub,
        Instruction::FunctionCall(1),
        Instruction::Add,
        Instruction::Return,
    ]);
    
    let mut vm = Vm::default();
    vm.load_program(program);
    vm.run(None).unwrap().result.unwrap().into()
}

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("fibonacci");
    for i in [20, 21, 22].iter() {
        group.bench_function(criterion::BenchmarkId::from_parameter(i), |b| {
            b.iter(|| fibonacci(black_box(*i)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);