use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use bytevm::prelude::{BlockEncoder, FunctionBuilder, ProgramBuilder, Vm};

fn main() {

    TermLogger::init(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Logger error");
    
    let input = 35;

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
    let result = vm.run(None, None).unwrap();

    println!("Time taken: {:?}", result.run_time.as_secs_f64());

}