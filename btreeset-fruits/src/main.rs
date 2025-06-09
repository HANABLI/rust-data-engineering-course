use rand::prelude::SliceRandom;
use rand::rng;
use std::collections::BTreeSet;


fn main() {
    let fruits = vec![
        "Apple",
        "Pear",
        "Cherry",
        "Strawberry",
        "Banana",
        "Fig",
        "Ananas",
        "Grape",
        "elderberry",
        "honeydraw",
    ];

    let amounts = [1, 3, 5, 7, 9];

    let mut rng = rng();

    for amount in amounts.iter() {
        let mut fruit_set = BTreeSet::new();
        let mut shuffled_fruits = fruits.clone();
        shuffled_fruits.shuffle(&mut rng);

        for fruit in shuffled_fruits {
            fruit_set.insert(fruit);
            if fruit_set.len() >= *amount {
                break;
            }
        }

        println!("{}: {:?}", amount, fruit_set);
    }
}
