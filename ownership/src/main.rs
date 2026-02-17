fn main() {
    // This code compiles
    let x = true;
    read(x);

    /* This code does not compile in Rust. In an interpreted language like Python or JavaScript this
    would raise a runtime exception such as NameError or ReferenceError. Each time an interpreted
    program reads a variable, then the interpreter must check whether that variable is defined.*/
    // read(y);
    // let y = true;

    /* A foundational goal of Rust is to ensure that your programs never have undefined
    behavior. That is the meaning of “safety.” */

    /* Since safety is the absence of undefined behavior, and since ownership is about safety,
     then we need to understand ownership in terms of the undefined behaviors it prevents. */

    // Rust provides a particular way to think about memory. Ownership is a discipline for
    // safely using memory within that way of thinking.

    // Variables live in the stack. The structure that is used in Rust for holding variables is
    // called a frame. A frame is a mapping from variables to values within a single scope,
    // such as a function.

    let n = 5; // The frame for main at location L1 holds n = 5.
    let y = plus_one(n); // The frame for main at location L3 holds n = 5; y = 6.
    println!("The value of y is: {}", y);

    /* Frames are organized into a stack of currently-called-functions.
    For example, at L2 the frame for main sits above the frame for the
    called function plus_one. After a function returns, Rust deallocates
    the function’s frame. (Deallocation is also called freeing or dropping,
    and we use those terms interchangeably.) This sequence of frames is
    called a stack because the most recent frame added is always the next frame freed. */

    // When an expression reads a variable, the variable's value is copied from its slot
    // in the stack frame, e.g.,

    let a = 5; // L1 stack frame a / 5
    let mut b = a; // L2 stack frame a / 5, b / 5
    b += 1; // L3 stack frame a / 5, b / 6. Value of `a` was copied over to b, leaving `a` unchanged.

    // ========== Boxes Live in the Heap ==========
    let c = [0; 1_000_000];
    let d = c;

    // The above code would copy over the one million element array into the stack frame, causing
    // the main frame to contain 2 million entries.

    // To transfer data without copying it, Rust uses `Pointers`. A pointer is a value that
    // describes a location in memory. The value that the pointer points to is called its
    // `pointee`. One common way to make a pointer is to allocate memory in the heap. The
    // heap is a separate region of memory where data can live indefinitely. Heap data is
    // not tied to a specific stack frame. Rust provides a construct called Box for putting data on the heap.
    let e = Box::new([0; 1_000_000]);
    let f = e;

    // Now, there's only a single array at a time. The value of e is a pointer to the array inside
    // the heap. The statement let f = e copies the pointer from e into f, but the pointed-to
    // data is not copied. e has been `moved` now.

    // Stack frames are associated with a specific function, and are deallocated when the
    // function returns. Data on the heap can live indefinitely. Both stack and heap data
    // is can be mutable and copyable. The heap can also contain pointers, even back to the stack.

    // Rust does not permit manual memory management. Stack frames are automatically managed by Rust.
    // When a function is called, Rust allocates a stack frame for the called function. When
    // the call ends, Rust deallocates the stack frame.

    // When heap data is allocated via Box::new, we can't manually call something like a `free()`
    // function.

    // ========== A Box's Owner Manages Deallocation ==========

    // Rust automatically frees a box's heap memory. Almost correct definitions:

    // Box deallocation principle (almost correct): If a variable is bound to a box,
    // when Rust deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

    let a_num = 4; // L1
    make_and_drop(); // L3

    /* At L1, before calling make_and_drop, the state of memory is just the stack frame for main.
    Then at L2, while calling make_and_drop, a_box points to 5 on the heap. Once make_and_drop
    is finished, Rust deallocates its stack frame. make_and_drop contains the variable a_box,
    so Rust also deallocates the heap data in a_box. Therefore, the heap is empty at L3. */

    // What happens when we bind two variables to a box?
    let a1 = Box::new([0; 1_000_000]);
    let b1 = a1;

    // The boxed array has now been bound to both `a` and `b`. Would Rust try to free the
    // box's heap memory twice?

    // When `a` is bound to  Box:new(...), we say that `a` owns the box. The statement
    // let b = a moves ownership of the box from `a` to `b`.

    // Box deallocation principle (fully correct): If a variable owns a box, when Rust
    // deallocates the variable’s frame, then Rust deallocates the box’s heap memory.

    // In the example above, `b` owns the boxed array. Therefore, when the scope ends, Rust
    // deallocates the box only once on behalf of `b`, not `a`. This means, we cannot use `a`
    // after ownership has been moved to `b`.

    let first = String::from("Ferris");
    let full = add_suffix(first);
    // println!("{full}, originally {first}"); This causes a compilation error `value used after moved`

    // So if you move a variable, Rust will stop you from using that variable later. More generally,
    // the compiler will enforce this principle:
    // Moved heap data principle: if a variable x moves ownership of heap data to another variable y,
    // then x cannot be used after the move.

    // ========== Cloning Avoids Moves ==========

    // One way to avoid moving data ownership is to `clone` data using the `.clone()` method.
    let original = String::from("Ferris");
    let original_clone = original.clone(); // We deep-copy the string data into a new heap allocation.
    let original_full = add_suffix(original_clone);
    println!("{original_full}, originally {original}");


    // ========== Summary ==========

    /* All heap data must be owned by exactly one variable.
    Rust deallocates heap data once its owner goes out of scope.
    Ownership can be transferred by moves, which happen on assignments and function calls.
    Heap data can only be accessed through its current owner, not a previous owner. */
}

fn add_suffix(mut name: String) -> String {
    name.push_str(" Jr.");
    name
}

fn make_and_drop() {
    let a_box = Box::new(5); // L2
}

fn plus_one(x: i32) -> i32 {
    x + 1 // The frame for plus_one at L2 holds x = 5.
}

fn read(y: bool) {
    if y {
        println!("y is true!");
    }
}
