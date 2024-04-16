/****************************************
 * Use this file for practice / sandboxing
 * cargo run --bin polars_playground
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]

use polars::prelude::*;
use polars_ops::pivot::*;

fn main() -> Result<(), PolarsError> {
    let df = df! [
        "Fruit" => ["apple", "pear", "orange", "apple"],
        "Color" => ["red", "green", "orange", "golden"],
        "Round" => ["fred", "marge", "chad", "wopper"],
        "PricePerPound" => [1.25, 1.39, 1.79, 1.17],
        "Count" => [20, 10, 7, 15]
    ]
    .unwrap();

    let df_pivot = pivot_stable(
        &df,
        ["Color"],
        ["Fruit"],
        Some(["Count", "PricePerPound"]),
        true,
        Some(PivotAgg::Sum),
        Some("->"),
    )?;

    println!("{}", df_pivot);
    Ok(())
}
