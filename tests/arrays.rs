use bytevm::prelude::*;

#[test]
fn test_create_array() {

    let mut program = Program::builder();

    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(1)
        .body(
            BlockEncoder::default()
                // Create an array with 3 elements
                .push_integer(1)
                .push_integer(2)
                .push_integer(3)
                .create_array(3)

                // Return the array
                .return_value()
        )
        .build()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
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

    let mut program = Program::builder();
    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(1)
        .body(
            BlockEncoder::default()
                // Create an array with 3 elements
                .push_integer(1)
                .push_integer(2)
                .push_integer(3)
                .create_array(3)

                // Return the second element
                .push_integer(1)
                .get_array_item()
                .return_value()
        )
        .build()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(2));
}

#[test]
fn test_set_array_element() {

    let mut program = Program::builder();

    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(1)
        .body(
            BlockEncoder::default()
                // Create an array with 3 elements
                .push_integer(1)
                .push_integer(2)
                .push_integer(3)
                .create_array(3)

                // Set the second element to 4
                .push_integer(1)
                .push_integer(4)
                .set_array_item()

                // Return the second element
                .push_integer(1)
                .get_array_item()
                .return_value()
        )
        .build()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(4));
}

#[test]
fn test_get_array_length() {
    let mut program = Program::builder();
    program.add_function(FunctionBuilder::default()
        .name("main")
        .arity(1)
        .body(
            BlockEncoder::default()
                // Create an array with 3 elements
                .push_integer(1)
                .push_integer(2)
                .push_integer(3)
                .create_array(3)

                // Get the length of the array
                .get_array_length()
                .return_value()
        )
        .build()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(3));
}