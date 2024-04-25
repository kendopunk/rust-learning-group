/****************************************
 * Example of a join w/ aggregations
 * cargo run --bin polars2_agg_join
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]

use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    // fruits ("products")
    let df_fruits = df!(
      "ID" => [1, 2, 3, 4],
      "Fruit" => ["Apple", "Banana", "Orange", "Pear"],
      "UnitPrice" => [0.85, 0.25, 1.17, 1.25]
    )?;

    // customers
    let df_customers = df!(
      "ID" => [101, 102, 103, 104],
      "Name" => ["Andrew", "Calvin", "Jakko", "Justin"],
      "Location" => ["Maryland", "Oregon", "Netherlands", "Pennsylvania"],
    )?;

    // orders
    let df_orders = df!(
      "ID" => [501, 502, 503, 504, 505, 506, 507, 508],
      "FruitId" => [1, 2, 1, 2, 3, 2, 3, 2],
      "CustomerId" => [104, 104, 102, 102, 102, 103, 103, 1234],
      "Quantity" => [5, 10, 3, 7, 1, 14, 4, 7]
    )?;

    /*
     * Calculate the total purchase made by each customer
     */
    let df_agg_join = df_orders
        .clone()
        .lazy()
        .join(
            df_customers.clone().lazy(),
            [col("CustomerId")],
            [col("ID")],
            JoinArgs::new(JoinType::Inner),
        )
        .join(
            df_fruits.clone().lazy(),
            [col("FruitId")],
            [col("ID")],
            JoinArgs::new(JoinType::Inner),
        )
        .with_column((col("Quantity") * col("UnitPrice")).alias("Cost"))
        .group_by(["Name"])
        .agg([col("Cost").sum().alias("TotalPurchase")])
        .collect()?;

    println!("{:?}", df_agg_join);

    Ok(())
}
