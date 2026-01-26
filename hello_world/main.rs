// These lines define a function named main. The main function is special: it is always the first code that runs in every executable Rust program. Here, the first line declares a function named main that has no parameters and returns nothing. If there were parameters, they would go inside the parentheses ().

//The function body is wrapped in {}. Rust requires curly brackets around all function bodies. It’s good style to place the opening curly bracket on the same line as the function declaration, adding one space in between.

// Format code with rustfmt
fn main() {

    // println! calls a Rust macro. Rust macros are a way to write code that generates code to extend Rust syntax.
    // Using a ! means that you’re calling a macro instead of a normal function and that macros don’t always follow the same rules as functions.
    println!("Hello, World!");
}

// Before running a Rust program, you must compile it using the Rust compiler by entering the rustc command and passing it the name of your source file, like this:

// rustc is the primary compiler for Rust. It translates source code into binaries or libraries. C# has the roslyn compiler, which is an open source .NET compiler, which is invoked via dotnet commands.

// rustc main.rs. This is similar to C/C++'s gcc or clang. This produces a binary executable, which you can execute by, e.g., ./main  or .\main on Windows. On Windows you see a .exe and a .pdb (containing debug information), on all other platforms only a main file.

// Rust is a "ahead-of-time" compiled language. You can give the executable to anyone without having Rust installed. Interpreted langauges like Python, Ruby, JavaScript need to have the langauges themselves installed to run a program. .NET needs the respective runtime because the compiled output is IL code, i.e., you need the CLR, garbage collector, and core libraries. You can, however, make a self-contained deployment, which bundles runtime and all dependencies into a single executable. Self-contained deployments are also target/platform dependent, e.g., linux-x64, etc. C# also allows AOT compilation for optimized use-cases like IoT, etc., but has some drawbacks like some missing dynamic features like reflection, etc.
