// Matches are exhaustive: the arms' patterns must cover all possibilities!
// Otherwise, we would get a compiler error.

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin_v2 {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}


fn main() {
    println!("Value in cents for Coin::Penny is {}",   value_in_cents(Coin::Penny));
    println!("Value in cents for Coin::Nickel is {}",  value_in_cents(Coin::Nickel));
    println!("Value in cents for Coin::Dime is {}",    value_in_cents(Coin::Dime));
    println!("Value in cents for Coin::Quarter is {}", value_in_cents(Coin::Quarter));

    patterns_with_values();
    option_match_pattern();
    catch_all_pattern();
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn patterns_with_values() {
    fn value_in_cents(coin: Coin_v2) -> u8 {
        match coin {
            Coin_v2::Penny => 1,
            Coin_v2::Nickel => 5,
            Coin_v2::Dime => 10,
            Coin_v2::Quarter(state) => {
                println!("State quarter from {state:?}!");
                25
            }
        }
    }

    value_in_cents(Coin_v2::Quarter(UsState::Alabama));
    value_in_cents(Coin_v2::Quarter(UsState::Alaska));

    // `if let` approach.
    let coin = Coin_v2::Quarter(UsState::Alabama);
    if let Coin_v2::Quarter(state) = coin {
        println!("State quarter from {state:?}!");
    }
}

fn option_match_pattern() {
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}

fn catch_all_pattern() {
    // `_` is a special pattern that matches any value and does not bind to that value.
    // This tells Rust we are not going to use the value, so Rust won't warn us about an unused variable.
    fn foo(dice_roll: u8) {
        match dice_roll {
            3 => println!("You got a 3!"),
            7 => println!("You got a 7!"),
            _ => println!("You got neither a 3 nor a 7, but a {dice_roll}!"),
        }
    }

    foo(3);
    foo(7);
    foo(10);
}
