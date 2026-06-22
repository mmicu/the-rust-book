// <https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-move>
fn main() {
    move();
}

fn move() {
    fn copy_int() {
        let x = 1;
        // Value is copied, because integers are simple values with a known, fixed size.
        let y = x;
    }

    fn copy_string() {
        // Not the same compared to the `copy_int` version.
        //
        // The `String` contains 3 parts:
        //  1. Pointer to the memory that holds the content of the string.
        //  2. Length: memory (in bytes) currently in use.
        //  3. Capacity: memory (in bytes) received from the allocator.
        let s1 = String::from("hello");
        // In this assignment, we copy the pointer, the length, and the capacity that are on the stack,
        // **but** we do not copy the data on the heap that the pointer refers to.
        let s2 = s1;
        // The usual problem **double free** won't happen here, since Rust won't call the `drop` method on `s1` and `s2`.
        // To do so, Rust considers `s1` as no longer valid.
        // Therefore, Rust **does not** need to free anything when s1 goes out of scope.
        // In fact, with the following line, we would get a compiler error:
        //
        // ```rust
        // println!("{s1}, world!");
        // ```
        //
        // This is indeed a **move** operation. In this example, we would say that `s1` was moved into `s2`.
    }

    copy_int();
    copy_string();
}
