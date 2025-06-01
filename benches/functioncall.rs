use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bytevm::prelude::*;

// Your function to benchmark
fn test_function_call() {
    let mut program = Program::builder();

    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(0)
        .body(
            BlockEncoder::default()
                // Call the add function with 1 and 2
                .push_integer(1)
                .push_integer(2)
                .call_function_by_name("add")
                // Return the result
                .return_value()
        )
        .build()
    );

    program.add_function(FunctionBuilder::default()
        .name("add")
        .arity(2)
        .body(
            BlockEncoder::default()
                .declare_local("a")
                .declare_local("b")
                .get_local("a")
                .get_local("b")
                .add()
                .return_value()
        )
        .build()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let _ = vm.run(None, None);
}

fn bench_fibonacci(c: &mut Criterion) {
    c.bench_function("function_call", |b| b.iter(|| test_function_call()));
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);