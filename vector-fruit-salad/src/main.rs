use rand::seq::SliceRandom;
// use rng function rather than thread_rng, thread_rng is deprecated
use rand::rng;

fn main() {
    let mut fruit = vec![
        "Orange",
        "Fig",
        "Pomegranate",
        "Cherry",
        "Apple",
        "Pear",
        "Peach",
    ];

    // Scramble (shuffle) the fruit
    let mut rng = rng();
    fruit.shuffle(&mut rng);

    // Print out fruit salad
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            print!("{}", item);
        }
    }
}
