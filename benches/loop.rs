use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bytevm::prelude::*;

// Your function to benchmark
fn loopy(input: i64) -> i64 {

    let mut program  = ProgramBuilder::default();

    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(0)
        .body(BlockEncoder::default()
            // Declare a local variable to hold the input
            .declare_local("i")
            .push_integer(input)
            .set_local("i")

            // Loop until i is 0
            .add_label("loop_start")
            .push_integer(0)
            .get_local("i")
            .less_than()
            .jump_if_false("loop_end")
            .get_local("i")
            .push_integer(1)
            .sub()
            .set_local("i")
            .jump("loop_start")
            .add_label("loop_end")

            // Return the result
            .get_local("i")
            .return_value()
        )
        .build()
    );
    
    let mut vm = Vm::default();
    vm.load_program(program.build());
    vm.run(None, None).unwrap().result.unwrap().into()
}

fn bench_fibonacci(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop");
    for i in [10, 100, 1000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_function(criterion::BenchmarkId::from_parameter(i), |b| {
            b.iter(|| loopy(black_box(*i)));
        });
    }
    group.finish();
}

criterion_group!(benches, bench_fibonacci);
criterion_main!(benches);