/****************************************
 * cargo run --bin polars3
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use chrono::NaiveDate;
use polars::{lazy::dsl::col, prelude::*};
fn main() -> Result<(), PolarsError> {
    // ////////////////////////////////////////////////
    // @TODO 1 (Loading Data)
    // Load the CSV files into
    // "df_products", "df_dist" and "df_stocks" respectively
    // ////////////////////////////////////////////////
    // assert_eq!(df_products.shape(), (500, 7));
    // assert_eq!(df_dist.shape(), (3, 4));
    // assert_eq!(df_stocks.shape(), (21, 6));

    // ////////////////////////////////////////////////
    // @TODO 2 (Datetime)
    // Calculate the average close price for the stock
    // from April 17 to the end of the month (inclusive)
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 3 (SQL)
    // Create the SQL context and register the two DataFrames
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 4 (SQL)
    // Run a SHOW TABLES query
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 5 (SQL)
    // select the name, category and retail price of the
    // 10 most expensive products
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 6 (SQL)
    // Join products and distribution_ctr
    // where the brand is "Wayfarers"
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 7 (SQL)
    // What is the average price of men's socks
    // in the Houston distribution center
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 8 (SQL)
    // What is the total retail value of all
    // women's category products in the Chicago distribution center
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 9 (SQL)
    // Use CTE to create a temporary table called "memphis_jeans"
    // of category = Jeans and distribution center = Memphis
    // then select everything in the Women's department
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 10 (SQL)
    // Find the lowest price price product where
    // the product name ends with "Socks"
    // ////////////////////////////////////////////////

    // ////////////////////////////////////////////////
    // @TODO 11 (SQL and Datetime)...stock_data.csv
    // Find the average volume for the
    // first 7 days (1-6, inclusive) of the month
    // ////////////////////////////////////////////////

    Ok(())
}
