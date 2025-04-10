use bytevm::prelude::*;

#[test]
fn test_user_defined_function() {
    
    let mut program = Program::builder();

    program.add_function("main", 1, BlockEncoder::default()
        .push_function_reference("add")
        .push_integer(1)
        .push_integer(2)
        .function_call(2)
        .return_value()
        .encode(),
    );
    
    program.add_function("add", 2,  BlockEncoder::default()
        .declare_local("a")
        .declare_local("b")
        .get_local("a")
        .get_local("b")
        .add()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}

#[test]
fn test_builtin_function() {
    
    let mut program = Program::builder();
    program.add_function("main", 1, BlockEncoder::default()
        .push_function_reference("native_add")
        .push_integer(1)
        .push_integer(2)
        .function_call(2)
        .return_value()
        .encode(),
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