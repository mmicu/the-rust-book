# The Rust book
Personal notes about [the Rust book](https://doc.rust-lang.org/book/).

## [1. Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)

### Installation
```
$ curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
$ . "$HOME/.cargo/env"
$ rustc --version
```

### Update
```
$ rustup update
```

### Docs
```
$ rustup doc --book
```

### Hello, world!
[main.rs](./snippets/01-getting-started/hello-world/main.rs):

```
$ cd ./snippets/01-getting-started/hello-world
$ rustc main.rs
$ ./main
```

### Hello, Cargo!
Cargo is Rust's build system and package manager.

```
$ cargo --version
$ cd ./snippets/01-getting-started
$ cargo new hello_cargo
$ cd hello_cargo
$ cargo run
```

To create `Cargo.toml`, you can run `cargo init`.

## [2. Programming a Guessing Game](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html)
Skip for now.

## [3. Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)

### [Variables and Mutability](./snippets/03-common-programming-concepts/variables_and_mutability.rs)

### [Data Types](./snippets/03-common-programming-concepts/data_types.rs)
Every value in Rust has a certain data type, which is specified or inferred by the compiler (if possible).
Two data types:

1. `Scalar`, single value:
    1. [Integers](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-types). How Rust handles [Integer Overflow](https://doc.rust-lang.org/book/ch03-02-data-types.html#integer-overflow).
    1. [Floating-point numbers](https://doc.rust-lang.org/book/ch03-02-data-types.html#floating-point-types).
    1. [Booleans](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-boolean-type).
    1. [Characters](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-character-type):
        - 4 bytes.
        - Represents a Unicode scalar from `U+0000` to `U+D7FF` and `U+E000` to `U+10FFFF`.
1. `Compound`, group multiple values into one type:
    1. [Tuples](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-tuple-type).
    1. [Arrays](https://doc.rust-lang.org/book/ch03-02-data-types.html#the-array-type).

### [Functions](snippets/03-common-programming-concepts/functions.rs)
