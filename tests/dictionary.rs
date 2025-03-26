use std::cell::RefCell;
use std::rc::Rc;
use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::runtime::Vm;

#[test]
fn test_create_dictionary() {

    let key1 = String::from("key1");
    let key2 = 4;
    let key3 = true;

    let program = Program {
        instructions: vec![

            // first entry, "key1" = 1
            Instruction::Push(Variant::String(key1.clone())),
            Instruction::Push(Variant::Integer(1)),

            // second entry, 7 = 2
            Instruction::Push(Variant::Integer(key2)),
            Instruction::Push(Variant::Integer(2)),

            // third entry. true = 3
            Instruction::Push(Variant::Boolean(key3)),
            Instruction::Push(Variant::Integer(3)),

            // Create dictionary
            Instruction::CreateDictionary(3),

            // Return
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
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

    let program = Program {
        instructions: vec![
            // Create a dictionary with 3 key-value pairs
            Instruction::Push(Variant::String(key1.clone())),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::String(key2.clone())),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::String(key3.clone())),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateDictionary(3),

            // Get the value of key2
            Instruction::Push(Variant::String(key1)),
            Instruction::GetDictionaryItem,

            // Return the value
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(1));
}

#[test]
fn test_get_dictionary_keys() {

    let key1 = String::from("key1");
    let key2 = String::from("key2");
    let key3 = String::from("key3");

    let program = Program {
        instructions: vec![
            // Create a dictionary with 3 key-value pairs
            Instruction::Push(Variant::String(key1.clone())),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::String(key2.clone())),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::String(key3.clone())),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateDictionary(3),

            // Return the value
            Instruction::GetDictionaryKeys,
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
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
