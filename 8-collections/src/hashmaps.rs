use core::panic;
use std::collections::HashMap;

pub fn hashmaps() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // creating a hashmap from 2 vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    // The type annotation HashMap<_, _> is needed here
    // because it’s possible to collect into many different data structures (collections)
    // and Rust doesn’t know which you want unless you specify.
    // For the parameters for the key and value types, however, we use underscores,
    // and Rust can infer the types that the hash map contains based on the types of the data in the vectors.
    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // For types that implement the Copy trait, like i32, the values are copied into the hash map.
    // For owned values like String, the values will be moved and the hash map will be the owner of those values.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // can't access field_name and field_value after here
    // println!("key = {}, value = {}", field_name, field_value);

    // accessing values in a hashmap
    let key = String::from("Favorite color");
    let val = map.get(&key);
    let val = match val {
        Some(val) => val,
        None => panic!("No value!"),
    };
    println!("Key: {}, Val: {}", key, val);

    // iterating over a map
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}
