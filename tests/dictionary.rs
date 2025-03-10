use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_create_dictionary() {

    let key1 = String::from("key1");
    let key2 = 4;
    let key3 = true;

    let program = Program {
        instructions: vec![

            // first entry, "key1" = 1
            Instruction::PushString(key1.clone()),
            Instruction::PushInteger(1),

            // second entry, 7 = 2
            Instruction::PushInteger(key2),
            Instruction::PushInteger(2),

            // third entry. true = 3
            Instruction::PushBoolean(key3),
            Instruction::PushInteger(3),

            // Create dictionary
            Instruction::CreateDictionary(3),

            // Return
            Instruction::Return
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
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
            Instruction::PushString(key1.clone()),
            Instruction::PushInteger(1),
            Instruction::PushString(key2.clone()),
            Instruction::PushInteger(2),
            Instruction::PushString(key3.clone()),
            Instruction::PushInteger(3),
            Instruction::CreateDictionary(3),

            // Get the value of key2
            Instruction::PushString(key1.clone()),
            Instruction::GetDictionaryItem,

            // Return the value
            Instruction::Return
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(1));
}


