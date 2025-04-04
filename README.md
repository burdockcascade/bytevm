[![tests][1]][2] [![docs][5]][6] [![crates][7]][8] [![license-mit-badge][]](#license)

[1]: https://github.com/burdockcascade/bytevm/actions/workflows/test.yml/badge.svg?branch=master
[2]: https://github.com/burdockcascade/bytevm/actions/workflows/test.yml
[5]: https://docs.rs/bytevm/badge.svg
[6]: https://docs.rs/bytevm
[7]: https://img.shields.io/crates/v/bytevm.svg
[8]: https://crates.io/crates/bytevm
[license-mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg

# ByteVM
ByteVM is a bytecode virtual machine written in Rust. It is designed to execute programs written in a custom bytecode format. The VM is designed to be fast, efficient, and easy to use. It is intended to be used as a platform for implementing programming languages, interpreters, and compilers.

## Status
ByteVM is currently in the early stages of development. The VM is not yet feature complete, and the API is subject to change. The VM is not yet suitable for production use.

## Examples
```rust
let mut program = Program::default();

program.add_function(String::main("main"), 0, vec![
    Instruction::Push(Variant::Integer(1)),
    Instruction::SetLocal(0),
    Instruction::Push(Variant::Integer(2)),
    Instruction::SetLocal(1),
    Instruction::Push(Variant::GlobalReference(String::from("add"))),
    Instruction::GetLocal(0),
    Instruction::GetLocal(1),
    Instruction::FunctionCall(2)
]);

program.add_function(String::from("add"), 2, vec![
    Instruction::GetLocal(0),
    Instruction::GetLocal(1),
    Instruction::Add,
    Instruction::Return
]);

let mut vm = Vm::default();
vm.load_program(program);
vm.run(None);
```

## License
ByteVM is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.