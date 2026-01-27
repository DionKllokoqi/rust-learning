use std::io;
use rand::Rng; // The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use std::cmp::Ordering;

// By default, Rust has a set of items defined in the standard library that it brings into the scope of every program. This set is called the prelude.
// If a type you want to use isn’t in the prelude, you have to bring that type into scope explicitly with a use statement. Using the std::io library provides you with a number of useful features, including the ability to accept user input.

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // rand::thread_rng function gives us a random number generator that is local to the current thread and seeded by the OS. gen_range method generates the random number via the range inputed, between 1 - 100 inclusive.

    loop {
        println!("Please input your guess.");

        // let is used to create a variable. For example, let apple = 5; creates a variable named apple and sets its value to 5. By default, variables are immutable, meaning that once a value is assigned to a variable name, you can’t change that value. To make a variable mutable, you can use the mut keyword before the variable name.

        let mut guess = String::new(); // String is a string type provided by the standard library that is a growable, UTF-8 encoded bit of text. The :: syntax in the ::new line indicates that new is an associated function of the String type. An associated function is a function that’s implemented on a type, in this case String.

        // We call the stdin function from the io module, which allows us to handle user input. This type represents a handle to the standard input of your terminal.
        io::stdin()
            .read_line(&mut guess) // We call the read_line method on the standard input handle. The full job of read_line is to take whatever the user types into standard input and append that into a string (without overwriting its contents), so we therefore pass that string as an argument. The string argument needs to be mutable so the method can change the string’s content. The & indicates that this argument is a reference, which gives you a way to let multiple parts of your code access one piece of data without needing to copy that data into memory multiple times. References are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.
            .expect("Failed to read line"); // read_line returns a Result value, which is an enum that can have two possible states: Ok and Err. This is the result pattern from functional programming which can be utilized in C# via libraries like LanguageCore.Ext. Result has methods defined on the type, such as expect(...). If the value of Result is Err, expect will cause the program to crash and display the message passed as an arg. If the Result is Ok, expect will just return the value, i.e., the number of bytes in the user input.

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // we want to match all Err values, no matter what information they have inside them. continue, which tells the program to go to the next iteration of the loop and ask for another guess.
        }; // Rust allows us to shadow the previous value of guess with a new one. Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables, such as guess_str and guess.

        println!("You guessed: {guess}"); // {} is a placeholder. If you want to print the result of an expression, you can do this: println!("x = {x} and y + 2 = {}", y + 2);

        // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

// Crates are a collection of Rust source code files. There are two types of crates: binary executables, e.g., this project, and, library crates, which contain code intended to be used by other programs. Crates are similar to Nuget in C#, but not quite the same.

// Crates.io is where open source crates are hosted by the Rust community.

// When you build a project for the first time, Cargo figures out all the versions of the dependencies that fit the criteria and then writes them to the Cargo.lock file. When you build your project in the future, Cargo will see that the Cargo.lock file exists and will use the versions specified there rather than doing all the work of figuring out versions again.

// Because the Cargo.lock file is important for reproducible builds, it’s often checked into source control with the rest of the code in your project.

// To update a version, while still staying in the same minor version, you can call `cargo update`, which will increase the version, e.g., from 0.8.5 -> 0.8.6 and update the Cargo.lock file with this newer version. To increase the minor version, we'd need to update the Cargo.toml file to, e.g., 0.9.0.

// cargo doc --open command will build documentation provided by all your dependencies locally and open it in your browser.