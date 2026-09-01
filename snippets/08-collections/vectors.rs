// Vec<T>, also known as a vector.
//
// They allow you to store more than one value in a single data structure that
// puts all the values next to each other in memory.
//
// Vectors can only store values of the same type.


fn main() {
    empty_vector();
    init_vector_with_vec_macro();
    updating_vector();
    reading_values_in_vector();
    ownership_and_borrowing_rules_in_vector();
    iterating_over_vector();
    enum_for_multiple_types();
    dropping_vector();
}

fn empty_vector() {
    // Rust does not the kind of values we store, so we need type annotation here.
    let v: Vec<i32> = Vec::new();
}

fn init_vector_with_vec_macro() {
    // Vec<i32>
    let v = vec![1, 2, 3];
}

fn updating_vector() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);
    v.push(3);
}

fn reading_values_in_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Rust will panic for this `let does_not_exist = &v[100];`
    // whereas, we get a `None` for this:
    let does_not_exist = v.get(100);
}

fn ownership_and_borrowing_rules_in_vector() {
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];

    v.push(6);

    // The following code wont' compile: `println!("The first element is: {first}");`.
    //
    // In particular, we are breaking the following rule: "You can't have mutable and immutable references in the same scope".
}

fn iterating_over_vector() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

fn enum_for_multiple_types() {
    // To overcome the fact that a vector stores elements of the same type,
    // we can use an enum.
    //
    // Rust needs to know what types will be in vector at compile time,
    // so that it knows exactly how much memory on the help will be needed to store each element.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

fn dropping_vector() {
    // When the vector gets dropped, all of its contents are also dropped,
    // meaning the integers it holds will be cleaned up.
    //
    // The borrow checker ensures that any references to contents of a vector are only used
    // while the vector itself is valid.
    {
        let v = vec![1, 2, 3];
    } // <- `v` goes out of scope and is freed here.
}
