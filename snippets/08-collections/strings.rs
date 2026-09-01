// Rust has only one string type in the core language,
// which is the string slice `str` that is usually seen in its borrowed form, `&str`.
//
// The `String` type, which is provided by Rust's standard library, is a:
//
//  - Growable
//  - Mutable
//  - Owned
//  - UTF-8 encoded
//
// string type.
fn main() {
    create_string();
    different_languages_strings();
    updating_string();
    concatenating_strings();
    indexing_strings();
    perspective_of_a_string_and_slicing();
    iterating_strings();
}

fn create_string() {
    // Many of the same operations available with `Vec<T>` are available with `String`,
    // since `String` is actually implemented as a wrapper around a vector of bytes (Vec<u8>).
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();

    // The method also works on a literal directly:
    let s = "initial contents".to_string();
    // Similar approach.
    let s = String::from("initial contents");
}

fn different_languages_strings() {
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שלום");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}

fn updating_string() {
    // We can grow a `String` by using the `push_str` method to append a string slice.
    //
    // The `push_str` method takes a string slice because we do not necessarily want to take ownership of the parameter.
    let mut s = String::from("foo");
    s.push_str("bar");

    // If the `push_str` method took ownership of `s2`, we would not be able to print its value afterwards.
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {s1}");
    println!("s2 is {s2}");

    // The `push` method takes a single character as a parameter and adds it to the `String`.
    let mut s = String::from("lo");
    s.push('l');
}

fn concatenating_strings() {
    // The reason `s1` is no longer valid after the addition,
    // and the reason we used a reference to `s2`,
    // has to do with the signature of the method that's called when we use the `+` operator.
    //
    // The `+` operator uses the `add` method, whose signature looks something like this: `fn add(self, s: &str) -> String`:
    //
    //   - `self` means the value is moved, hence `s1` won't be available afterwards.
    //   - `&s` is the reference, hence `s2` will be available afterwards.
    //
    // **Very important**
    // The type of `&s2` is `&String`, not `&str`, as specified in the second parameter to `add`.
    // So, why does the following code compile?
    //
    // The compiler can coerce the `&String` argument into a `&str`.
    // When we call the `add` method, Rust uses a deref coercion,
    // which here turns `&s2` into `&s2[..]`.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note: `s1` has been moved here and can no longer be used.
    println!("s3 is {s3}");

    // Same behavior in case we need to concatenate multiple strings.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = s1 + "-" + &s2 + "-" + &s3;
    println!("s is {s}");
}

fn indexing_strings() {
    // In many other programming languages, accessing individual characters in a string by referencing them
    // by index is a valid and common operation.
    //
    // However, if you try to access parts of a `String` using indexing syntax in Rust, you will get an error.
    //
    // In fact, the following code won't compile:
    //
    // ```rust
    // let s1 = String::from("hi");
    // let h = s1[0];
    // ```
    //
    // **Why not?**
    // A `String` is a wrapper over a `Vec<u8>`.
    // If we consider a string like "hola", `len` will be 4,
    // which means the vector storing this string is 4 bytes long.
    // In fact, each of these letters takes 1 byte when encoded in UTF-8.
    //
    // However, let's consider the string "Здравствуйте".
    // You might say that `len` is 12, but it is 24 instead.
    // That's the number of bytes it takes to encode this string in UTF-8,
    // because each Unicode scalar value in that string takes 2 bytes of storage.
    //
    // Therefore, an index into the string's bytes will not always correlate to a valid Unicode scalar value.
    let s = String::from("Hola");
    let n = s.len();
    println!("len(\"{s}\") = {n}");

    let s = String::from("Здравствуйте");
    let n = s.len();
    println!("len(\"{s}\") = {n}");
}

fn perspective_of_a_string_and_slicing() {
    // There are actually three relevant ways to look at strings from Rust's perspective:
    //
    //   1. Bytes.
    //   2. Scalar values.
    //   3. Grapheme clusters (closest thing to what we would call *letters*).
    //
    // If we look at the Hindi word "नमस्ते", it is stored as a vector of `u8` values that looks like this:
    //
    //   `[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]`
    //
    // That's 18 bytes.
    //
    // If we look at them as Unicode scalar values, which are what Rust's `char` type is, those bytes look like this:
    //
    //   `['न', 'म', 'स', '्', 'त', 'े']`
    //
    // These are 6 `char` values here, but the fourth and sixth are not letters:
    // they are diacritics that don't make sense on their own.
    //
    // Finally, if we look a them as grapheme clusters,
    // we would get what a person would call the 4 letters that make yo the Hindi word:
    //
    //   `["न", "म", "स्", "ते"]`
    //
    //
    //
    // Indexing into a string is often a bad idea because it is not clear what the return type of
    // the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.
    //
    // ```rust
    // let hello = "Здравствуйте";
    // ```
    //
    // If we were to try to slice only part of a character’s bytes with something like `&hello[0..1]`,
    // Rust would panic at runtime in the same way as if an invalid index were accessed in a vector.
}

fn iterating_strings() {
    let sep = "==========";
    println!("{}", sep);

    for c in "Зд".chars() {
        println!("{c}");
    }
    println!("{}", sep);

    for b in "नमस्ते".chars() {
        println!("{b}");
    }
    println!("{}", sep);

    for b in "Зд".bytes() {
        println!("{b}");
    }

    // Getting grapheme clusters from strings is complex,
    // so this functionality is not provided by the standard library.
    //
    // Crates are available on <crates.io> if this is the functionality you need.
}
