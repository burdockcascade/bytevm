use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_add_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::Add,
            Instruction::PushInteger(3),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));

}

#[test]
fn test_add_and_compare_false() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::Add,
            Instruction::PushInteger(4),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(false));
}

#[test]
fn test_sub_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(5),
            Instruction::PushInteger(2),
            Instruction::Sub,
            Instruction::PushInteger(3),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_mul_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(2),
            Instruction::PushInteger(3),
            Instruction::Mul,
            Instruction::PushInteger(6),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_div_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(6),
            Instruction::PushInteger(3),
            Instruction::Div,
            Instruction::PushInteger(2),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_mod_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(7),
            Instruction::PushInteger(3),
            Instruction::Mod,
            Instruction::PushInteger(1),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_pow_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(2),
            Instruction::PushInteger(3),
            Instruction::Pow,
            Instruction::PushInteger(8),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_negate() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(2),
            Instruction::Negate,
            Instruction::PushInteger(-2),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_less_than() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::LessThan,
            Instruction::PushBoolean(true),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_less_than_or_equal() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::PushInteger(1),
            Instruction::LessEqual,
            Instruction::PushBoolean(true),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_greater_than() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(2),
            Instruction::PushInteger(1),
            Instruction::GreaterThan,
            Instruction::PushBoolean(true),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_greater_than_or_equal() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::PushInteger(1),
            Instruction::GreaterEqual,
            Instruction::PushBoolean(true),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_not_equal() {
    let program = Program {
        instructions: vec![
            Instruction::PushInteger(1),
            Instruction::PushInteger(2),
            Instruction::NotEqual,
            Instruction::PushBoolean(true),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Boolean(true));
}