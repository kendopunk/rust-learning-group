/****************************************
 * cargo run --bin polars1
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use polars::prelude::*;

fn main() -> Result<(), PolarsError> {
    println!("Hello Polars1");

    let filepath = "/path/to/your/organizations-100.csv";

    // ////////////////////////////////////////////////
    // @TODO 1
    // Load the organizations-100.csv file into
    // a Polars DataFrame
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df = ...
    // assert_eq!(df.shape(), (100, 9));

    // ////////////////////////////////////////////////
    // @TODO 2
    // Print the data types (dtypes) and shape of the DF
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]

    // ////////////////////////////////////////////////
    // @TODO 3
    // The "Industry" column has some empty (NULL) values
    // how many NULL values are there?
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]

    // ////////////////////////////////////////////////
    // @TODO 4
    // Fill Industry NULL values with "Unknown"
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let df = df ...

    // ////////////////////////////////////////////////
    // @TODO 5
    // Sort the DataFrame by name ASC and show the first 10
    // items of just the "Name" series
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let sorted_df = df ...
    // println!("{:?}", sorted_df);

    // ////////////////////////////////////////////////
    // @TODO 6
    // Show the Industry with the most number of employees
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let industry_count_df = df ...
    // println!("{:?}", industry_count_df);

    // get the first row
    // let row = industry_count_df.get(0).unwrap();
    // assert_eq!(row[0], polars::prelude::AnyValue::String("Plastics"));
    // assert_eq!(row[1], polars::prelude::AnyValue::Int64(25894));

    // ////////////////////////////////////////////////
    // @TODO 7
    // Count the total number of employees in organizations
    // under the "Legal Services" industry category
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let legal_services_emp_count_df = df ...
    // println!("{:?}", legal_services_emp_count_df);

    // get the first row
    // let row = legal_services_emp_count_df.get(0).unwrap();
    // assert_eq!(row[1], polars::prelude::AnyValue::Int64(8360));

    // ////////////////////////////////////////////////
    // @TODO 8
    // THIS ONE IS CHALLENGING
    // Count the number of organizations where
    // the "Website" series value starts with "https"
    // [hint]: there is a "starts_with" function in one of the features
    // ////////////////////////////////////////////////
    // [YOUR CODE HERE]
    // let https_df = df ...

    Ok(())
}
