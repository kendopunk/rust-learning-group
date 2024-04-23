/****************************************
 * cargo run --bin polars3_answers
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]
use polars::prelude::*;
use polars_sql::SQLContext;

fn main() -> Result<(), PolarsError> {
    let base_path =
        String::from("/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars3/data");
    let prod_path = format!("{base_path}/products.csv");
    let dist_path = format!("{base_path}/distribution_centers.csv");

    // ////////////////////////////////////////////////
    // @TODO 1
    // Load the CSV files into
    // "df_products" and "df_dist" respectively
    // ////////////////////////////////////////////////
    let df_products = CsvReader::from_path(prod_path).unwrap().finish()?;
    let df_dist = CsvReader::from_path(dist_path).unwrap().finish()?;
    assert_eq!(df_products.shape(), (500, 7));
    assert_eq!(df_dist.shape(), (3, 4));

    // ////////////////////////////////////////////////
    // @TODO 2
    // Create the SQL context and register the two DataFrames
    // ////////////////////////////////////////////////
    let mut ctx = SQLContext::new();
    ctx.register("products", df_products.lazy());
    ctx.register("distribution_ctr", df_dist.lazy());

    // ////////////////////////////////////////////////
    // @TODO 3
    // Run a SHOW TABLES query
    // ////////////////////////////////////////////////
    // let query = "SHOW TABLES";
    // println!("{:?}", ctx.execute(query).unwrap().collect().unwrap());

    // ////////////////////////////////////////////////
    // @TODO 4
    // select the name, category and retail price of the
    // 10 most expensive products
    // ////////////////////////////////////////////////
    // let query = "SELECT Name, Category, RetailPrice
    //   FROM products
    //   ORDER BY RetailPrice DESC
    //   LIMIT 10";
    // println!("{:?}", ctx.execute(query).unwrap().collect().unwrap());

    // ////////////////////////////////////////////////
    // @TODO 5
    // Join products and distribution_ctr
    // where the brand is "Wayfarers"
    // ////////////////////////////////////////////////
    // let query = "SELECT * from products p
    //   INNER JOIN distribution_ctr d
    //   ON p.DistributionCenterID = d.ID
    //   WHERE (p.Brand = 'Wayfarers')";
    // println!("{:?}", ctx.execute(query).unwrap().collect().unwrap());

    // ////////////////////////////////////////////////
    // @TODO 6
    // What is the average price of men's socks
    // in the Houston distribution center
    // ////////////////////////////////////////////////
    // let query = "SELECT AVG(p.RetailPrice) as average_sock_price
    //   FROM products p
    //   INNER JOIN distribution_ctr d
    //   ON p.DistributionCenterID = d.ID
    //   WHERE (p.Category = 'Socks' AND p.Department = 'Men' AND d.CenterName = 'Houston TX')";
    // println!("{:?}", ctx.execute(query).unwrap().collect().unwrap());

    // ////////////////////////////////////////////////
    // @TODO 7
    // What is the total retail value of all
    // women's category products in the Chicago distribution center
    // ////////////////////////////////////////////////
    // let query = "SELECT SUM(p.RetailPrice) as total_retail_value
    //   FROM products p
    //   INNER JOIN distribution_ctr d
    //   ON p.DistributionCenterID = d.ID
    //   WHERE (p.Department = 'Women' AND d.CenterName = 'Chicago IL')";
    // println!("{:?}", ctx.execute(query).unwrap().collect().unwrap());

    // ////////////////////////////////////////////////
    // @TODO 8
    // Use CTE to create a temporary table called "memphis_jeans"
    // of category = Jeans and distribution center = Memphis
    // then select everything in the Women's department
    // ////////////////////////////////////////////////
    // let query = "WITH memphis_jeans as (
    //   SELECT * from products p
    //   INNER JOIN distribution_ctr d
    //   ON p.DistributionCenterID = d.ID
    //   WHERE (p.Category = 'Jeans' AND d.CenterName = 'Memphis TN')
    // )
    // SELECT * FROM memphis_jeans WHERE (p.Department = 'Women')";
    // println!("{:?}", ctx.execute(query).unwrap().collect().unwrap());

    // ////////////////////////////////////////////////
    // @TODO 9
    // Find the lowest price price product where
    // the product name ends with "Socks"
    // ////////////////////////////////////////////////
    // let query = "SELECT min(RetailPrice) as lowest_price_socks
    //   FROM products
    //   WHERE ENDS_WITH(Name, 'Socks')";
    // println!("{:?}", ctx.execute(query).unwrap().collect().unwrap());
    Ok(())
}
