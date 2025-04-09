use bytevm::prelude::*;

#[test]
fn test_add_and_compare() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(1)
        .push_integer(2)
        .add()
        .push_integer(3)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));

}

#[test]
fn test_add_and_compare_false() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(1)
        .push_integer(2)
        .add()
        .push_integer(4)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(false));
}

#[test]
fn test_sub_and_compare() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(5)
        .push_integer(2)
        .sub()
        .push_integer(3)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_mul_and_compare() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(2)
        .push_integer(3)
        .mul()
        .push_integer(6)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_div_and_compare() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(6)
        .push_integer(3)
        .div()
        .push_integer(2)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_mod_and_compare() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(7)
        .push_integer(3)
        .modulus()
        .push_integer(1)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_pow_and_compare() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(2)
        .push_integer(3)
        .pow()
        .push_integer(8)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_negate() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(2)
        .negate()
        .push_integer(-2)
        .equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_less_than() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(1)
        .push_integer(2)
        .less_than()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_less_than_or_equal() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(1)
        .push_integer(1)
        .less_than_or_equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_greater_than() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(2)
        .push_integer(1)
        .greater_than()
        .return_value()
        .encode()
    );
    
    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_greater_than_or_equal() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(1)
        .push_integer(1)
        .greater_than_or_equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_not_equal() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(1)
        .push_integer(2)
        .not_equal()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}