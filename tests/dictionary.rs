use bytevm::prelude::*;

#[test]
fn test_create_dictionary() {

    let key1 = String::from("key1");
    let key2 = 4;
    let key3 = true;

    let mut program = Program::builder();
    program.add_function("main", 1, BlockEncoder::default()
        // Create a dictionary with 3 key-value pairs
        .push_string(key1.clone())
        .push_integer(1)
        .push_integer(key2.clone())
        .push_integer(2)
        .push_boolean(key3.clone())
        .push_integer(3)
        .create_dictionary(3)

        // Return the value
        .return_value()

        // encode the program
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();

    match result {
        Variant::Dictionary(array) => {
            assert_eq!(array.borrow().len(), 3);
            assert_eq!(array.borrow().get(&Variant::String(key1)), Some(&Variant::Integer(1)));
            assert_eq!(array.borrow().get(&Variant::Integer(key2)), Some(&Variant::Integer(2)));
            assert_eq!(array.borrow().get(&Variant::Boolean(key3)), Some(&Variant::Integer(3)));
        }
        _ => panic!("Expected table")
    }
}

#[test]
fn test_get_dictionary_item() {

    let key1 = String::from("key1");
    let key2 = String::from("key2");
    let key3 = String::from("key3");

    let mut program = Program::builder();
    program.add_function("main", 1, BlockEncoder::default()
        // Create a dictionary with 3 key-value pairs
        .push_string(key1.clone())
        .push_integer(1)
        .push_string(key2.clone())
        .push_integer(2)
        .push_string(key3.clone())
        .push_integer(3)
        .create_dictionary(3)

        // Get the value for key2
        .push_string(key1)
        .get_dictionary_item()
        .return_value()

        // encode the program
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(1));
}

#[test]
fn test_get_dictionary_keys() {

    let key1 = String::from("key1");
    let key2 = String::from("key2");
    let key3 = String::from("key3");

    let mut program = Program::builder();
    program.add_function("main", 1, BlockEncoder::default()
        // Create a dictionary with 3 key-value pairs
        .push_string(key1.clone())
        .push_integer(1)
        .push_string(key2.clone())
        .push_integer(2)
        .push_string(key3.clone())
        .push_integer(3)
        .create_dictionary(3)

        // Get the keys of the dictionary
        .get_dictionary_keys()
        .return_value()

        // encode the program
        .encode()
    );

    let mut vm = Vm::default();
    vm.load_program(program.build());
    let result = vm.run(None).unwrap().result.unwrap();

    // find key1 in result
    match result {
        Variant::Array(array) => {
            assert_eq!(array.borrow().len(), 3);
            assert!(array.borrow().contains(&Variant::String(key1)));
            assert!(array.borrow().contains(&Variant::String(key2)));
            assert!(array.borrow().contains(&Variant::String(key3)));
        }
        _ => panic!("Expected array")
    }

}