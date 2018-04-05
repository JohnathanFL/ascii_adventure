/*
*   rpgtypes.rs
*   
*   Defines basic types and operations for the entire game. For actual game
*   logic, look elsewhere.

*   Johnathan Lee
*/

use std::collections::HashMap;


use traits::*;


pub type StatType = u8;

#[derive(Debug, Copy, Clone)]
pub struct BattleStats {
    pub health: StatType,
    pub armor: StatType,
    pub qi: StatType,
    pub spirit: StatType,
    pub stamina: StatType,
}

impl BattleStats {
    pub fn new() -> BattleStats {
        BattleStats {
            health: 0,
            armor: 0,
            qi: 0,
            spirit: 0,
            stamina: 0,
        }
    }
}


#[derive(Debug, Copy, Clone)]
pub struct PersonalStats {
    pub strength: StatType,    // How hard can you hit?
    pub perception: StatType,  // How sharp are your senses?
    pub endurance: StatType,   // How much of a beating can your body take?
    pub charisma: StatType,    // How easily can you make people like you?

    pub intellect: StatType,   // How well can you do math?
    pub wisdom: StatType,      // How well do you know you should be studying for the test tomorrow?
    pub agility: StatType,     // How quickly can you dodge/run?

    pub soul: StatType,        // 
}

impl PersonalStats {
    pub fn new() -> PersonalStats {
        PersonalStats {
            // 10 is the value for a normal human.
            strength: 10,
            perception: 10,
            endurance: 10,
            charisma: 10,
            intellect: 10,
            agility: 10,
            soul: 10,
            wisdom: 10,
        }
    }
    pub fn new_with(
        strength: StatType,
        perception: StatType,
        endurance: StatType,
        charisma: StatType,
        intellect: StatType,
        agility: StatType,
        soul: StatType,
        wisdom: StatType,
    ) -> PersonalStats {
        return PersonalStats {
            strength,
            perception,
            endurance,
            charisma,
            intellect,
            agility,
            soul,
            wisdom
        };
    }
}