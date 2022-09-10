use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
}

pub fn print_an_address() -> Result<()> {
    let address = Address {
        street: "10 Downing Street".to_owned(),
        city: "London".to_owned(),
    };
    let j = serde_json::to_string(&address)?;
    // println!("{}", j);
    Ok(j)
}
