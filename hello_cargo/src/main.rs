// Cargo is Rust’s build system and package manager. Most Rustaceans use this tool to manage their Rust projects because Cargo handles a lot of tasks for you, such as building your code, downloading the libraries your code depends on, and building those libraries. (We call the libraries that your code needs dependencies.)

// This project was created with the command `cargo new hello_cargo`, which created the following directory structure:
// hello_cargo
// ├── Cargo.toml
// └── src
//     └── main.rs
// The Cargo.toml (Tom’s Obvious, Minimal Language) file contains metadata about your project and its dependencies, i.e., cargos configuration format, and the src/main.rs file is where your Rust code lives.

// Usually, this command would also initialize a git repository and add a .gitignore file, but since this already was a git repository, Cargo skipped that step.

// Statements under [package] are needed by cargo to compile your code.
// The last line, [dependencies], is the start of a section for you to list any of your project’s dependencies. In Rust, packages of code are referred to as crates.

// Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration files, and anything else not related to your code.

// cargo init will create a toml file if there is none.

fn main() {
    println!("Hello, world!");
}

// Running `cargo build` will compile your code and produce an executable in the target/debug directory. Cargo also creates a Cargo.lock file to record the exact versions of your dependencies.

// With `cargo run`, Cargo will build your code (if necessary) and then run the resulting executable. This is a convenient way to compile and run your code in one step.
// $ cargo run
//    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
//     Running `target/debug/hello_cargo`
// Hello, world!

// cargo check. This command quickly checks your code to make sure it compiles but doesn’t produce an executable. Many Rustaceans run cargo check periodically as they write their program to make sure it compiles.

// cargo build --release to compile it with optimizations for release -> target/release directory.