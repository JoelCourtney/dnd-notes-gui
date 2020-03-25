use serde::{Serialize, Deserialize};

mod race;

use race::Race;

#[derive(Serialize, Deserialize, Debug)]
pub struct Character {
    played_by: String,
    name: String,
    race: Race,
    // class: Option<Class>,
    // level: Option<u8>,
    // alignment: Option<Alignment>,
    // inventory: Vec<Item>,
    // traits: Vec<String>,
    // notes: Vec<String>,
    relationship: String
}

impl Character {
}