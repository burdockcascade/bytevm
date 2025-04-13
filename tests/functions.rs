use bytevm::prelude::*;

#[test]
fn test_user_defined_function() {
    
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
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}

#[test]
fn test_builtin_function() {
    
    let mut program = Program::builder();
    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(0)
        .body(
            BlockEncoder::default()
                // Call the add function with 1 and 2
                .push_integer(1)
                .push_integer(2)
                .call_function_by_name("native_add")
                // Return the result
                .return_value()
        )
        .build()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    vm.register_native_function(String::from("native_add"), |args: Vec<Variant>| {
        let a = args[0].clone();
        let b = args[1].clone();
        Some(a + b)
    });

    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(3));
}