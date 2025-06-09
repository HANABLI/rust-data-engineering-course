use rand::prelude::IndexedRandom;
use rand::rng;
use std::collections::HashSet;

fn generate_fruits() -> &'static str {
    let fruits = [
        "Apple",
        "Pear",
        "Cherry",
        "Fig",
        "Date",
        "Grape",
        "Strawberry",
        "Banana",
    ];
    let mut rng = rng();
    fruits.choose(&mut rng).unwrap()
}

fn main() {
    let mut fruit_set = HashSet::new();
    println!("Generating 100 fruits.... ");
    for _ in 0..100 {
        fruit_set.insert(generate_fruits());
    }
    println!("Number of unique fruits generated: {}", fruit_set.len());
}
