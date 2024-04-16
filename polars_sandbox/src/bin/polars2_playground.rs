/****************************************
 * Use this file for practice / sandboxing
 * cargo run --bin polars2_playground
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]

use polars::prelude::*;

fn main() {
    // let df = df! [
    //     "obj"        =>  ["ring", "shoe", "ring"],
    //     "price"   => [65, 42, 65],
    //     "value" => [53, 55, 54],
    //     "date"        =>["2022-02-07", "2022-01-07", "2022-03-07"]
    // ]
    // .unwrap();

    // let out = pivot(
    //     &df,
    //     ["price", "value"],
    //     ["obj"],
    //     Some(["date"]),
    //     true,
    //     Some(PivotAgg::Sum),
    //     Some("|"),
    // )
    // .unwrap();

    // /*
    // pivot_df: &DataFrame,
    //   index: I0,
    //   columns: I1,
    //   values: Option<I2>,
    //   sort_columns: bool,
    //   agg_fn: Option<PivotAgg>,
    //   separator: Option<&str>, */

    // //println!("{}", df);
    // println!("{}", out);
}
