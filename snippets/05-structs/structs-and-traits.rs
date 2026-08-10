#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // If we do not add `#[derive(Debug)]`, we would get a compiler error:
    //
    // ```
    // error[E0277]: `Rectangle` doesn't implement `Debug`
    // ```
    //
    println!("rect is {rect:?}");

    // Another way to print out a value using the `Debug` format is to use the `dbg!` macro,
    // which takes ownership of an expression (as opposed to `println!`, which takes a reference),
    // prints the file and line number of where that `dbg!` macro call occurs in your code along with the resultant value of that expression,
    // and returns ownership of the value.
    //
    // Calling the `dbg!` macro prints to the standard error console stream (stderr).
    dbg!(&rect);
    dbg!(rect);
}
