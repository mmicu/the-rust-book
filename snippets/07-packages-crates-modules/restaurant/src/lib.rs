// In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default.
//
// If you want to make an item like a function or struct private, you put it in a module.
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    // We can also use `pub` to designate structs and enums as public.
    // Each attribute must use `pub` to make it public, otherwise it is private by default.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    // In contrast, if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();

        // Using `super` allows us to reference an item that we know is in the parent module.
        super::deliver_order();
    }

    fn cook_order() {}
}

// Similar to creating a symbolic link in the filesystem.
//
// This is valid only in the scope where the `use` occurs.
//
// This is the idiomatic way to bring a function into scope with `use`.
// Specifying the parent module when calling the function makes it clear
// that the function is not locally defined while still minimizing repetition of the full path.
use crate::front_of_house::hosting;

// Bringing the same names in the current scope can be solved with the usage of `as`.
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    Ok(())
}

fn function2() -> IoResult<()> {
    Ok(())
}

// Use the `re-exporting` technique to bring an item into scope,
// but also making that item available for others to bring into their scope.
//
// `as pub_hosting` is not necessary but needed here since `hosting` is already into scope.
pub use crate::front_of_house::hosting as pub_hosting;

// The standard std library is also a crate that’s external to our package.
use std::collections::HashMap;

// Glob operator.
//
// Much useful when testing to bring everything under test into the `tests` module.
use std::collections::*;

// The `eat_at_restaurant` function is part of our library crate's public API, so we mark it with the `pub` keyword.
pub fn eat_at_restaurant() {
    // Absolute path.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path.
    front_of_house::hosting::add_to_waitlist();

    // Thanks to the `use` defined before the function.
    hosting::add_to_waitlist();

    // Interact with the struct.
    let meal = back_of_house::Breakfast::summer("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Interact with the enum.
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
