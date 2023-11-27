#[allow(dead_code)]
fn multiply_two_numbers(a: i32, b: i32) -> i32 {
    a * b
}

#[allow(dead_code)]
fn square_float(a: f32) -> f32 {
    a * a
}

#[allow(dead_code)]
fn is_odd(num: i32) -> bool {
    num % 2 != 0
}

fn main() {
    println!("Hello, Session 2");

    /*
    UNCOMMENT HERE...

    // FUNCTIONS
    let result = multiply_two_numbers(10i32, 20u32);
    assert_eq!(result, 200);

    // SCOPING
    let a: u8 = 10;
    println!("a = {}", a);
    {
        let b: u8 = 20;
        println!("b = {}", b);
    }
    println!("a is {} and b is {}", a, b);

    // CONTROL FLOW
    let num: i32 = 200;
    if (num <= 100) {
        println!("LTE 100");
    } elif num <= 200 {
        println!("LTE 200");
    } else {
        println!("GTE 200");
    }

    // IF / LET
    let passed = true;
    let score = if passed { 60 } else { "fail" };
    println!("score = {}", score);

    // LOOPS
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if (counter == 25) {
            exit;
        }
    };
    println!("result = {}", result);

    // PRINT THE SQUARE OF ALL VALUES OF AN ARRAY
    let arr: [f32; 5] = [1.1, 2.2, 3.3, 4.4, 5.5];
    foreach element in arr {
        println!("{}", square_float(element));
    }

    // WHAT IS A BETTER WAY TO WRITE THIS MATCH?
    let apples: u8 = 7;
    match apples {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 => println!("You have less than 10 apples."),
        _x if apples >= 10 => println!("You have 10 or more apples."),
        _ => println!("I don't know how many apples you have.")
    };

    // PATTERN MATCHING
    let x = 7.5;
    let number_is_odd = match x {
        _x if is_odd(x) => true,
        _x if !is_odd(x) => false
    };
    println!("Number is odd? {}", number_is_odd);

    // PATTERN MATCHING BINDINGS
    // Using the binding syntax (@), make this code print "At least one passing grade."
    let scores: (f32, f32, f32) = (55.5, 59.9, 60.1);
    match scores {
        YOUR_CODE_HERE => println!("At least one passing grade."),
        _ => println!("Everyone failed.")
    }

    ...TO HERE
    */
}
