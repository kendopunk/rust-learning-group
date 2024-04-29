/****************************************
 * cargo run --bin polars3
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use polars::{lazy::dsl::col, prelude::*};
fn main() -> Result<(), PolarsError> {
    let data = ["2024-01-03", "2024-03-17", "2024-04-04", "2024-04-07"];

    let df = df!(
      "Date" => data
    )?
    .clone()
    .lazy()
    .with_columns([col("Date").str().to_date(StrptimeOptions {
        format: Some("%Y-%m-%d".into()),
        strict: false,
        exact: true,
        cache: false,
    })])
    .collect()?;

    // assuming we have a valid datetime type

    let df_with_year = df
        .clone()
        .lazy()
        .with_columns([col("Date").dt().century().alias("Century")])
        .collect()?;

    println!("{:?}", df_with_year);

    Ok(())
}
