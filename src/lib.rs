mod variant;
mod runtime;
mod program;
mod builder;

pub mod prelude {
    pub use crate::builder::BlockEncoder;
    pub use crate::builder::ProgramBuilder;
    pub use crate::builder::FunctionBuilder;
    pub use crate::program::Program;
    pub use crate::runtime::Vm;
    pub use crate::runtime::VmError;
    pub use crate::runtime::VmExecutionResult;
    pub use crate::variant::Variant;
}