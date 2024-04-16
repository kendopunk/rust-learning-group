# Polars1 (Session 6) Homework

## 1) Solve the @TODO Tasks in polars1/src/main.rs

1. Look for // @TODO comments for code you need to write / provide
1. Run `$> cargo run --bin polars1` to check code
1. My solutions are in polars_sandbox/src/bin/polars1_answers.rs
1. Happy Rusting

The data file for the homework is located in src/data/organizations-100.csv. It has 100 rows of data and 9 columns:

- Index (numeric)
- OrganizationId (string)
- Name (string)
- Website (string)
- Country (string)
- Description (string)
- Founded (numeric)
- Industry (string)
- NumEmployees (numeric)

Loading a CSV can be tricky when it comes to file paths, your current working directory and `cargo run`. It is recommended you use an absolute file path when trying to open the CSV file.
