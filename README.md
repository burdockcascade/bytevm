[![CI](https://github.com/burdockcascade/bytevm/actions/workflows/ci.yml/badge.svg)](https://github.com/burdockcascade/bytevm/actions/workflows/ci.yml)

# ByteVM
ByteVM is a bytecode virtual machine written in Rust. It is designed to execute programs written in a custom bytecode format. The VM is designed to be fast, efficient, and easy to use. It is intended to be used as a platform for implementing programming languages, interpreters, and compilers.

## Examples
Here is an example of a simple program that adds two numbers together:
```rust
let program = Program {
    instructions: vec![
        Instruction::PushInteger(1),
        Instruction::PushInteger(2),
        Instruction::Add,
        Instruction::PushInteger(3),
        Instruction::Equals,
        Instruction::Return
    ],
    ..Default::default()
};
```