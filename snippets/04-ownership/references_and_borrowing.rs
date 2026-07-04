fn main() {
    let s = String::from("hello");

    // Note: The opposite of referencing by using `&` is dereferencing,
    // which is accomplished with the dereference operator, `*`.
    print_string(&s);

    let mut s = String::from("hello");
    mutable_reference(&mut s);
    println!("{s}");
    print_string(&s);

    let r1 = &mut s;
    let r2 = &mut s;
    // println!("{r1}, {r2}"); // **Compiler error**: It's not possible to ave two mutable references at the same time
                               //                     on the same variable.

    // New scope can be created, allowing multiple mutable references (**not simultaneous ones**):
    {
        let r1 = &mut s;
        print_string(&r1);
    }
    let r2 = &mut s;
    print_string(&r2);

    // Not possible to have a mix of immutable and mutable references.
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    let r3 = &mut s; // BIG PROBLEM
    // println!("{r1}, {r2}, and {r3}"); // **Compiler error**

    // A reference's scope starts from where it is introduced and continues through the last time that reference is used.
    // Hence, this is valid, since **the scopes don't overlap**:
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1} and {r2}");
    // Variables `r1` and `r2` will not be used after this point.

    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn print_string(s: &String) {
    println!("{s}");
} // Here, `s` goes out of scope. But because `s` does not have ownership of what
  // it refers to, the `String` is not dropped.

fn mutable_reference(s: &mut String) {
    s.push_str(", world");
}
