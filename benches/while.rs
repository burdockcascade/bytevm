use bytevm::program::{Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Your function to benchmark
fn do_while(input: i64) -> i64 {

    let mut program = Program::default();
    
    program.add_function(String::from("main"), 1, vec![
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
        Instruction::Return,
    ]);

    let mut vm = Vm::default();
    vm.load_program(program);
    vm.run(None).unwrap().result.unwrap().into()
}

fn bench_while_loop(c: &mut Criterion) {
    let mut group = c.benchmark_group("while");
    for i in [10, 100, 1000].iter() {
        group.bench_function(criterion::BenchmarkId::from_parameter(i), |b| {
            b.iter(|| do_while(black_box(*i)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_while_loop);
criterion_main!(benches);