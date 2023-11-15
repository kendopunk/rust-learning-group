use std::array;

mod test;

/// Add two 8-bit integers together
#[allow(dead_code)]
fn add_i8s(num1: i8, num2: i8) -> i8 {
    return num1 + num2;
}

/// Add two unsigned 32-bit integers together
#[allow(dead_code)]
fn add_u32s(num1: u32, num2: u32) -> u32 {
    // Note: an explicit "return" statement is not required
    // You can just return the expression without a semicolon

    // the line below is the same as "return num1 + num2;"
    num1 + num2
}

/// Add two double precision floats together
#[allow(dead_code)]
fn add_f64s(num1: f64, num2: f64) -> f64 {
    num1 + num2
}

fn main() {
    // 8-bit integers
    let i1 = 8i8;
    let i2 = 10u8;
    println!("i1 + i2 = {}", add_i8s(i1, i2));

    // 32-bit integers
    let u1: u32 = 200;
    let u2: f64 = 300;
    println!("u1 + u2 = {}", add_u32s(u1, u2));

    // floats
    let f1 = 3.14159f32;
    let f2 = 1.23456f64;
    println!("f1 + f2 = {}", add_f64s(f1, f2));

    // tuples
    let my_tuple: (i32, i32, f64) = (10, 20, 30.5);
    println!("my_tuple = {:?}", my_tuple);
    let (first, second) = my_tuple;
    println!("first = {}", first);
    println!("second = {}", second);

    // arrays
    let array_of_floats: [f32; 5] = [1, 2, 3, 4, 5];
    println!("The length of array_of_floats is {}", array_of_floats.len());

    // variables and mutability
    let score: f32 = 85.4;
    println!("Score before extra credit: {}", score);
    score = score + 5.0;
    println!("Score after extra credit: {}", score);

    // area of a circle
    const PI = 3.14159;
    let radius: f64 = 1.234;
    println!(
        "Area of circle with radius {} is {}",
        radius,
        radius * radius * PI
    );
}
