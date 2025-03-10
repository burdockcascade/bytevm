use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::Vm;

#[test]
fn test_add_and_compare() {
    let program = Program {
        instructions: vec![
            // Set local 0
            Instruction::PushInteger(1),
            Instruction::SetLocal(0),
            
            // Set local 1
            Instruction::PushInteger(2),
            Instruction::SetLocal(1),
            
            // Get local 0
            Instruction::GetLocal(0),
            Instruction::Return
        ],
        ..Default::default()
    };

    let vm = Vm::new(program);
    assert_eq!(vm.run(None).unwrap().result.unwrap(), Variant::Integer(1));

}

#[test]
fn test_overwrite_local() {
    let program = Program {
        instructions: vec![
            // Set local 0
            Instruction::PushInteger(1),
            Instruction::SetLocal(0),

            // Set local 0
            Instruction::PushInteger(2),
            Instruction::SetLocal(0),

            // Get local 0
            Instruction::GetLocal(0),
            Instruction::Return
        ],
        ..Default::default()
    };

    let vm = Vm::new(program);
    assert_eq!(vm.run(None).unwrap().result.unwrap(), Variant::Integer(2));
}
