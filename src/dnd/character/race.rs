use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Race {
    base_race: BaseRace,
    variant: String,
    subrace: String
}

#[derive(Serialize,Deserialize, Debug)]
pub enum BaseRace {
	Dragonborn,
	Dwarf,
	Elf,
	Gnome,
	HalfElf,
	Halfling,
	HalfOrc,
	Human,
	Tiefling,
	Aarakocra,
	Genasi,
	Goliath,
	Aasimar,
	Bugbear,
	Firbolg,
	Goblin,
	Kenku,
	Kobold,
	Lizardfolk,
	Orc,
	Tabaxi,
	Triton,
	YuantiPureblood,
	Tortle,
	Gith,
	Changeling,
	Kalashtar,
	Shifter,
	Warforged,
	Centaur,
	Loxodon,
	Minotaur,
	SimicHybrid,
	Vedalken,
	Verdan,
	Dampear,
    Drow,
    Other(String),
	Unknown
}

impl BaseRace {

}