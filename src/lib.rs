use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub word_list: Vec<String>,
    pub urls: Vec<String>
}

pub struct Item {
    name: String,
    producer: String,
    quantity: i16,
    mass: String,
    max_per_household: i16,
}

impl Item {
    fn new() -> Self {
        Item {
            name: "".to_string(),
            producer: "".to_string(),
            quantity: 0,
            mass:
        }
    }
}
