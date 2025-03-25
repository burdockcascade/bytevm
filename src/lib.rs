use crate::program::Program;
use crate::vm::{Vm, VmError, VmExecutionResult, VmOptions};

pub mod variant;
pub mod vm;
mod stack;
pub mod program;

pub fn run(program: Program) -> Result<VmExecutionResult, VmError> {
    Vm::new(program, VmOptions::default()).run()
}
