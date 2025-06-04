use criterion::{black_box, criterion_group, criterion_main, Criterion};
use bytevm::prelude::*;

// Your function to benchmark
fn fibonacci(input: i64) -> i64 {

    let mut program  = ProgramBuilder::default();

    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(0)
        .body(BlockEncoder::default()
            // Declare a local variable to hold the input
            .declare_local("n")
            .push_integer(input)
            .set_local("n")

            // Call the fib function
            .get_local("n")
            .call_function_by_name("fib")

            // Return the result
            .return_value()
        )
        .build()
    );

    program.add_function(FunctionBuilder::default()
        .name("fib")
        .arity(1)
        .body(BlockEncoder::default()
            // Declare local variables for the Fibonacci function
            .declare_local("n")

            // if n <= 1 then return n
            .get_local("n")
            .push_integer(1)
            .less_than_or_equal()
            .jump_if_false("end")
            .get_local("n")
            .return_value()
            .add_label("end")

            // fib(n - 1)
            .get_local("n")
            .push_integer(1)
            .sub()
            .call_function_by_name("fib")

            // fib(n - 2)
            .get_local("n")
            .push_integer(2)
            .sub()
            .call_function_by_name("fib")

            // add the results of fib(n-1) and fib(n-2)
            .add()

            // return the result
            .return_value())
        .build()
    );
    
    let mut vm = Vm::default();
    vm.load_program(program.build());
    vm.run(None, None).unwrap().result.unwrap().into()
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