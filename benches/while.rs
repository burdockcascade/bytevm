use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bytevm::program::{GlobalEntry, Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;

// Your function to benchmark
fn fibonacci(input: i64) -> i64 {

    let program = Program {
        instructions: vec![
            // Set i = 0
            Instruction::Push(Variant::Integer(0)),
            Instruction::SetLocal(0),

            // Set max = 10
            Instruction::Push(Variant::Integer(input)),
            Instruction::SetLocal(1),

            // while i < max
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::LessThan,

            // Jump if false
            Instruction::JumpIfFalse(13),

            // Increment i
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Add,
            Instruction::SetLocal(0),

            // Jump to the beginning
            Instruction::Jump(4),

            // Return i
            Instruction::GetLocal(0),
            Instruction::Return

        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    vm.run(None).unwrap().result.unwrap().into()
}

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("while");
    for i in [10, 100, 1000].iter() {
        group.bench_function(criterion::BenchmarkId::from_parameter(i), |b| {
            b.iter(|| fibonacci(black_box(*i)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);