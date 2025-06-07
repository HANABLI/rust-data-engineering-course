/*
 */

use rand::seq::SliceRandom;
use rand::rng;

pub fn create_fruit_salad(num_fruits: usize) -> Vec<String> {
    let fruits = vec![
        "Arbutus".to_string(),
        "Poir".to_string(),
        "Apple".to_string(),
        "Strawberry".to_string(),
        "Fig".to_string(),
        "Cherry".to_string(),
        "Melon".to_string(),
        "Orange".to_string(),
        "Banana".to_string(),
        "Pomegranate".to_string(),
    ];

    let mut rng = rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);

    fruits.into_iter().take(num_fruits).collect()
}