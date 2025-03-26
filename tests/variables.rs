use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::runtime::Vm;

#[test]
fn test_get_variable() {
    let program = Program {
        instructions: vec![
            // Set local 0
            Instruction::Push(Variant::Integer(1)),
            Instruction::SetLocal(0),
            
            // Set local 1
            Instruction::Push(Variant::Integer(2)),
            Instruction::SetLocal(1),
            
            // Get local 0
            Instruction::GetLocal(0),
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
fn test_overwrite_local() {
    let program = Program {
        instructions: vec![
            // Set local 0
            Instruction::Push(Variant::Integer(1)),
            Instruction::SetLocal(0),

            // Set local 0
            Instruction::Push(Variant::Integer(2)),
            Instruction::SetLocal(0),

            // Get local 0
            Instruction::GetLocal(0),
            Instruction::Return
        ],
        ..Default::default()
    };

    let mut vm = Vm::default();
    vm.load_program(program);
    let result = vm.run(None).unwrap().result.unwrap();

    assert_eq!(result, Variant::Integer(2));
}
