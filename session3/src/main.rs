mod test;

fn unused_function(v: Vec<i32>) {
    println!("Pretty print vector: {:#?}", v);
}

fn main() {
    println!("Hello, Session 3");

    /*
    UNCOMMENT HERE...
    let unused_var = 10u32;

    ////////////////////////////////////////////////////////////
    // VECTORS
    // https://doc.rust-lang.org/std/vec/struct.Vec.html
    ////////////////////////////////////////////////////////////

    // adding to a vector
    let v1: Vec<i32> = Vec::new();
    v1.push(1);
    println!("v1 = {:?}", v1);

    // getting a vector's length
    let v2: Vec<f32> = vec![2.2, 3.3, 4.4, 5.5];
    assert_eq!(/* YOUR CODE HERE */, 4);

    // clearing out a vector
    let v3 = vec![1, 2, 3, 4, 5];
    /* YOUR CODE HERE */
    assert_eq!(v3.is_empty(), true);

    // append from one vector into another
    let mut first_vector = vec![1, 2, 3, 4, 5];
    let mut second_vector = vec![6, 7, 8, 9, 10];
    /* YOUR CODE HERE */
    assert_eq!(first_vector, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    assert_eq!(second_vector.is_empty(), true);

    // dividing up a vector
    let mut first_vector = vec![1, 2, 3, 4, 5];
    let second_vector = /* YOUR CODE HERE */
    assert_eq!(first_vector, vec![1]);
    assert_eq!(second_vector, vec![2, 3, 4, 5]);

    // using match with Vector get(index)
    let characters: Vec<char> = vec!['A', 'B', 'C', 'D', 'E'];
    let looking_for_letter_D = characters.get(3);
    match looking_for_letter_D {
        'D' => println!("I found the letter D!"),
        _ => println!("I did not find the letter D"),
    }

    ////////////////////////////////////////////////////////////
    // STRINGS
    // https://doc.rust-lang.org/std/vec/struct.Vec.html
    ////////////////////////////////////////////////////////////

    // mutating a String
    let mut s1 = String::from("Hello ");
    /* YOUR CODE TO add "world!" to s1 HERE */
    assert_eq!(s1, "Hello world!");

    // split a String into two separate strings
    let mut group = String::from("Rust Learning Group");
    let second_part = /* YOUR "split off" (hint) CODE HERE */
    assert_eq!(group, "Rust ");
    assert_eq!(second_part, "Learning Group");

    ////////////////////////////////////////////////////////////
    // OWNERSHIP AND BORROWING
    ////////////////////////////////////////////////////////////

    // follow the compiler's advice to get this to work
    fn add_borrowing(s: &mut String) {
        s.push_str("and Borrowing");
    }
    let mut s1 = String::from("Ownership ");
    add_borrowing(s1);
    assert_eq!(s1, "Ownership and Borrowing");

    // add the reference operator (&) in two places in this code block
    // to get it to work
    {
        fn print_vector(v: Vec<i32>) {
            println!("Inside print_vector function {:?}", v);
        }

        let v = vec![10, 20, 30];
        print_vector(v);  // hint: print_vector() is taking ownership of v here
        println!("{}", v[0]); // this will error until you fix it
    }

    ...TO HERE
    */
}
