// Every value in Rust is of a certain data type, which tells Rust what kind of data is being specified so it knows how to work with that data.

// Rust is a statically typed language, which means that it must know the types of all variables at compile time. However, the compiler can usually infer what type we want to use based on the value and how we use it.

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!"); // Type annotation tells Rust what kind of type this variable is.

    // Scalar Types:

    // Rust scalar types represent a single value. Rust has four primary scalar types: integers, floating-point numbers, booleans, and characters.

    // Integer type:
    // An integer is a number without a fractional component. Rust supports signed and unsigned integers. Signed integers start with `i`, unsigned ones with `u`.

    // Length	Signed	Unsigned
    // 8-bit	i8	u8
    // 16-bit	i16	u16
    // 32-bit	i32	u32
    // 64-bit	i64	u64
    // 128-bit	i128	u128
    // architecture dependent	isize	usize

    // Signed numbers are stored using two’s complement representation.

    // Each signed variant can store numbers from −(2n − 1) to 2n − 1 − 1 inclusive, where n is the number of bits that variant uses. So an i8 can store numbers from −(27) to 27 − 1, which equals −128 to 127. Unsigned variants can store numbers from 0 to 2n − 1, so a u8 can store numbers from 0 to 28 − 1, which equals 0 to 255.

    // Rust’s defaults are generally good places to start: integer types default to i32. The primary situation in which you’d use isize or usize is when indexing some sort of collection.

    // Integer overflow:

    // Let’s say you have a variable of type u8 that can hold values between 0 and 255. If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors.

    // 1. In debug mode, Rust includes checks for integer overflow that cause your program to panic at runtime if this behavior occurs.
    // 2. When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics. Instead, if overflow occurs, it performs two’s complement wrapping. In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold. In the case of a u8, the value 256 becomes 0, the value 257 becomes 1, and so on.

    // Floating point types:

    // f32 and f64, which are 32 bits and 64 bits in size, respectively.
    // default type is f64 because on modern CPUs, it’s roughly the same speed as f32 but is capable of more precision. All floating-point types are signed.

    let x = 2.0; // f64
    println!("f64 x is {x}");

    let y: f32 = 3.0; // f32
    println!("f32 y is {y}");

    // Rust supports the basic mathematical operations: addition, subtraction, multiplication, division, and remainder. Integer division truncates toward zero to the nearest integer.

    // addition
    let sum = 5 + 10;
    println!("The sum is: {sum}");

    // subtraction
    let difference = 95.5 - 4.3;
    println!("The difference is: {difference}");

    // multiplication
    let product = 4 * 30;
    println!("The product is: {product}");

    // division
    let quotient = 56.7 / 32.2;
    println!("The quotient is: {quotient}");

    let truncated = -5 / 3; // Results in -1
    println!("The truncated division is: {truncated}");

    // remainder
    let remainder = 43 % 5;
    println!("The remainder is: {remainder}");

    // Boolean type:

    // As in most other programming languages, a Boolean type in Rust has two possible values: true and false. Booleans are one byte in size.
    let t = true;
    println!("bool value of t is : {t}");

    let f: bool = false; // with explicit type annotation
    println!("bool value of f is : {f}");

    // Character type:

    // Rust’s char type is the language’s most primitive alphabetic type.
    let c = 'z';
    println!("char value of c is : {c}");

    let z: char = 'ℤ'; // with explicit type annotation
    println!("char value of z is : {z}");

    let heart_eyed_cat = '😻';
    println!("char value of heart_eyed_cat emoji is : {heart_eyed_cat}");

    // Note that we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode scalar value, which means it can represent a lot more than just ASCII. Accented letters; Chinese, Japanese, and Korean characters; emoji; and zero-width spaces are all valid char values in Rust.

    // Compound Types:

    // Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

    // Tuples:

    // A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

    // We create a tuple by writing a comma-separated list of values inside parentheses. Each position in the tuple has a type, and the types of the different values in the tuple don’t have to be the same.

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value:

    let (x, y, z) = tup; // uses a pattern with let to take tup and turn it into three separate variables, x, y, and z. This is called destructuring because it breaks the single tuple into three parts.

    println!("The value of y is: {y}");

    // We can also access a tuple element directly by using a period (.) followed by the index of the value we want to access.

    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // The tuple without any values has a special name, unit. This value and its corresponding type are both written () and represent an empty value or an empty return type. Expressions implicitly return the unit value if they don’t return any other value.

    // Additionally, we can modify individual elements of a mutable tuple.

    let mut x: (i32, i32) = (1, 2);
    x.0 = 0; // Final value is zero
    x.1 += 5; // Final value is 7

    // The Array Type:

    // Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

    let a = [1, 2, 3, 4, 5];

    // Arrays are useful when you want your data allocated on the stack, the same as the other types we have seen so far, rather than the heap, or, when when you want to ensure you always have a fixed number of elements.

    // An array isn’t as flexible as the vector type, though. A vector is a similar collection type provided by the standard library that is allowed to grow or shrink in size because its contents live on the heap.

    // arrays are more useful when you know the number of elements will not need to change. For example, if you were using the names of the month in a program, you would probably use an array rather than a vector because you know it will always contain 12 elements:

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    // You write an array’s type using square brackets with the type of each element, a semicolon, and then the number of elements in the array, like so:

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets

    let a = [3; 5];

    // An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing

    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    // If you try to access an array index that is out of range during runtime, e.g., an array is defined, user is prompted for an index and writes one that is out of range, rust will panic. This is a safety principle that is missing in many other low-level langauges, where an incorrect index could access an invalid area of memory. Rust protects you against this kind of error by immediately exiting instead of allowing the memory access and continuing.
}
