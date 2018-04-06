use std::collections::{HashMap, HashSet};
extern crate uuid;
use uuid::Uuid;

use types::*;

#[derive(Clone)]
pub enum MetadataVariant {
    Bool(bool), Str(String), Int(i32)
}

// All structs that get used in the game MUST implement this in order to function
pub trait RPGType {
    fn get_metadata(&self) -> &HashMap<String, MetadataVariant>;
    fn set_metadata(&mut self, String, MetadataVariant);

    fn get_module(&self) -> String;
    fn get_uuid_name(&self) -> String;

    fn make_clone(&self) -> Box<RPGType>;

}

pub trait Character {
    // Who are you?
    fn get_name(&self) -> String;
    fn set_name(&mut self, String);

    // Stats not determined per battle. Changed by leveling up.
    fn get_personal_stats(&self) -> PersonalStats;
    fn set_personal_stats(&mut self, PersonalStats);

    // Armor/health/qi/etc. Changed by changing personal stats.
    fn get_battle_stats(&self) -> BattleStats;
    fn set_battle_stats(&mut self, BattleStats);

    // For factions, as well as how much they like certain people.
    fn get_affiliation(&self, String) -> u8;
    fn set_affiliation(&mut self, String, u8); // Set affil for faction(String) to (u8).

    // Cover custom stats/attributes
    fn get_attribute(&self, String) -> MetadataVariant;
    fn set_attribute(&mut self, String, MetadataVariant);
}



pub trait Usable {
    fn use_it(&mut self, &mut Character);
}

pub trait Equippable {
    fn equip(&mut self, &mut Character);
}

pub trait Consumable {
    // Returns whether item has been used up or is otherwise removed from inventory.
    fn consume(&mut self, &mut Character) -> bool;
}

pub trait Storable {
    fn store_in(&mut self, &mut Storage);
    fn remove_from(&mut self, &mut Storage);

    fn get_size(&self) ->u8;
    fn set_size(&mut self, u8);
}

pub trait Storage {
    fn store(&mut self, &mut Storable);
    fn remove(&mut self, String) -> Box<Storable>;

    fn get_stored(&self) -> HashSet<String>;
}