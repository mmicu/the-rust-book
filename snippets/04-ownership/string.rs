fn main() {
    // All data types with known fixed size at compile time.
    // They can be easily stored on the stack and popped when their scope is over.
    let x: i32 = 10;
    let b: bool = true;

    // String literals are immutable and not stored in the heap.
    let s = "hello";

    // If we want to store text that comes from stdin for example,
    // we must use the `String` type.
    //
    // This type manages data allocated on the **heap and
    // as such is able to store an amount of text that is unknown at compile time.
    let s = String::from("hello");
    println!("{s}");

    // This kind of string can be mutated.
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{s}");
}
