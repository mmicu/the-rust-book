// Methods are similar to functions.
//
// Unlike functions:
//   1. Methods are defined within the context of a struct (or an `enum` or a `trait` object).
//   2. Their first parameter is always `self`, which represents the instance of the struct the method is being called on.
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // `self` is a short for `self: &Self`.
    //
    // Also, within an `impl` block, the type `Self` is an alias for the type that the `impl` block is for.
    //
    // In this case, we are borrowing the `Self` instance (by using `&self`),
    // but methods can:
    //   1. Take ownership of `self`.
    //   2. Borrow `self` immutably (as done here).
    //   3. Borrow `self` mutably.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function without `self`.
    //
    // The `Self` keywords in the return type and in the body of the function are aliases for the type that appears after the `impl` keyword,
    // which in this case is `Rectangle`.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

// We can have multiple `impl` blocks.
impl Rectangle {
    fn perimeter(&self) -> u32 {
        (self.width + self.height) * 2
    }
}

fn main() {
    rectangle_definition();
    associated_functions();
}

fn rectangle_definition() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area() // Method syntax used here.
    );

    // Rust has a feature called `automatic referencing and dereferencing`.
    //
    // When you call a method with `object.something()`,
    // Rust automatically adds in `&`, `&mut`, or `*` so that object matches the signature of the method.
    // In other words, the following are the same:
    //
    // ```rust
    // p1.distance(&p2);
    // (&p1).distance(&p2);
    // ```

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}

fn associated_functions() {
    // All functions defined within an `impl` block are called `associated functions` because they are associated with
    // the type named after the `impl`.
    //
    // We can define associated functions that do not have `self` as their first parameter (and thus are not methods)
    // because they do not need an instance of the type to work with (e.g. `String::from`).
    let rect = Rectangle::square(10);
}
