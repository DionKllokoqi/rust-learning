
// Constants are immutable and cannot have the `mut` keyword.
// They can be local or global.
// Type of const value must be annotated.
// Constants can be only set to a constant expression which can be determined at compile time.
// Constants useful for values in your application domain that multiple parts of the program might need to know about, such as the maximum number of points any player of a game is allowed to earn, or the speed of light.
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Rust variables are by default immutable.
    // let x = 5;
    // println!("The value of x is: {x}");
    // x = 6; // This will not compile -> ^^^^^ cannot assign twice to immutable variable.
    // println!("The value of x is: {x}");

    let mut x = 5; // This will work since x is explicitly declared as mutable.
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Rust allows us to define a new variables with the same name as a previous variable. This is called `shadowing`. The second variable overshadows the first, taking any uses of the variable name to itself until either it itself is shadowed or the scope ends. We can shadow a variable by using the same variable’s name and repeating the use of the let keyword.
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}"); // Will print 12
    }

    println!("The value of y is: {y}"); // Will print 6

    println!("Value of globally defined constant variable three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing also allows us to use a different type. For example, we might want to store from the user the amount of spaces to use for some text from their input, and then determine its value as a number.
    // let spaces = "    ";
    // let spaces = spaces.len();

    // If we use mut instead of shadowing, this will throw a compile error
    // let mut spaces = "    ";
    // spaces = spaces.len();

    let mut z: u32 = 1;
    {
        let mut z = z;
        z += 2;
    }
    println!("{z}");
}
