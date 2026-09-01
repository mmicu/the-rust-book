// The type `HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V` using a hashing function,
// which determines how it places these keys and values into memory.
//
// Details on the hashing function that Rust (*SipHash*) uses can be found [here](https://doc.rust-lang.org/book/ch08-03-hash-maps.html#hashing-functions).
use std::collections::HashMap;

fn main() {
    create_hash_map();
    managing_ownership_in_hash_map();
}

fn create_hash_map() {
    let mut scores = HashMap::new();

    // Adding values.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{scores:?}");

    // Overwriting a value.
    scores.insert(String::from("Yellow"), 100); // Value will be replaced, since the "Yellow" is already in the hash map.
    println!("{scores:?}");

    // Getting values.
    let team_name = String::from("Blue");
    // `copied()` is used to get an `Option<i32>` rather than an `Option<&i32>`.
    let score = scores.get(&team_name).copied().unwrap_or(0);

    // `entry` return an enum called `Entry` that represents a value that might or might not exist.
    //
    // The `or_insert` method on `Entry` is defined to:
    //
    //   - Return a mutable reference to the value for the corresponding `Entry` key, if that key exists.
    //   - Insert the parameter as the new value for this key and returns a mutable reference to the new value, otherwise.
    scores.entry(String::from("Blue")).or_insert(0);
    scores.entry(String::from("Yellow")).or_insert(0);
    scores.entry(String::from("Red")).or_insert(50);
    println!("{scores:?}");

    // Iterating.
    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

fn managing_ownership_in_hash_map() {
    // For types that implement the `Copy` trait, like `i32`, the values are copied into the hash map.
    // For owned values like `String`, the values will be moved and the hash map will be the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // `field_name` and `field_value` are invalid at this point.
}
