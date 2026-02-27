fn main() {
    /* Ownership, boxes and moves provide a foundation for safe programming with the heap.
    However, this is inconvenient. For example: */
    let m1 = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    // let s = format!("{}, {}", m1, m2); This will not compile, since pointee is freed after we exit greet.

    // ========== References as Non-Owning Pointers ==========

    // A `reference` is a kind of pointer.
    let m1 = String::from("Hello");
    let m2 = String::from("world"); // L1
    greet_by_reference(&m1, &m2); // L3 Note the ampersands.
    let s = format!("{} {}!", m1, m2);

    // The expression &m1 uses the ampersand operator to create a reference to (or “borrow”) m1.
    // &String means "reference to a string".
    // At L2, g1 is a reference pointing to m1, and m1 is a String containing a box that points
    // to "Hello" on the heap. While m1 owns the heap data, g1 does not own either m1 or "Hello".
    // After `greet_by_reference` ends, i.e., we reach L3, no heap data has been deallocated. Only
    // the stack frame for `greet_by_reference` disappears. This is in-line with the `Box Deallocation
    // Principle`. Since g1 does not own "Hello", Rust did not deallocate it on behalf of g1.

    // References are "non-owning pointers".

    // Underlying operator is the * dereference operator. A few examples:

    let mut x: Box<i32> = Box::new(1);
    let a: i32 = *x;         // *x reads the heap value, so a = 1
    *x += 1;                 // *x on the left-side modifies the heap value,
    //     so x points to the value 2

    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value

    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it

    // Rust implicitly inserts dereferences and references in certain cases, such as
    // calling a method with the dot operator.

    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs();      // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs();       // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len();      // implicit reference
    assert_eq!(s_len1, s_len2);

    /* The i32::abs function expects an input of type i32. To call abs with a Box<i32>,
    you can explicitly dereference the box like i32::abs(*x).
    You can also implicitly dereference the box using method-call syntax like x.abs().
    The dot syntax is syntactic sugar for the function-call syntax.

    This implicit conversion works for multiple layers of pointers. For example,
    calling abs on a reference to a box r: &Box<i32> will insert two dereferences.

    This conversion also works the opposite direction. The function str::len expects
    a reference &str. If you call len on an owned String, then Rust will insert a single
    borrowing operator. (In fact, there is a further conversion from String to str!) */

    // ========== Simultaneous Aliasing and Mutation ==========

    /* Pointers enable `aliasing`. Aliasing is accessing the same data through different variables.
    Combined with mutation, this can lead to issues:
        1. Deallocating the aliased data leaves the other variable pointing to deallocated memory.
        2. Aliased data is mutated, leading to unexpected behavior during runtime.
        3. By concurrently mutating the aliased data, there can be a race condition leading to
        nondeterministic behavior.*/

    // Vectors, compared to arrays, don't have a fixed length, and are allocated on the heap.
    // The vec! macro allocates a heap of a certain `capacity`. When we push data to a vector,
    // the vector has to create a new allocation with a larger capacity, copy all the elements over,
    // and deallocate the previous heap array.

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    v.push(4); // This shows a compilation error.
    println!("Third element is {}", *num);

    // Data cannot be both aliased and mutated.

    // ========== References Change Permissions on Places

    /* Rust ensures safety through the `borrow checker`. The core idea behind the `borrow checker`
     is that variables have three kinds of permissions on their data:
        * Read (R): data can be copied to another location.
        * Write (W): data can be mutated.
        * Own (O): data can be moved or dropped.

    these permissions don't exist at runtime, only within the compiler. They describe how the compiler
    `thinks` about your program before the program is executed.

    By default, a variable has read/own permissions (RO) on its data. If a variable is annotated with let mut,
    then it also has the write permission (W). The key idea is that references can temporarily remove these
    permissions.*/

    // Permissions example:

    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &i32 = &v[2];
    println!("Third element is {}", *num);
    v.push(4);

    /*
    1. After let mut v = (...), the variable v has been initialized (indicated by ). It gains +R+W+O permissions (the plus sign indicates gain).
    2. After let num = &v[2], the data in v has been borrowed by num (indicated by ). Three things happen:
        * The borrow removes WO permissions from v (the slash indicates loss). v cannot be written or owned, but it can still be read.
        * The variable num has gained RO permissions. num is not writable (the missing W permission is shown as a dash ‒) because it was not marked let mut.
        * The place *num has gained the R permission.
    3. After println!(...), then num is no longer in use, so v is no longer borrowed. Therefore:
        * v regains its WO permissions (indicated by ).
        * num and *num have lost all of their permissions (indicated by ).
    4. After v.push(4), then v is no longer in use, and it loses all of its permissions.
    */

    let x = 0;
    let mut x_ref = &x;

    /* Notice that x_ref has the W permission, while *x_ref does not. That means we can assign a different
    reference to the x_ref variable (e.g. x_ref = &y), but we cannot mutate the data it points to (e.g. *x_ref += 1).
    More generally, permissions are defined on places and not just variables. A place is anything you can put on
    the left-hand side of an assignment.

    The goal of these permissions is to ensure that data cannot be mutated if it is aliased. Creating a reference
    to data (“borrowing” it) causes that data to be temporarily read-only until the reference is no longer in use.
    Rust uses these permissions in its borrow checker. The borrow checker looks for potentially unsafe operations
    involving references.*/
}

fn greet_by_reference(g1: &String, g2: &String) {
    println!("{} {}!", g1, g2); // L2
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}
