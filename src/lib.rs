use crate::program::Program;
use crate::vm::{VmError, VmExecutionResult};

pub mod variant;
pub mod vm;
mod stack;
pub mod program;

pub fn run(program: Program) -> Result<VmExecutionResult, VmError> {
    let vm = vm::Vm::new(program);
    vm.run(None)
}
