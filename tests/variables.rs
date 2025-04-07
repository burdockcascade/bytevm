use bytevm::builder::BlockEncoder;
use bytevm::program::Program;
use bytevm::runtime::Vm;
use bytevm::variant::Variant;

#[test]
fn test_get_variable() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .declare_local("a")
        .push_integer(1)
        .set_local("a")
        .get_local("a")
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(1));

}

#[test]
fn test_overwrite_local() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
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
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(2));
}
