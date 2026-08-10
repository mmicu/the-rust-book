// The `Option` type encodes the very common scenario in which a value could be something, or it could be nothing.
//
// Rust does not have the `null` feature that many other languages have.
// In languages with `null`, variables can always be in one of two states: `null` or `not-null`.
//
// The `null` led to "Null References: The Billion Dollar Mistake" by Tony Hoare, the inventor of `null`.
//
// Rust solves this problem in a very elegant way.
// The `null` issue is not really with the concept but with the particular implementation.
// As such, Rust does not have nulls, but it does have an enum that can encode the concept
// of a value being present or absent.
//
// This enum is `Option<T>``, and it is defined by the standard library as follows:
//
// ```rust
// enum Option<T> {
//     None,
//     Some(T),
// }
// ```
//
// It is so important that it is even included in the prelude.
//
// Because `Option<T>` and `T` are different types,
// the compiler won't let us use an `Option<T>` value as if it were definitely a valid value.
// For example, this code won't compile, because it is trying to add an `i8` to an `Option<i8>`:
//
// ```rust
// let x: i8 = 5;
// let y: Option<i8> = Some(5);
// let sum = x + y;
// ```
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');

    let absent_number: Option<i32> = None;
}
