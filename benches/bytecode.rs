use bytevm::program::{Instruction, Program};
use bytevm::runtime::Vm;
use bytevm::variant::Variant;
use criterion::{criterion_group, criterion_main, Criterion};

// Your function to benchmark
fn test_variable_set() {
    let mut program = Program::default();

    program.add_function(String::from("main"), 0, vec![
        Instruction::Push(Variant::Integer(1)),
        Instruction::SetLocal(0),
        Instruction::Return,
    ]);

    let mut vm = Vm::default();
    vm.load_program(program);
    let _ = vm.run(None);
}

fn test_variable_get() {
    let mut program = Program::default();

    program.add_function(String::from("main"), 0, vec![
        Instruction::Push(Variant::Integer(1)),
        Instruction::SetLocal(0),
        Instruction::GetLocal(0),
        Instruction::Return,
    ]);

    let mut vm = Vm::default();
    vm.load_program(program);
    let _ = vm.run(None);
}

fn test_variable_overwrite() {
    let mut program = Program::default();

    program.add_function(String::from("main"), 0, vec![
        Instruction::Push(Variant::Integer(1)),
        Instruction::SetLocal(0),
        Instruction::Push(Variant::Integer(2)),
        Instruction::SetLocal(0),
        Instruction::GetLocal(0),
        Instruction::Return,
    ]);

    let mut vm = Vm::default();
    vm.load_program(program);
    let _ = vm.run(None);
}

fn bench_instructions(c: &mut Criterion) {
    c.bench_function("test_variable_set", |b| b.iter(|| test_variable_set()));
    c.bench_function("test_variable_get", |b| b.iter(|| test_variable_get()));
    c.bench_function("test_variable_overwrite", |b| b.iter(|| test_variable_overwrite()));
}

criterion_group!(benches, bench_instructions);
criterion_main!(benches);