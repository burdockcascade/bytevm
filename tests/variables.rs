use bytevm::prelude::*;

#[test]
fn test_get_variable() {
    let mut program = Program::builder();
    program.add_function("main", 1, BlockEncoder::default()
        .declare_local("a")
        .push_integer(1)
        .set_local("a")
        .get_local("a")
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(1));

}

#[test]
fn test_overwrite_local() {
    let mut program = Program::builder();
    program.add_function("main", 1, BlockEncoder::default()
        .declare_local("a")
        .push_integer(1)
        .set_local("a")
        .push_integer(2)
        .set_local("a")
        .get_local("a")
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(2));
}
