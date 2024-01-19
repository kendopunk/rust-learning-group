# Session 1 Homework

## 1) Install Rust

[Installation instructions here](https://www.rust-lang.org/tools/install)

If using VSCode, install the [Rust Analyzer](https://code.visualstudio.com/docs/languages/rust) extension to assist with code completion, hints, etc.

---

## 2) Create a new project using cargo in a separate folder outside of this project, like your home directory

```sh
$> cd some_directory
$> cargo new my_project
$> cd my_project
```

1. View the contents and structure of the new project
1. Run `$> cargo run` from the command line. What do you see?
1. Look at the contents of the `target` folder created as part of `cargo run`
1. On the command line, run the executable created in `target`, i.e. `$> ./target/debug/my_project`
1. Create a release build using `$> cargo build --release`. What do you see in the `target` directory now?
1. Compare the byte sizes of the debug and release build.

---

## 3) Fix main.rs in session1

1. session1 `main.rs` has several intentional errors
1. Remove the block comments from the words `UNCOMMENT HERE...` to the words `...TO HERE`
1. ...or pull out blocks of code individually outside the comments and work on them one at a time.
1. Run `$> cargo run --bin session1` and reviewing the errors and warnings
1. Fix the problems using the Rust Analyzer extension and the compiler output
1. There will be some code we haven't covered yet, like `mod test` and `#[allow(dead_code)]`. Disregard for now

---

## 4) Fix the tests

1. `test.rs` also has intentional errors as well
1. Remove the block comments from the words `UNCOMMENT HERE...` to the words `...TO HERE`
1. Using `$> cargo test --bin session1`
1. Using what you learned in this first lesson, fix the test code.
1. If you want to know about test assertions, i.e. `assert_eq!`, [see here](https://doc.rust-lang.org/std/macro.assert_eq.html)

---

## 5) Generate the documentation

1. `$> cargo doc`
1. Check the `target` directory to see where the documentation was generated
1. Open the `index.html` for session1 in the browser of your choice.
1. Run `$> cargo clean` to remove the `target` directory
