# Session 1 Homework

## 1) Install Rust

[Installation instructions here](https://www.rust-lang.org/tools/install)

If using VSCode, install the [Rust Analyzer](https://code.visualstudio.com/docs/languages/rust) extension to assist with code completion, hints, etc.

---

## 2) Create a new project using cargo

```sh
$> cd some_directory
$> cargo new my_project
$> cd my_project
```

1. View the contents and structure of the new project
2. Run `$> cargo run` from the command line. What do you see?
3. Look at the contents of the `target` folder created as part of `cargo run`
4. On the command line, run the executable created in `target`, i.e. `$> ./target/debug/my_project`
5. Create a release build using `$> cargo build --release`. What do you see in the `target` directory now?
6. Compare the byte sizes of the debug and release build.

---

## 3) Fix main.rs in session1-intro

1. The `main.rs` has several intentional errors
2. Run `$> cargo run --bin session1-intro` and reviewing the errors and warnings
3. Fix the problems using the Rust Analyzer extension and the compiler output
4. There will be some code we haven't covered yet, like `mod test` and `#[allow(dead_code)]`. Disregard for now

---

## 4) Fix the tests

1. The `test.rs` file has errors as well.
2. Using `$> cargo test --bin session1-intro`
3. Using what you learned in this first lesson, fix the test code.
4. If you want to know about test assertions, i.e. `assert_eq!`, [see here](https://doc.rust-lang.org/std/macro.assert_eq.html)

---

## 5) Generate the documentation

1. `$> cargo doc`
2. Check the `target` directory to see where the documentation was generated
3. Open the `index.html` for session1-intro in the browser of your choice.
4. Run `$> cargo clean` to remove the `target` directory
