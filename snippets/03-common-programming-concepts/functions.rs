fn another_function_before_main() {
    println!("Another function (before main)!");
}

fn main() {
    another_function_after_main();
    another_function_before_main();

    function_with_params(10, 'c');
    function_with_params(110, 'a');

    statements_and_expressions();

    let x = function_return_five();
    assert!(x == 5);

    let x = function_plus_one(10);
    assert!(x == 11);
}

fn another_function_after_main() {
    println!("Another function (after main)!");
}

fn function_with_params(x: i32, c: char) {
    println!("x is {x}");
    println!("c is {c}");
}

fn statements_and_expressions() {
    // Statements:  instructions that perform some action and *do not* return a value.
    // Expressions: evaluate to a resultant value.

    let y = 1; // -> `statement`. It *does not* return a value.

    let y = {
        let x = 1;
        x + 2 // -> `expression`. Note the missing semicolon here.
    };
}

fn function_return_five() -> u32 { 5 }

fn function_plus_one(x: u32) -> u32 { x + 1 }
