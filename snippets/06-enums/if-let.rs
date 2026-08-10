// The `if let` syntax lets you combine `if` and `let` into a less verbose way
// to handle values that match one pattern while ignoring the rest.

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    // Verbose approach.
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        _ => (),
    }

    // Better approach.
    //
    // The syntax `if let` takes a pattern and an expression separated by an equal sign.
    // You can think of `if let` as syntax sugar for a `match` that runs code when
    // the value matches one pattern and then ignores all other values.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {max}");
    }

    // `let...else` example: <https://doc.rust-lang.org/book/ch06-03-if-let.html#listing-6-9>.
}
