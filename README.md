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

### [Control Flow](snippets/03-common-programming-concepts/control_flow.rs)

## [4. Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
**Ownership** is a **set of rules** that govern how a Rust program manages memory.

### [What is ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

#### [The Stack and the Heap](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap)
Both the **stack** and the **heap** are parts of memory available to your code to use at runtime,
but they are structured in a different way.

* **Stack**:
    * Faster compared to the *heap*
    * **LIFO**
    * All data stored must have a known and fixed size
* **Heap**:
    * Slower compared to the *stack*
    * Stores data with unknown size at compile time or size that might change
    * Less organized
    * An allocator:
        * Finds an empty spot in the heap that is big enough
        * Marks it as in use
        * Returns a *pointer*, which is the address of that location
    * Thee *pointer* has a known size, hence it can be stored in the heap

#### [Ownership rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-rules)
1. Each value in Rust has an *owner*.
1. There can only be one *owner* at a time.
1. When the *owner* goes out of scope, the value will be *dropped*.

#### [Variable scope](./snippets/04-ownership/variable_scope.rs)

#### [The String type](./snippets/04-ownership/string.rs)

#### [Memory and Allocation](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation)
For the string literal, we know its content at compile time,
so the text is hardcoded directly into the final executable.

With the `String` type (mutable&growable), we need to allocate an amount of memory in the heap,
which is unknown at compile time:

1. Memory must be request at runtime by the memory allocator.
    - Done by us.
    - Done by using `String::from`.
1. Need a way to return this memory to the memory allocator when we are done with the `String`.
    - "The Rust approach".
    - Memory is automatically returned once **the variable** that **owns** it goes **out of scope**.

```rust
{
    let s = String::from("hey");
    // Use `s` from here.
} // Scope is over. `s` is not available anymore.
```

When a variable goes out of scope, Rust calls a special function for us: **drop** ([RAII in C++](https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization)).

##### [Variables and Data Interacting with Move](./snippets/04-ownership/move.rs)

##### [Scope and Assignment](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#scope-and-assignment)
In addition to calling `drop` after a variable goes out of scope,
Rust calls the same when you assign a completely new value to an existing variable.

```rust
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");
```

##### [Variables and Data Interacting with Clone](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#variables-and-data-interacting-with-clone)
We can use `clone` for deep copies (both `stack` and `heap` data):

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
println!("s1 = {s1}, s2 = {s2}");
```

##### [Stack-Only Data: Copy](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#stack-only-data-copy)
Why don't we need to call `clone` for the following?

```rust
let x = 5;
let y = x;

println!("x = {x}, y = {y}");
```

The reason is that types such as integers that have a **known size** at compile time are **stored entirely** on the **stack**,
so **copies** of the actual values are **quick** to make.

Rust has a special annotation called the `Copy` trait that we can place on **types that are stored on the stack**,
as integers are (TODO: link the Chapter 10).

`Copy` **cannot be used** if the type, or any of its parts, has implemented the `Drop` trait.
Here are some of the types that implement `Copy`:

* All the `integer` types.
* The `Boolean` type.
* All the `floating-point` types.
* The character type.
* `Tuples`, if they only contain types that also implement `Copy`.

#### [Ownership and Functions](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ownership-and-functions)
The mechanics of passing a value to a function are **similar** to those when assigning a value to a variable.
Passing a variable to a function will `move` or `copy`, just as assignment does.

For more details, check [ownership_functions](./snippets/04-ownership/ownership_functions.rs)

#### [Return Values and Scope](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)
Returning values can also transfer ownership.

The ownership of a variable follows the same pattern every time: **Assigning** a value to another variable **moves** it.
When a variable that includes data on the **heap** goes out of scope,
the value will be cleaned up by `drop` unless ownership of the data has been moved to another variable.

For more details, check [ownership_functions](./snippets/04-ownership/ownership_return_values.rs)

### [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
A reference is like a pointer that it's an address we can follow to access data stored at address.
**That data is owned by some other variable**.
Unlike a pointer, a reference is **guaranteed to point** to a valid value.

We call the action of creating a reference **borrowing**.

For more details, check [references_and_borrowing](./snippets/04-ownership/references_and_borrowing.rs)

#### [Mutable References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#mutable-references)
Mutable references have **one big restriction**: if you have a mutable reference to a value,
you can have no other references to that value.

The benefit of having such restriction is that **Rust can prevent data races at compile time**.
A *data race* is similar to a race condition and happens when these three behaviors occur:

1. Two or more pointers access the same data at the same time.
1. At least one of the pointer is being used to write to the data.
1. There's no mechanism being used to synchronize access to the data.

Also, it is **not possible** to have a mutable reference while we have an immutable one to the same value.

However, **multiple immutable references are allowed** (no data modified/written, hence same data remains).

As usual, the scope (in this case the reference's one) plays a crucial role.
In fact, the following code is valid:

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{r1} and {r2}");
// Variables `r1` and `r2` will not be used after this point.

let r3 = &mut s; // no problem
println!("{r3}");
```

#### [The Rules of References](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#the-rules-of-references)
* At any given time, you can have *either* one mutable reference or any number of immutable references.
* References must always be valid.

### [The slice type](https://doc.rust-lang.org/book/ch04-03-slices.html)
**Slices** let you **reference a contiguous sequence of elements** in a collection.
A slice is a kind of reference, so it does not have ownership.

For more details, check [references_and_borrowing](./snippets/04-ownership/references_and_borrowing.rs)

#### String Slices
For more details, check [references_and_borrowing](./snippets/04-ownership/string-slices.rs)

