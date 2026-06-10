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
