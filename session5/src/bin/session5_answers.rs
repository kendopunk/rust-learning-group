/****************************************
 * cargo run --bin session5_answers
 ****************************************/
#![allow(dead_code, unused_variables, unused_imports)]

use std::collections::{HashMap, HashSet, LinkedList};

#[derive(Debug)]
enum Fruit {
    Apple,
    Strawberry,
    Banana,
}

struct Dessert {
    name: String,
    fruit: Fruit,
}

fn main() {
    println!("Hello, Session 5");

    ////////////////////////////////////////
    // (1) enum
    ////////////////////////////////////////
    println!(
        "The variants of Fruit are {:?}, {:?}, and {:?}",
        Fruit::Apple,
        Fruit::Strawberry,
        Fruit::Banana
    );
    let my_fruit = Fruit::Banana;
    assert_eq!(matches!(my_fruit, Fruit::Banana), true);
    println!("");

    let desserts: Vec<_> = vec![
        Dessert {
            name: "apple pie".to_string(),
            fruit: Fruit::Apple,
        },
        Dessert {
            name: "strawberry cake".to_string(),
            fruit: Fruit::Strawberry,
        },
        Dessert {
            name: "banana pudding".to_string(),
            fruit: Fruit::Banana,
        },
    ];
    for dessert in desserts {
        println!("{:?} has the {:?} variant.", dessert.name, dessert.fruit);
    }
    println!("");

    ////////////////////////////////////////
    // (2) HashMap
    ////////////////////////////////////////
    let mut fish = HashMap::new();
    fish.insert("shark", "saltwater");
    fish.insert("bass", "freshwater");
    fish.insert("catfish", "brackish water");

    // @TODO print out the kind of water a catfish lives in
    let water_type = fish.get("catfish").unwrap();
    println!("A catfish lives in {}", water_type);

    // @TODO assert that the fish HashMap contains the string slice "shark"
    assert_eq!(fish.contains_key("shark"), true);

    // @TODO remove the entry with the "bass" key
    fish.remove("bass");
    assert_eq!(fish.len(), 2);

    // @TODO remove all entries in the fish HashMap
    fish.clear();
    assert_eq!(fish.len(), 0);

    ////////////////////////////////////////
    // (3) HashSet
    ////////////////////////////////////////
    let mut rgb_colors = HashSet::new();
    rgb_colors.insert((255, 255, 0));
    rgb_colors.insert((0, 255, 255));
    rgb_colors.insert((99, 0, 204));
    rgb_colors.insert((255, 255, 255));

    // @TODO print out the first element of the HashSet
    println!("The first one is {:?}", rgb_colors.get(&(255, 255, 0)));

    // @TODO See if the set contains the color white (255, 255, 255)
    assert_eq!(rgb_colors.contains(&(255, 255, 255)), true);

    ////////////////////////////////////////
    // (4) LinkedList
    ////////////////////////////////////////
    let mut my_list = LinkedList::new();
    my_list.push_back(1);
    my_list.push_back(2);
    my_list.push_back(3);

    // @TODO add the number 4 to the front of the list
    my_list.push_front(4);
    assert_eq!(my_list, [4, 1, 2, 3].into_iter().collect());

    // @TODO mutably iterate over the list and mutiply each element by 10
    for element in my_list.iter_mut() {
        *element *= 10;
    }
    assert_eq!(my_list, [40, 10, 20, 30].into_iter().collect());
}
