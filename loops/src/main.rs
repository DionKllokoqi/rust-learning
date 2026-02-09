fn main() {
    // === Loop essentials ===

    // Rust has three kinds of loops: loop, while, and for.

    // The loop keyword tells Rust to execute a block of code over and over again forever or until you explicitly tell it to stop.
    loop {
        println!("again!");
        break;
    }

    // You can place the `break` keyword within the loop to tell the program when to stop executing the loop.
    // We can also use the keyword `continue`, which in a loop tells the program to skip over any
    // remaining code in this iteration of the loop and go to the next iteration.

    // === Returning values from loops ===

    // You can use loops to retry operations which you know might fail.
    // You can also pass the result of an operation that's done inside a loop outside of it, by placing it
    // after the `break` keyword.
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // Rust compiler treats a break expression and a return expression as having the value unit, or ().

    // The `break` keyword exits the current loop. The `return` keyword exits the current function.

    // === Loop labels for loop disambiguation ===
    /* If you have loops within loops, break and continue apply to the innermost loop
    at that point. You can optionally specify a loop label on a loop that you can then
    use with break or continue to specify that those keywords apply to the labeled loop
    instead of the innermost loop. */
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // === Conditional loops with while ===

    // A program will often need to evaluate a condition within a loop.
    // While the condition is true, the loop runs. When the condition ceases to be true,
    // the program calls break, stopping the loop. This is natively supported with while loops.

    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // === Looping through a collection with for ===
    // With while
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    /* this approach is error prone; we could cause the program to panic if the index
    value or test condition is incorrect. For example, if you changed the definition of
    the a array to have four elements but forgot to update the condition to while index < 4,
    the code would panic. It's also slow due to the runtime code added by the compiler to
    check that the index is still inside the array bounds. */

    // As a more concise alternative, you can use a for loop and execute some code
    // for each item in a collection.
    let b = [10, 20, 30, 40, 50];

    for element in b {
        println!("the value is: {}", element);
    }

    // This is less error-prone due to compile time restrictions and
    // the machine code generated from for loops can be more efficient as well,
    // because the index doesn’t need to be compared to the length of the array at every iteration

    // We can also loop the countdown with a for loop using range from the standard lib.
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
