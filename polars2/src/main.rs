use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    let df = df![
      "Fruit" => ["Apple", "Apple", "Pear"],
      "Color" => ["red", "yellow", "green"],
      "Buyer" => [Some("Mark"), None, Some("John")],
      "Count" => [Some(1000), None, Some(100)]
    ]?;

    let interp_df = df
        .clone()
        .lazy()
        .with_columns([col("Count").interpolate(InterpolationMethod::Linear)])
        .collect()?;

    println!("{:?}", interp_df);

    Ok(())
}
