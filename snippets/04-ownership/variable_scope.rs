// <https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variable-scope>
fn main() {
    let s = "hello"; // A string literal.
} // Scope is over and `s` is not valid anymore.
