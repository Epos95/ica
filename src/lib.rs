use serde::Serialize;
use serde::Deserialize;

use std::fmt;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub word_list: Vec<String>,
    pub urls: Vec<String>
}

#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub producer: String,
    pub mass: String,
    pub more_info: String,
    pub stammispris: bool,
    pub frozen: bool,
    pub deal: String
}

impl Item {
    pub fn new(
        name: String,
        producer: String,
        mass: String,
        more_info: String,
        thing: (bool, bool, String)
    ) -> Self {
        Item {
            name: name,
            producer: producer,
            mass: mass,
            more_info: more_info,
            stammispris: thing.0,
            frozen: thing.1,
            deal: thing.2
        }
    }
}

impl fmt::Display for Item {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.deal)?;
        Ok(())
    }
}
