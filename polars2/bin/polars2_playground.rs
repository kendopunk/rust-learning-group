/****************************************
 * Use this file for practice / sandboxing
 * cargo run --bin polars2_playground
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = df![
      "Fruit" => ["Apple", "Apple", "Pear"],
      "Color" => ["red", "yellow", "green"],
      "Buyer" => [Some("Justin"), None, Some("Jakko")],
      "Count" => [Some(30), Some(20), None]
    ]?;

    let expression_df = df
        .clone()
        .lazy()
        .with_columns([col("Count").fill_null(median("Count"))])
        .collect()?;

    println!("{:?}", expression_df);

    Ok(())
}
