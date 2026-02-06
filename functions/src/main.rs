// The main function is the entry point of Rust programs.
fn main() {
    println!("Hello, world!");

    // Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller. The below function can be called above in main().
    another_function();

    // Call function with argument 5 for x.
    another_function_with_parameters(5);

    print_labeled_measurement(5, 'h')
}

// fn keyword allows you to define new functions.

// Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.

fn another_function() {
    println!("Another function.");
}

// We can define functions to have parameters, which are special variables that are part of a function’s signature. When a function has parameters, you can provide it with concrete values for those parameters. Technically, the concrete values are called arguments, but in casual conversation, people tend to use the words parameter and argument interchangeably.

// In function signatures, you must declare the type of each parameter.

fn another_function_with_parameters(x: i32) {
    println!("The value of x is: {x}");
}

// Function with multiple parameters.

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// Statements and Expressions

// Rust is an expression based language.

// Statements are instructions that perform some action and do not return a value.

// Expressions evaluate to a resultant value.

fn statements_and_expressions() {
    // Creating a variable and assigning it a value is a statement.
    let y = 6;

    // Function definitions are also statements.

    // Statements do not return values, therefore, you can't assign a let statement to another variable, like below:
    let x = (let y = 6); // The let y = 6 statement does not return a value, so there isn’t anything for x to bind to. In other languages, like C or Ruby, an assignment returns the value of the assignment, so you can write, e.g., x = y = 6.

    // Expressions evaluate to a value, e.g., 5 + 6 evaluates to 11.

    // Calling a function is an expressions. Calling a macro is an expression.A new scope block created with curly brackets is an expression.

    // This expression evaluates to 4, which is assigned to y. Note that the x + 1 line doesn’t have a semicolon at the end. Expressions do not include ending semicolons. If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}

// Functions with Return Values:

// We don’t name return values, but we must declare their type after an arrow (->). In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function. You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

fn five() -> i32 {
    5
}

fn call_five() {
    let x = five();

    println!("The value of x is: {x}");
}

// The function below would cause a compilation error, since it evaluates the return line as a statement, not an expression, which doesn't return anything, as required by the function signature:
fn plus_one(x: i32) -> i32 {
    x + 1;
}
