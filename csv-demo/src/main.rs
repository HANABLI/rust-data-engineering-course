use csv::{Writer, Reader};
use std::collections::HashMap;
use std::str::FromStr;
use std::error::Error;

#[derive(Debug, serde::Deserialize)]
struct Record {
    fruit: String,
    price: String,
}

fn main() -> Result<(), Box<dyn Error>> {

    let fruits = [
        ("Apple", 1.25),
        ("Banana", 0.75),
        ("Orange", 1.00),
        ("Mango", 2.50),
        ("PineApple", 3.00),
        ("Pear", 2.55),
        ("Cherry", 6.00),
    ];

    let mut writer = Writer::from_path("output.csv")?;

    writer.write_record(["fruit", "price"])?;

    for (fruit, price) in &fruits {
        writer.write_record([*fruit, &price.to_string()])?;
    }

    writer.flush()?;


    let reader = Reader::from_path("output.csv");
    let mut discount = HashMap::new();
    for result in reader?.deserialize() {
        let record: Record = result?;
        println!("{}, {}", record.fruit, record.price);
        discount.insert(record.fruit, f32::from_str(&record.price).unwrap());
    }

    let mut discount_writer = Writer::from_path("discount.csv")?;
    discount_writer.write_record(["fruit", "price"])?;
    for (fruit, price) in discount {
        let discount_price: f32 = (price * 90.0) / 100.0;
        discount_writer.write_record([fruit, discount_price.to_string()])?;
    }
    
    discount_writer.flush()?;
    Ok(())
}
