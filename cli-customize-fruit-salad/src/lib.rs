/*
    This code defines a function called create_fruite_salad
    that takes a mutable vector of Strings as input and returns
    and return a new vector of strings that contains the same elements
    as the input vector, but in a random order.
*/
use rand::seq::SliceRandom;
use rand::rng;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = rng();
    fruits.shuffle(&mut rng);

    fruits
}