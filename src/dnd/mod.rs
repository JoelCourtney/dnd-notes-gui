mod character;

use serde::{Serialize, Deserialize};
use character::Character;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Campaign {
    name: String,
    dm: String,
    chars: Vec<Character>
}

impl Campaign {
    pub fn read(path: &str) -> Self {
        let json = fs::read_to_string(path).expect("Unable to read file");
        serde_json::from_str(&json).expect(&format!("Unable to deserialize json:\n{}",json))
    }

    pub fn write(&self, path: &str) {
        let json = serde_json::to_string_pretty(&self).expect(&format!("Unable to serialize object:\n{:?}",&self));
        println!("{}",json);
        fs::write(path, json).expect("Unable to write to file");
    }
}