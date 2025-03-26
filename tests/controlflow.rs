use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::runtime::Vm;

#[test]
fn test_jump() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Jump(4),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Return,
            Instruction::Push(Variant::Integer(3)),
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}

#[test]
fn test_jump_if_false() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Boolean(false)),
            Instruction::JumpIfFalse(4),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Return,
            Instruction::Push(Variant::Integer(2)),
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(2));
}

#[test]
fn test_dont_jump_if_true() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Boolean(true)),
            Instruction::JumpIfFalse(4),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Return,
            Instruction::Push(Variant::Integer(2)),
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
fn test_for_while_loop() {
    let program = Program {
        instructions: vec![
            // Set i = 0
            Instruction::Push(Variant::Integer(0)),
            Instruction::SetLocal(0),

            // Set max = 10
            Instruction::Push(Variant::Integer(10)),
            Instruction::SetLocal(1),

            // while i < max
            Instruction::GetLocal(0),
            Instruction::GetLocal(1),
            Instruction::LessThan,

            // Jump if false
            Instruction::JumpIfFalse(13),

            // Increment i
            Instruction::GetLocal(0),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Add,
            Instruction::SetLocal(0),

            // Jump to the beginning
            Instruction::Jump(4),

            // Return i
            Instruction::GetLocal(0),
            Instruction::Return

        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(10));
}

#[test]
fn test_foreach_array_loop() {

    let array = 0;
    let index = 1;
    let max = 2;
    let item = 3;

    let program = Program {
        instructions: vec![
            // Create an array with 3 elements
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateArray(3),
            Instruction::SetLocal(array),

            // Set i = 0
            Instruction::Push(Variant::Integer(0)),
            Instruction::SetLocal(index),

            // Get the length of the array and store it in max
            Instruction::GetLocal(array),
            Instruction::GetArrayLength,
            Instruction::SetLocal(max),

            // while i < max
            Instruction::GetLocal(index), // 10
            Instruction::GetLocal(max),
            Instruction::LessThan,

            // Jump if false
            Instruction::JumpIfFalse(23), // 13

            // Get the array
            Instruction::GetLocal(array),
            Instruction::GetLocal(index),
            Instruction::GetArrayItem,
            Instruction::SetLocal(item),

            // Increment i
            Instruction::GetLocal(1),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Add,
            Instruction::SetLocal(1),

            // Jump to the beginning
            Instruction::Jump(10),

            // Return i
            Instruction::GetLocal(index),
            Instruction::Return // 23

        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}

#[test]
fn test_foreach_dictionary_loop() {

    //TermLogger::init(LevelFilter::Trace, Config::default(), TerminalMode::Mixed, ColorChoice::Auto).expect("Logger error");


    let dictionary = 0;
    let keys = 1;
    let index = 2;
    let max = 3;
    let item = 4;

    let program = Program {
        instructions: vec![
            // Create a dictionary with 3 key-value pairs
            Instruction::Push(Variant::String(String::from("key1"))),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::String(String::from("key2"))),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::String(String::from("key3"))),
            Instruction::Push(Variant::Integer(3)),
            Instruction::CreateDictionary(3),
            Instruction::SetLocal(dictionary),

            // Get the keys of the dictionary
            Instruction::GetLocal(dictionary),
            Instruction::GetDictionaryKeys,
            Instruction::SetLocal(keys),

            // Set i = 0
            Instruction::Push(Variant::Integer(0)), // 11
            Instruction::SetLocal(index),

            // Get the length of the dictionary and store it in max
            Instruction::GetLocal(keys),
            Instruction::GetArrayLength,
            Instruction::SetLocal(max),

            // while i < max
            Instruction::GetLocal(index), // 16
            Instruction::GetLocal(max),
            Instruction::LessThan,

            // Jump if false
            Instruction::JumpIfFalse(31),

            // Get the dictionary
            Instruction::GetLocal(dictionary),

            // put dictionary item on stack
            Instruction::GetLocal(keys),
            Instruction::GetLocal(index),
            Instruction::GetArrayItem,

            // get dictionary item
            Instruction::GetDictionaryItem,
            Instruction::SetLocal(item),

            // Increment i
            Instruction::GetLocal(index),
            Instruction::Push(Variant::Integer(1)),
            Instruction::Add,
            Instruction::SetLocal(index),

            // Jump to the beginning
            Instruction::Jump(16),

            // Return i
            Instruction::GetLocal(index), // 31
            Instruction::Return

        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(3));
}
