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
let mut program = Program::builder();

program.add_function(String::from("main"), 0, BlockEncoder::default()

    // Declare a local variable to hold the input
    .declare_local("n")
    .push_integer(10)
    .set_local("n")
    
    // Call the fib function
    .push_function_reference("fib")
    .get_local("n")
    .function_call(1)
    
    // Return the result
    .return_value()
    
    // encode
    .encode()
);

program.add_function(String::from("fib"), 1, BlockEncoder::default()

    // Declare local variables for the Fibonacci function
    .declare_local("n")
    
    // if n <= 1 then return n
    .get_local("n")
    .push_integer(1)
    .less_than_or_equal()
    .jump_if_false("end")
    .get_local("n")
    .return_value()
    .add_label("end")
    
    // fib(n - 1)
    .push_function_reference("fib")
    .get_local("n")
    .push_integer(1)
    .sub()
    .function_call(1)
    
    // fib(n - 2)
    .push_function_reference("fib")
    .get_local("n")
    .push_integer(2)
    .sub()
    .function_call(1)
    
    // add the results of fib(n-1) and fib(n-2)
    .add()
    
    // return the result
    .return_value()
    
    // encode
    .encode()
);

let mut vm = Vm::default();
vm.load_program(program.build());
vm.run(None);
```

## License
ByteVM is distributed under the terms of the MIT license. See [LICENSE](LICENSE) for details.