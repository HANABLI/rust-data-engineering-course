/*
This code starts with an initial VecDeque,
converts it to a Vec for Shuffling, and then converts it back to a VecDeque.
After that, it pushes "Pomegranate" to the front of the deque, and "Fig" and "cherry"
to the back of the deque.
Finally, it prints out the final fruit salad.

A VecDeque is a double-ended queue, which means that you can push and pop elements from both ends
of the queue.
*/
use rand::seq::SliceRandom;
use rand::rng;
use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();

    fruit.push_back("Arbutus");
    fruit.push_back("Loquat");
    fruit.push_back("Strawberry");

    //Sramble (shuffle) the fruit
    let mut rng = rng();
    let mut fruit: Vec<_> = fruit.into_iter().collect();
    fruit.shuffle(&mut rng);

    // Convert it back to VecDeque
    let mut fruit: VecDeque<_> = fruit.into_iter().collect();

    // Add fruits to the both ends of the queue after shuffling
    fruit.push_back("Pomegranate");
    fruit.push_back("Fig");
    fruit.push_front("cherry");

    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}