/****************************************
 * cargo run --bin polars2
 * @See polars_2_answers.rs for my solutions
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    println!("Hello Polars2");

    // ////////////////////////////////////////////////
    // @TODO 1
    // Load albums.json, customers.jsonl and orders.csv into
    // dataframes called, respectively:
    // - df_albums (albums.json)
    // - df_customers (customers.jsonl)
    // - df_orders (orders.csv)
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df_albums =
    // let df_customers =
    // let df_orders =

    // ////////////////////////////////////////////////
    // @TODO 2 (lesson 6 review)
    // Show the one album with earliest release date
    // display only the title, artist and release date
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df_release =

    // ////////////////////////////////////////////////
    // @TODO 3 (lesson 6 review)
    // Calculate a single value representing the average
    // spend across all orders
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df_avg_spend =

    // ////////////////////////////////////////////////
    // @TODO 4
    // Some of the album prices in the
    // orders DF are missing, so please interpolate
    // some values
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]

    // ////////////////////////////////////////////////
    // @TODO 5
    // WHICH ACTIVE CUSTOMERS BOUGHT SOMETHING?
    // Show only active customers who bought an album
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df_customer_active =

    // ////////////////////////////////////////////////
    // @TODO 6
    // WHO DID NOT BUY SOMETHING?
    // Create a DataFrame which only shows the
    // customer(s) who DID NOT buy an album
    // BONUS: show only the customer name + location
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df_no_album_purchase =

    // ////////////////////////////////////////////////
    // @TODO 7
    // HOW MUCH DID EACH CUSTOMER PAY?
    // Calculate spend for each customer
    // and show the customer's name and their total spend
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df_customer_total =

    Ok(())
}
