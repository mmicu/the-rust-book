fn main() {
    scalar_types();
    compound_types();
}

fn scalar_types() {
    fn integers() {
        let a: i8 = -1;
        let b: u8 = 1;
        println!("a is {a}");
        println!("b is {b}");
    }

    fn floats() {
        let a: f32 = 1.21;
        let b: f64 = 2.96;
        println!("a is {a}");
        println!("b is {b}");
    }

    fn booleans() {
        let t = true;
        let f = false;
        println!("t is {t}");
        println!("f is {f}");
    }

    fn chars() {
        let c = 'a';
        println!("c is {c}");
        let c = '🧐';
        println!("c is {c}");
    }

    integers();
    floats();
    booleans();
    chars();
}

fn compound_types() {
    fn tuples() {
        // Fixed length, they cannot grow or shrink in size,
        // but they can contain mixed types.
        let t: (i32, f64, bool) = (1, 2.1, false);
        println!("Tuple: {:?}", t);

        // `destructuring`.
        let (a, b, c) = t;

        // `t.<i>` to access the i-th element.
        println!("a is {a}");
        assert!(a == t.0);

        println!("b is {b}");
        assert!(b == t.1);

        println!("c is {c}");
        assert!(c == t.2);

        // Empty tuple -> `unit`.
        let t = ();
        println!("Empty tuple -> `unit`: {:?}", t);
    }

    fn arrays() {
        // Elements must have the same type and fixed length.
        //
        // Data allocated in the stack.
        let a = [1, 2, 3, 4, 5];
        println!("Array: {:?}", a);

        let a: [i32; 3] = [1, 2, 3];
        println!("Array: {:?}", a);

        let a = [0; 10];
        println!("Array: {:?}", a);

        let a = [1, 2];
        println!("Array: {:?}", a);
        let first = a[0];
        let second = a[1];
        println!("First: {first}");
        println!("Second: {second}");

        // let third = a[2]; // Compiler error.
    }

    tuples();
    arrays();
}
