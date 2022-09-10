// use serde::{Deserialize, Serialize};
// use serde_json::Result;

// #[derive(Serialize, Deserialize)]
// struct Address {
//     street: String,
//     city: String,
// }

// fn print_an_address() -> Result<()> {
//     let address = Address {
//         street: "10 Downing Street".to_owned(),
//         city: "London".to_owned(),
//     };
//     let j = serde_json::to_string(&address)?;
//     println!("{}", j);
//     Ok(())
// }

// fn main() {
//     print_an_address().unwrap();
// }

use rust_api_demo::{print_an_address, Point};

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    let ddd = print_an_address();

    println!("ssss = {:?}", ddd.unwrap());
}
