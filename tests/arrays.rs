use bytevm::builder::BlockEncoder;
use bytevm::program::Program;
use bytevm::runtime::Vm;
use bytevm::variant::Variant;

#[test]
fn test_create_array() {

    let mut program = Program::default();

    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .push_integer(1)
        .push_integer(2)
        .push_integer(3)
        .create_array(3)
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    match result {
        Variant::Array(array) => {
            assert_eq!(array.borrow().len(), 3);
            assert_eq!(array.borrow().get(0), Some(&Variant::Integer(1)));
            assert_eq!(array.borrow().get(1), Some(&Variant::Integer(2)));
            assert_eq!(array.borrow().get(2), Some(&Variant::Integer(3)));
        }
        _ => panic!("Expected array")
    }
}

#[test]
fn test_get_array_element() {

    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .declare_local("arr")
        .push_integer(1)
        .push_integer(2)
        .push_integer(3)
        .create_array(3)
        .set_local("arr")
        .get_local("arr")
        .push_integer(1)
        .get_array_item()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(2));
}

#[test]
fn test_set_array_element() {

    let mut program = Program::default();

    program.add_function(String::from("main"), 1, BlockEncoder::default()

        // Create an array with 3 elements
        .declare_local("arr")
        .push_integer(1)
        .push_integer(2)
        .push_integer(3)
        .create_array(3)
        .set_local("arr")

        // Set the second element to 4
        .get_local("arr")
        .push_integer(1)
        .push_integer(4)
        .set_array_item()

        // Return the second element
        .get_local("arr")
        .push_integer(1)
        .get_array_item()
        .return_value()

        // Encode the program
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(4));
}

#[test]
fn test_get_array_length() {
    let mut program = Program::default();
    program.add_function(String::from("main"), 1, BlockEncoder::default()
        .declare_local("arr")
        .push_integer(1)
        .push_integer(2)
        .push_integer(3)
        .create_array(3)
        .set_local("arr")
        .get_local("arr")
        .get_array_length()
        .return_value()
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(3));
}