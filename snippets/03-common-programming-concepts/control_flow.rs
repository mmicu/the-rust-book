fn main() {
    if_expressions(2);
    if_expressions(3);
    if_expressions(4);

    if_and_let();

    loops();
}

fn if_expressions(number: i32) {
    if number == 2 {
        println!("It's a 2!");
    } else if number == 3 {
        println!("It's a 3!");
    } else {
        println!("It's *not* a 2 nor a 3!");
    }
}

fn if_and_let() {
    // `if` is an expression.
    // We can use it on the right side of a `let` statement to assign the outcome to a variable.
    let x = if true { 1 } else { 2 };
    assert!(x == 1);

    // Compiler error! Both branches must return the same type.
    // let number = if condition { 5 } else { "six" };
}

fn loops() {
    // Three kinds of loops: `loop`, `while`, and `for`.

    // This runs forever: loop { println!("Hi!"); }
    loop {
        println!("Hello!");
        break;
    }

    // Loops can return values.
    let mut i = 0;
    let result = loop {
        i += 1;
        if i == 10 {
            break i * 10; // Equivalent: `return i * 10;`
        }
    };
    assert!(result == 100);

    // Loops with labels.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    // `while` loops.
    let mut i = 5;
    while i > 0 {
        println!("i = {i}");
        i -= 1;
    }

    // Looping Through a Collection with `for`.
    let a = [1, 2, 3, 4, 5];
    for val in a {
        println!("val = {val}");
    }
}
