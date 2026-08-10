struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    structs_examples();
    tuple_structs();
    unit_structs();
}

fn structs_examples() {
    fn build_user_verbose(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }

    fn build_user_less_verbose(email: String, username: String) -> User {
        // We can use the *field init shorthand* syntax to rewrite `build_user_verbose` so that it behaves exactly the same,
        // but does not have the repetition of `username` and `email`.
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }


    // Create an `instance`.
    let user1 = User {
        active: true,
        username: String::from("some_user"),
        email: String::from("some_user@foo.com"),
        sign_in_count: 1,
    };

    // Rust does not allow us to mark only certain fields as mutable.
    // The entire struct must be mutable.
    let mut user2 = User {
        active: true,
        username: String::from("some_user"),
        email: String::from("some_user@foo.com"),
        sign_in_count: 1,
    };
    user2.active = false;

    // Create a new instance **without** the update syntax.
    let user3 = User {
        active: user1.active,
        username: user1.username.clone(),
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Create a new instance **with** the update syntax.
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // Data is moved, so we cannot use `user1` anymore.
}

fn tuple_structs() {
    // Tuple structs do not have names associated with their fields.
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let Point(x, y, z) = origin;
    let Color(x, y, z) = black;
}

fn unit_structs() {
    // Unit-like structs are structs that do not have any fields.
    struct AlwaysEqual;

    let subject = AlwaysEqual;
}
