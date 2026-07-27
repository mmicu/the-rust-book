fn main() {
    string_slices();
    string_literals_as_slices();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slices() {
    // A `string slice` is a reference to a contiguous sequence of the elements of a `String`,
    // and it looks like this:
    let s = String::from("hello world");

    // `hello` and `world` are references to a portion of the `String`.
    let hello = &s[0..5];
    let world = &s[6..11];

    println!("{s}");
    println!("{hello}");
    println!("{world}");

    // Slice the entire string.
    let slice = &s[..];
    println!("{slice}");

    let s = String::from("hello world!");
    let fs = first_word(&s);
    println!("The first word of {s} is {fs}");
}

fn string_literals_as_slices() {
    // String literals are stored inside the binary.
    //
    // The type of `s` here is `&str`: it is a `slice` pointing to that specific point of the binary.
    //                                 This is also why string literals are immutable; `&str` is an immutable reference.
    let s = "Hello, world!";

    // Then, a better way to define `first_word` would be:
    //
    // ```rust
    // fn first_word(s: &str) -> &str { ... }
    // ```
    //
    // If we have a `string slice`, we can pass that directly.
    // If we have a `String`, we can pass a slice of the `String` or a reference to the `String`.
    //
    // This flexibility takes advantage of `deref coercions`.
}