> Note: String slice range indices must occur at valid `UTF-8` character boundaries.
> If you attempt to create a string slice in the middle of a multibyte character, your program will exit with an error.

## [5. Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
A *struct*, or *structure*, is a custom data type that lets you package together and name multiple related values that make up a meaningful group.

In a way, structs are similar to tuples, with the difference that you can name each piece of data in the structs.

For more details, check:

- [structs.rs](./snippets/05-structs/structs.rs) for basic struct definitions.
- [structs-and-traits.rs](./snippets/05-structs/structs-and-traits.rs) for checking how to add functionality with derived traits.
- [structs-and-methods.rs](./snippets/05-structs/structs-and-methods.rs) for methods definitions on structs.

## [6. Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
Enumerations, also referred to as enums, allow you to define a type by enumerating its possible variants.
Enums give you a way of saying a value is one of a possible set of values.

For more details, check:

- [enums.rs](./snippets/06-enums/enums.rs) for basic enum definitions.
- [option-enum.rs](./snippets/06-enums/option-enum.rs) for describing the importance of the `Option` enum.
- [match-control-flow.rs](./snippets/06-enums/match-control-flow.rs) for describing the `match` control flow.
- [if-let.rs](./snippets/06-enums/if-let.rs) for describing `if let` and `let...else`.

## [7. Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
Rust has a number of features that allow you to manage your code's organization:

- *Packages*: a `Cargo` feature that lets you build, test, and share crates.
- *Crates*: a tree of modules that produces a library or executable.
- *Modules and use*: let you control the organization, scope, and privacy of paths.
- *Paths*: a way of naming an item, such as a struct, function, or module.

### [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html#packages-and-crates)
A *crate* is the **smallest amount of code** that the Rust compiler considers at a time.
Even if you run *rustc* rather than cargo and pass a single source code file,
the compiler considers that file to be a crate.
Such crates can contain modules.

A crate can come in one of two forms:

1. A **binary crate**: programs you can compile to an executable.
Each must have a function called `main`.
1. A **library crate**: they do not have a `main` function,
and they do not compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.

Also, the *crate root* is a source file that the Rust compiler starts from and
makes up the root module of your crate.

On the other hand, a *package* is a bundle of **one or more crates** that provides a set of functionality.
A package can contain as many binary crates as you like, but at most only one library crate.
Also, it must contain at least one crate, whether that's a library or binary crate.

```
$ cargo new foo
    Creating binary (application) `foo` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
$ ls foo
Cargo.toml      src
$ ls foo/src
main.rs
```

- `src/main.rs`: binary crate.
- `src/lib.rs`: library crate and corresponding crate root.

If a package contains *src/main.rs* and *src/lib.rs*,
it has two crates: a *binary* and a *library*, both with the same name as the package.
A package can have multiple binary crates by placing files in the *src/bin* directory: each file will be a separate binary crate.

### [Control Scope and Privacy with Modules](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#control-scope-and-privacy-with-modules)

#### [Modules Cheat Sheet](https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet)
1. Start from the **crate root**. Either `src/main.rs` or `src/lib.rs`.
1. Declaring **modules**: in the crate root file, you can declare new modules.
If you use `mod garden`, the compiler will look:
    1. Inline `{ ... }` instead of `;` in `mod garden`.
    1. `src/garden.rs`.
    1. `src/garden/mod.rs`.
1. Declaring **submodules**: in *any file* other than the crate root,
you can declare submodules. For example, `mod vegetables;` in `src/garden.rs`.
The compiler will look:
    1. Inline `{ ... }` instead of `;` in `mod vegetables`.
    1. `src/garden/vegetables.rs`.
    1. `src/garden/vegetables/mod.rs`.
1. Paths to code in modules: once a module is part of your crate,
you can refer to code in that module from anywhere else in that same crate (if privacy rules allow).
For example: `crate::garden::vegetables::Asparagus`.
1. Private/public: code within a module is private from its parent modules by default.
To make a module public, declare it with `pub mod` instead of `mod`.
1. The use keyword: within a scope, the use keyword creates shortcuts to items to reduce repetition of long paths.
For example, to refer to `crate::garden::vegetables::Asparagus`,
you can create a shortcut with `use crate::garden::vegetables::Asparagus;`,
and simple use `Asparagus`.

For more details, check [backyard](./snippets/07-packages-crates-modules/backyard).

### [Paths for Referring to an Item in the Module Tree](https://doc.rust-lang.org/book/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html#paths-for-referring-to-an-item-in-the-module-tree)
To call a function, we need to know its path.

A path can take two forms:

* An absolute path is the full path starting from a crate root.
* A relative path starts from the current module and uses `self`, `super`, or an identifier in the current module.

For more details, check:

*  [restaurant](./snippets/07-packages-crates-modules/restaurant/).
*  [restaurant-2](./snippets/07-packages-crates-modules/resturant-2/).
*  [backyard](./snippets/07-packages-crates-modules/backyard/).

Lastly, Rust also supports a different way for importing modules.
Check [Alternate File Paths](https://doc.rust-lang.org/book/ch07-05-separating-modules-into-different-files.html#alternate-file-paths).

## [8. Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
Rust's standard library includes a number of very useful data structures called *collections*,
which can contain multiple values.

Unlike the built-in array and tuple types, the data that these collections point to is stored to the heap.

Such collections are:

* [Vectors](./snippets/08-collections/vectors.rs).
* [Strings](./snippets/08-collections/strings.rs).
* [Hash maps](./snippets/08-collections/hash-maps.rs).
