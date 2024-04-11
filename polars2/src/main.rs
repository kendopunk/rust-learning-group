//use polars::prelude::*;

// fn main() -> Result<(), PolarsError> {
//     //     let f1 = "/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars2/data/test.json";

//     //     let df = JsonReader::new(&mut f1).finish()?;
//     //     //let df = JSONReader::from_path(f1).unwrap().finish()?;

//     //     println!("{:?}", df);

//     //     Ok(())
//     // }

//     println!("alsdfs");
// }
#![allow(dead_code, unused_variables, unused_imports)]
use polars::{
    error::PolarsError,
    io::{json::JsonReader, SerReader},
};

fn main() -> Result<(), PolarsError> {
    // let file =
    //     "/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars2/data/nested_newline.json";

    // let mut file = std::fs::File::open(file).unwrap();
    // let df = JsonReader::new(&mut file).finish().unwrap();
    // println!("{:?}", df);

    // let foo = df.select(["staff"]).expect("column selection error");
    // println!("{:?}", foo.get_row(0));

    // let file = "/Users/mfehr/workspace/sandbox/rust/rust-learning-group/polars2/data/nested.jsonl";

    // let mut file = std::fs::File::open(file).unwrap();
    // let df = JsonLineReader::new(&mut file).finish().unwrap();
    // println!("{:?}", df);

    Ok(())
}

/*
use polars::{
  error::PolarsError,
  io::{json::JsonReader, SerReader},
};

fn main() -> Result<(), PolarsError> {

  let file = "/path/to/your/file.json";

  let mut file = std::fs::File::open(file).unwrap();
  let df = JsonReader::new(&mut file).finish().unwrap();
    println!("{:?}", df);

  Ok(())
}
* */
