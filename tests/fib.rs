use bytevm::builder::BlockEncoder;
use bytevm::program::Program;
use bytevm::runtime::Vm;
use bytevm::variant::Variant;

#[test]
fn test_fib() {

    let input = 20;
    let expected_result = fib(input);

    let mut program  = Program::default();

    program.add_function(String::from("main"), 0, BlockEncoder::default()

        // Declare a local variable to hold the input
        .declare_local("n")
        .push_integer(input)
        .set_local("n")

        // Call the fib function
        .push_function_pointer(1)
        .get_local("n")
        .function_call(1)

        // Return the result
        .return_value()

        // encode
        .encode()
    );
    
    program.add_function(String::from("fib"), 2, BlockEncoder::default()
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
        .push_function_pointer(1)
        .get_local("n")
        .push_integer(1)
        .sub()
        .function_call(1)

        // fib(n - 2)
        .push_function_pointer(1)
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
    vm.load_program(program);
    let result = vm.run(None).unwrap();
    
    assert_eq!(result.result.unwrap(), Variant::Integer(expected_result));
}

fn fib(n: i64) -> i64 {
    if n <= 1 {
        return n;
    }
    fib(n - 1) + fib(n - 2)
}