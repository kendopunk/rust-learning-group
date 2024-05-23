#![allow(dead_code, unused)]

use std::{fmt::Debug, process::Output};

#[derive(Clone, Debug)]
struct Point<T> {
    x: T,
    y: T,
}

fn duplicate(a: impl Clone + Debug) -> impl Clone + Debug {
    a.clone()
}

// fn duplicate<T>(a: T) -> T
// where
//     T: Clone + Debug,
// {
//     println!("Duplicating - {:?}", a);
//     a.clone()
// }

// fn my_func<T>(x: T) -> T
// where
//     T: Debug,
// {
//     x
// }
fn my_func(x: impl Debug) -> impl Debug {
    x
}

fn main() {
    let my_point = Point { x: 10.5, y: 44.2 };
    let copied_point = duplicate(my_point);
    println!("{:?}", copied_point);
    assert_eq!(copied_point.x, 10.5);

    println!("{:?}", my_func(1));
    println!("{:?}", my_func("faffy"));
    println!("{:?}", my_func(Point { x: 10, y: 20 }));
}
