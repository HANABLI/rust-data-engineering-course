use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "4.5.39",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {

    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested.
    let fruits = opts.number;

    // Create the fruits salad.
    create_fruit_salad(fruits);

    println!("Created Fruit salad with {} fruits: {:?}", fruits, create_fruit_salad(fruits));
}
