/****************************************
 * cargo run --bin session9_answers
 ****************************************/
use std::collections::HashMap;
use std::fmt::Display;
use std::hash::Hash;

fn main() {
    // ////////////////////////////////////////////////
    // @TODO 1
    // Write a generic function called "find_min" which returns
    // the minimum from two inputs "a" and "b" both of type T
    // ////////////////////////////////////////////////
    fn find_min<T: PartialOrd>(a: T, b: T) -> T {
        if a < b {
            return a;
        } else {
            return b;
        }
    }
    assert_eq!(find_min(10, 20), 10);
    assert_eq!(find_min(3.14159, 0.0001), 0.0001);

    // ////////////////////////////////////////////////
    // @TODO 2
    // Write a generic function called "to_tuple" which
    // takes three typed values T, U, V
    // and returns a tuple of the three arguments
    // ////////////////////////////////////////////////
    fn make_tuple<T, U, V>(a: T, b: U, c: V) -> (T, U, V) {
        (a, b, c)
    }
    assert_eq!(
        make_tuple("hello", 1.5, [3, 6, 9]),
        ("hello", 1.5, [3, 6, 9])
    );

    // ////////////////////////////////////////////////
    // @TODO 3
    // Write a generic function called "make_hash_map" which
    // takes two arguments: a:T and b: U
    // where a is the key and b is the value
    // and return a HashMap
    // ////////////////////////////////////////////////
    fn make_hash_map<T, U>(a: T, b: U) -> HashMap<T, U>
    where
        T: PartialEq + Eq + Hash,
    {
        let mut hm: HashMap<T, U> = HashMap::new();
        hm.insert(a, b);

        hm
    }
    assert_eq!(make_hash_map(1, "hello").get(&1), Some(&"hello"));
    assert_eq!(
        make_hash_map("array", [1, 2, 3]).get(&"array"),
        Some(&[1, 2, 3])
    );
    assert_eq!(
        make_hash_map([1, 2, 3], "array").get(&[1, 2, 3]),
        Some(&"array")
    );

    // ////////////////////////////////////////////////
    // @TODO 4
    // Write a generic function called "concat" which
    // takes two Display-able arguments of type T
    // and returns a concatenated string value
    // ////////////////////////////////////////////////
    fn concat<T: Display>(a: T, b: T) -> String {
        format!("{}{}", a, b)
    }
    assert_eq!(concat(1, 10), String::from("110"));
    assert_eq!(concat(100.1, 38.7), String::from("100.138.7"));
    assert_eq!(concat("hello", "world"), String::from("helloworld"));

    // ////////////////////////////////////////////////
    // @TODO 5
    // Write a generic function called "longer_slice" which
    // accepts references to two slice-type data structures
    // (array, vector, etc.)
    // and returns a reference to the slice that is longer
    // Follow the compiler guidance on lifetimes
    // ////////////////////////////////////////////////
    fn longer_slice<'a, T>(a: &'a [T], b: &'a [T]) -> &'a [T] {
        if a.len() > b.len() || a.len() == b.len() {
            return a;
        } else {
            return b;
        }
    }

    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [6, 7, 8];
    assert_eq!(longer_slice(&arr1, &arr2), &arr1);

    let v1 = vec!['a', 'b', 'c'];
    let v2 = vec!['d', 'e', 'f', 'g', 'h'];
    assert_eq!(longer_slice(&v1, &v2), &v2);

    let s1 = "Learning".as_bytes();
    let s2 = "rust".as_bytes();
    assert_eq!(longer_slice(s1, s2), s1);
}
