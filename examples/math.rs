use log::LevelFilter;
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use bytevm::prelude::{BlockEncoder, FunctionBuilder, ProgramBuilder, Vm};

fn main() {

    TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Logger error");

    
    let mut program  = ProgramBuilder::default();

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
    let result = vm.run(None, None).unwrap();

    println!("Time taken: {:?}", result.run_time.as_secs_f64());

}