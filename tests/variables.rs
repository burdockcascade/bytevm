use bytevm::program::{Instruction, Program};
use bytevm::variant::Variant;
use bytevm::vm::{Vm, VmOptions};

#[test]
fn test_get_variable() {
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

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(1));

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

    let result = Vm::new(program, VmOptions::default()).run(None).unwrap().result.unwrap();
    assert_eq!(result, Variant::Integer(2));
}
