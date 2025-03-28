use std::collections::HashMap;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bytevm::program::{GlobalEntry, Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;

// Your function to benchmark
fn fibonacci(input: i64) -> i64 {

    let mut globals = HashMap::new();
    globals.insert(String::from("main"), GlobalEntry::UserDefinedFunction {
        address: 0,
        arity: 0
    });

    let fib_func_name = String::from("fib");
    globals.insert(fib_func_name.clone(), GlobalEntry::UserDefinedFunction {
        address: 4,
        arity: 2
    });

    let program = Program {
        globals,
        instructions: vec![

            // main
            Instruction::Push(Variant::Identifier(fib_func_name.clone())),
            Instruction::Push(Variant::Integer(input)),
            Instruction::FunctionCall(1),
            Instruction::Return,

            // fib
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

            // Halt
            Instruction::Halt
        ]
    };

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