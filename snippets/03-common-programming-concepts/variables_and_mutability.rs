const A_GLOBAL: u32 = 10;

fn main() {
    mutability();
    constants();
    shadowing();
}

fn mutability() {
    // Variables are immutable by default.
    let x = 5;
    println!("x is {x}");

    // x = 10; // !! Compiler error !!

    // Use `mut`.
    let mut x = 10;
    println!("x is {x}");
    x = 11;
    println!("x is {x}");
}

fn constants() {
    // Constants can be defined in any scope.
    const A_LOCAL: u32 = 1;
    println!("A_LOCAL is {A_LOCAL}");
    println!("A_GLOBAL is {A_GLOBAL}");
}

fn shadowing() {
    // You can declare a new variable with the same name as a previous variable.
    let x = 5;
    let x = x + 1;
    {
        let x = x + 2;
        assert!(x == 8);
    }
    assert!(x == 6);
}
