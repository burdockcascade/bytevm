use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::runtime::Vm;

#[test]
fn test_add_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Add,
            Instruction::Push(Variant::Integer(3)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));

}

#[test]
fn test_add_and_compare_false() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Add,
            Instruction::Push(Variant::Integer(4)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(false));
}

#[test]
fn test_sub_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(5)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::Sub,
            Instruction::Push(Variant::Integer(3)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_mul_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::Mul,
            Instruction::Push(Variant::Integer(6)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_div_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(6)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::Div,
            Instruction::Push(Variant::Integer(2)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_mod_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(7)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::Mod,
            Instruction::Push(Variant::Integer(1)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_pow_and_compare() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(3)),
            Instruction::Pow,
            Instruction::Push(Variant::Integer(8)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_negate() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(2)),
            Instruction::Negate,
            Instruction::Push(Variant::Integer(-2)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_less_than() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::LessThan,
            Instruction::Push(Variant::Boolean(true)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_less_than_or_equal() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(1)),
            Instruction::LessEqual,
            Instruction::Push(Variant::Boolean(true)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_greater_than() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(2)),
            Instruction::Push(Variant::Integer(1)),
            Instruction::GreaterThan,
            Instruction::Push(Variant::Boolean(true)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };
    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_greater_than_or_equal() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(1)),
            Instruction::GreaterEqual,
            Instruction::Push(Variant::Boolean(true)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}

#[test]
fn test_not_equal() {
    let program = Program {
        instructions: vec![
            Instruction::Push(Variant::Integer(1)),
            Instruction::Push(Variant::Integer(2)),
            Instruction::NotEqual,
            Instruction::Push(Variant::Boolean(true)),
            Instruction::Equal,
            Instruction::Halt
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Boolean(true));
}