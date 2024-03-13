mod my_module {
    /**
     * given a string slice (&str) and a minimum length, determine if the
     * word is long enough
     */
    #[allow(dead_code)]
    pub fn word_is_long_enough(s: &str, min_length: usize) -> Result<bool, String> {
        if s.chars().count() >= min_length {
            return Ok(true);
        }
        return Err(format!("'{}' is not long enough.", s));
    }

    #[allow(dead_code)]
    pub struct Cat {
        pub name: String,
        pub breed: String,
        pub legs: u8,
    }

    #[allow(dead_code)]
    pub struct Bird {
        pub name: String,
        pub legs: u8,
    }
    // @TODO
    // (1) derive a compiler-provided printable format for the Cat struct
    // (2) create a pretty-print version for the Bird struct
    //      "My bird is named _ and it has _ legs."
    // (3) Allow both structs to be cloned and compared for equality
    // (4) Create a Default implementation for both Cat and Bird, w/ resonable default values
    // (5) Create a trait called Speak that requires a function speak() -> void
    // (6) Implement Speak for Cat -> "meow"
    // (7) Implement Speak for Bird -> "chirp"
}

// use my_module::Speak

fn main() {
    println!("Hello, Session 4");

    /*
    UNCOMMENT HERE...

    // (1) Result<T,E>
    assert_eq!(
        my_module::word_is_long_enough("hello", 4),
        Ok("yes".to_string())
    );
    assert_eq!(
        my_module::word_is_long_enough("hello", 20),
        Err("'hello' is not long enough.".to_string())
    );

    // (2) Result<T, E> - match
    let long_enough = my_module::word_is_long_enough("hello world", 25);
    match long_enough {
        Ok(value) => println!("The word is long enough: {}", value),
        Err() => println!("{}", msg),
    }

    // (3) Result<T, E> - unwrap_or()
    let result = my_module::word_is_long_enough("hello world", 10).unwrap_or(false);
    assert_eq!(result, false);

    // (4) Structs - instantiation and printing
    let cat = my_module::Cat {
        name: "Fluffy".to_string(),
        breed: "Russian Blue".to_string(),
        legs: 4,
    };
    let bird = my_module::Bird {
        name: "Tweety".to_string(),
        legs: 2,
    };

    println!("My cat's name is {}", cat.name);
    println!("My bird's name is {}", bird.name);
    println!("The entire cat object is {:?}", cat);
    println!("The pretty-printed bird object is {:?}", bird);

    // (5) Structs - cloning
    let mut your_cat = cat.clone();
    your_cat.name = "Tiger".to_string();
    assert_eq!(your_cat.name, "Tiger".to_string());

    // (6) Structs - equality
    assert_ne!(your_cat, cat);

    // (7) Structs - default values
    let default_cat = my_module::Cat::default();
    let default_bird = my_module::Bird::default();
    println!("Default cat: {:?}", default_cat);
    println!("Default bird: {:?}", default_bird);

    // (8) Traits - implementation
    default_cat.speak();
    default_bird.speak();

    ...TO HERE
    */
}
