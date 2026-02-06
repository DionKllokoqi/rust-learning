fn main() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    /* The error indicates that Rust expected a bool but got an integer.
    Unlike languages such as Ruby and JavaScript,
    Rust will not automatically try to convert non-Boolean types to a Boolean.
    The below would give a compilation error. */
    // let number = 3;
    //
    // if number {
    //     println!("number was three");
    // }

    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable.
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // if we assign a value based on an if/else expression, the results from each arm of the code
    // need to be the same type. The below gives a compilation error.
}
