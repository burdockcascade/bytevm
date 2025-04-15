use bytevm::prelude::*;

#[test]
fn test_jumps() {
    
    let target = 1_000_000;

    let mut program = Program::builder();
    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(0)
        .body(
            BlockEncoder::default()

                // create a variable to hold the result
                .declare_local("i")
                .push_integer(0)
                .set_local("i")

                // create a variable to hold the max value
                .declare_local("max")
                .push_integer(target)
                .set_local("max")

                // start of the loop
                .add_label("start")

                // check if i < max
                .get_local("i")
                .get_local("max")
                .less_than()
                .jump_if_false("end")

                // increment i
                .get_local("i")
                .push_integer(1)
                .add()
                .set_local("i")

                // jump to the start of the loop
                .jump("start")

                // end of the loop
                .add_label("end")

                // return the result
                .get_local("i")
                .return_value()
        )
        .build()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap();

    assert_eq!(result.result.unwrap(), Variant::Integer(target));
    println!("Elapsed time: {:?}", result.run_time.as_secs_f64());
}