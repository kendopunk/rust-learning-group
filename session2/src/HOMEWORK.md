# Session 2 Homework

## 1) Working with cargo, crates.io and Dependencies

1. We're building a simple Rust program that needs to convert between camel (HelloWorld), kebab (hello-world) and snake (hello_world) cases.
1. Create a new `cargo` project and name it whatever you'd like.
1. Using [crates.io](https://crates.io/), see if you can find a package that does this kind of case conversion (it does exist).
1. Follow the instructions on crates.io to add this dependency to your project
1. Write a program which converts "hello world":
   1. From snake to camel
   1. From snake to kebab
   1. from kebab to camel
   1. from kebab to snake
   1. from camel to snake
   1. from came to kebab
1. Suggest using the `assert_eq!` macro rather than `println!`

```rust
fn main() {
  assert_eq!("camelCase", your_snake_to_camel_function("camel_case"));
}
```

## 2) Fix main.rs in session2

1. session2 `main.rs` has several intentional errors
1. Remove the block comments from the words `UNCOMMENT HERE...` to the words `...TO HERE`
1. ...or pull out blocks of code individually outside the comments and work on them one at a time.
1. Run `$> cargo run --bin session2` and reviewing the errors and warnings
1. Fix the problems using the Rust Analyzer extension and the compiler output

---

## 3) Fix test.rs in session2

1. `test.rs` also has intentional errors as well
1. Remove the block comments from the words `UNCOMMENT HERE...` to the words `...TO HERE`
1. Using `$> cargo test --bin session2`
1. Using what you learned in the second lesson, fix the test code.

---
