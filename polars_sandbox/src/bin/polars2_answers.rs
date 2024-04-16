/****************************************
 * cargo run --bin polars2_answers
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    // ////////////////////////////////////////////////
    // @TODO 1
    // Load albums.json, customers.jsonl and orders.csv into
    // dataframes called, respectively:
    // - df_albums (albums.json)
    // - df_customers (customers.jsonl)
    // - df_orders (orders.csv)
    // ////////////////////////////////////////////////
    let mut album_file = std::fs::File::open(
        "/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars2/data/albums.json",
    )?;
    let df_albums = JsonReader::new(&mut album_file).finish()?;
    // println!("{:?}", df_albums);

    let mut customer_file = std::fs::File::open(
        "/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars2/data/customers.jsonl",
    )?;
    let df_customers = JsonLineReader::new(&mut customer_file).finish()?;
    // println!("{:?}", df_customers);

    // ////////////////////////////////////////////////
    // @TODO (lesson 6 review)
    // Show the one album with earliest release date
    // display only the title, artist and release date
    // ////////////////////////////////////////////////
    let df_release = df_albums
        .clone()
        .lazy()
        .sort(
            "released",
            SortOptions {
                descending: false,
                nulls_last: true,
                ..Default::default()
            },
        )
        .limit(1)
        .collect()?
        .select(["title", "artist", "released"]);
    // println!("{:?}", df_release);

    let df_orders = CsvReader::from_path(
        "/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars2/data/orders.csv",
    )
    .unwrap()
    .finish()?;
    // println!("{:?}", df_orders);

    // ////////////////////////////////////////////////
    // @TODO
    // Some of the album prices in the
    // orders DF are missing, so please interpolate
    // some values
    // ////////////////////////////////////////////////
    let df_orders = df_orders
        .clone()
        .lazy()
        .with_columns([col("price").interpolate(InterpolationMethod::Linear)])
        .collect()?;
    // println!("{:?}", df_orders);

    // ////////////////////////////////////////////////
    // @TODO
    // WHICH ACTIVE CUSTOMERS BOUGHT SOMETHING?
    // Show active customers who bought an album
    // ////////////////////////////////////////////////
    let df_customer_active = df_customers
        .clone()
        .lazy()
        .filter(col("active").eq(lit(true)))
        .join(
            df_orders.clone().lazy(),
            [col("id")],
            [col("customerId")],
            JoinArgs::new(JoinType::Inner),
        )
        .collect()?;
    // println!("{:?}", df_customer_active);

    // ////////////////////////////////////////////////
    // @TODO
    // WHO DID NOT BUY SOMETHING?
    // Create a DataFrame which only shows the
    // customer(s) who DID NOT buy an album
    // BONUS: show only the customer name + location
    // ////////////////////////////////////////////////
    let df_no_album_purchase = df_customers
        .clone()
        .lazy()
        .join(
            df_orders.clone().lazy(),
            [col("id")],
            [col("customerId")],
            JoinArgs::new(JoinType::Left),
        )
        .filter(col("id_right").is_null())
        .collect()?
        .select(["name", "location"]);
    // println!("{:?}", df_no_album_purchase);

    // ////////////////////////////////////////////////
    // @TODO
    // HOW MUCH DID EACH CUSTOMER PAY
    // Calculate what each customer paid for their
    // album(s) and just show the name and total price
    // ////////////////////////////////////////////////
    let df_customer_total = df_orders
        .clone()
        .lazy()
        .join(
            df_customers.clone().lazy(),
            [col("customerId")],
            [col("id")],
            JoinArgs::new(JoinType::Inner),
        )
        .join(
            df_albums.clone().lazy(),
            [col("albumId")],
            [col("id")],
            JoinArgs::new(JoinType::Inner),
        )
        .with_column((col("quantity") * col("price")).alias("Cost"))
        .group_by(["name"])
        .agg([col("Cost").sum().alias("TotalPurchase")])
        .collect()?;
    // println!("{:?}", df_customer_total);

    Ok(())
}
