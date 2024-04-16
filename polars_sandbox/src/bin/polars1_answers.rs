/****************************************
 * cargo run --bin polars1_answers
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    println!("Hello Polars1");

    // CHANGE THIS TO MATCH THE LOCATION ON YOUR FILESYSTEM
    let filepath = "/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars1/src/data/organizations-100.csv";

    // ////////////////////////////////////////////////
    // @TODO 1
    // Load the organizations-100.csv file into a Polars DataFrame
    // ////////////////////////////////////////////////
    let df = CsvReader::from_path(filepath).unwrap().finish()?;
    assert_eq!(df.shape(), (100, 9));

    // ////////////////////////////////////////////////
    // @TODO 2
    // Print the data types (dtypes) and shape of the DF
    // ////////////////////////////////////////////////
    println!("{:?}", df.dtypes());
    println!("{:?}", df.shape());

    // ////////////////////////////////////////////////
    // @TODO 3
    // The "Industry" column has some empty (NULL) values
    // how many NULL values are there?
    // ////////////////////////////////////////////////
    println!("{:?}", df.null_count());

    // ////////////////////////////////////////////////
    // @TODO 4
    // Fill Industry NULL values with "Unknown"
    // ////////////////////////////////////////////////
    let df = df
        .clone()
        .lazy()
        .with_columns([col("Industry").fill_null(lit("Unknown"))])
        .collect()?;

    // ////////////////////////////////////////////////
    // @TODO 5
    // Sort the DataFrame by name ASC and show the first 10
    // items of just the "Name" series
    // ////////////////////////////////////////////////
    let sorted_df = df
        .clone()
        .lazy()
        .sort(
            "Name",
            SortOptions {
                descending: false,
                nulls_last: false,
                ..Default::default()
            },
        )
        .select([col("Name")])
        .limit(10)
        .collect()?;
    println!("{:?}", sorted_df);

    // ////////////////////////////////////////////////
    // @TODO 6
    // Show the Industry with the most number of employees
    // ////////////////////////////////////////////////
    let industry_count_df = df
        .clone()
        .lazy()
        .group_by(["Industry"])
        .agg([col("NumEmployees").sum().alias("EmpCount")])
        .sort(
            "EmpCount",
            SortOptions {
                descending: true,
                nulls_last: false,
                ..Default::default()
            },
        )
        .limit(1)
        .collect()?;

    println!("{:?}", industry_count_df);

    // get the first row
    let row = industry_count_df.get(0).unwrap();
    assert_eq!(row[0], polars::prelude::AnyValue::String("Plastics"));
    assert_eq!(row[1], polars::prelude::AnyValue::Int64(25894));

    // ////////////////////////////////////////////////
    // @TODO 7
    // Count the total number of employees in organizations
    // under the "Legal Services" industry category
    // ////////////////////////////////////////////////
    let legal_services_emp_count_df = df
        .clone()
        .lazy()
        .group_by(["Industry"])
        .agg([col("NumEmployees").sum().alias("EmpCount")])
        .filter(col("Industry").eq(lit("Legal Services")))
        .collect()?;

    println!("{:?}", legal_services_emp_count_df);

    // get the first row
    let row = legal_services_emp_count_df.get(0).unwrap();
    assert_eq!(row[1], polars::prelude::AnyValue::Int64(8360));

    // ////////////////////////////////////////////////
    // @TODO 8
    // THIS ONE IS CHALLENGING
    // Count the number of organizations where
    // the "Website" series value starts with "https"
    // [hint]: there is a "starts_with" function in one of the features
    // ////////////////////////////////////////////////
    let https_df = df
        .clone()
        .lazy()
        .select([col("Website")
            .str()
            .starts_with(lit("https"))
            .alias("IsHttps")])
        .filter(col("IsHttps").eq(lit(true)))
        .group_by(["IsHttps"])
        .agg([col("IsHttps").sum().alias("TotalHttpsWebsites")])
        .collect()?;

    println!("{:?}", https_df);

    Ok(())
}
