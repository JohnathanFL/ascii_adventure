#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(non_snake_case)]

extern crate uuid;
extern crate chrono;

pub mod traits;
pub mod types;
pub mod default_objects;

use std::ops::*;
use std::collections::{HashMap};
use std::rc::{Rc};
use std::fmt;
use std::fmt::{Debug, Formatter};
use std::error::{Error};
use std::result::{*};
use std::cell::*;

use chrono::Local;
use uuid::Uuid;

pub type UUID = uuid::Uuid;

pub trait RPGType {}

struct CurMax<T> {
    cur: T,
    max: T,
}

pub type MetadataTable = HashMap<String, MetadataVariant>;

#[derive(Clone)]
pub enum RPGVariantFunction {
    Ternary(Rc<Fn(&mut RPGType, &mut RPGType, &mut RPGType, Option<MetadataTable>)>),
    Binary(Rc<Fn(&mut RPGType, &mut RPGType, Option<MetadataTable>)>),
    Unary(Rc<Fn(&mut RPGType, Option<MetadataTable>)>),
    Void(Rc<Fn()>),
}

impl Debug for RPGVariantFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let fnType = match self {
            &RPGVariantFunction::Ternary(_) => "Ternary",
            &RPGVariantFunction::Binary(_) => "Binary",
            &RPGVariantFunction::Unary(_) => "Unary",
            &RPGVariantFunction::Void(_) => "Void",
        };
        write!(f, "RPGFunction{{{}}}", fnType)
    }
}

#[derive(Debug, Clone)]
pub enum MetadataVariant {
    Number(i32),
    Str(String),
    Bool(bool),
    Function(RPGVariantFunction),
}

pub mod metadata_tags {
    static STORABLE_META: &'static str = "IsStorable";
    static EQUIPPABLE_META: &'static str = "IsEquippable";
    static CONSUMABLE_META: &'static str = "IsConsumable";
    static WEAPON_META: &'static str = "IsWeapon";
    static CHARACTER_META: &'static str = "IsCharacter";
}

// Using i16 instead of u16 to allow for buffing and debuffing stats.
pub type StatType = i16;

#[derive(Debug, Clone)]
pub struct BattleStats {
    pub health: StatType, // How many cuts are on you?
    pub armor: StatType, // How much force does it take to cut you?
    pub qi: StatType, // How much magic can you pump out? (Doubles as mana).
    pub spirit: StatType, // How strong is your soul? (Mental fortitude, among others.)
    pub stamina: StatType, // How far can you run?
    pub metadata: Option<MetadataTable>, // Custom stuff
}

impl BattleStats {
    pub fn new() -> BattleStats {
        BattleStats {
            health: 0,
            armor: 0,
            qi: 0,
            spirit: 0,
            stamina: 0,
            metadata: None,
        }
    }
}



#[derive(Debug, Clone)]
pub struct PersonalStats {
    pub strength: StatType, // How hard can you hit?
    pub perception: StatType, // How sharp are your senses?
    pub endurance: StatType, // How much of a beating can your body take?
    pub charisma: StatType, // How easily can you make people like you?

    pub intellect: StatType, // How well can you do math?
    pub wisdom: StatType, // How well do you know you should be studying for the test tomorrow?
    pub agility: StatType, // How quickly can you dodge/run?

    pub soul: StatType, // Both for soul based stuff in XianXia sense and for willpower.

    pub metadata: Option<MetadataTable>, // Can be used for stuff like cultivation levels and other customs.
}

#[derive(Debug, Clone)]
pub struct PhysicalDamage {
    // u16 as it only makes sense to remove health on physical.
    pub blunt: u16,
    pub pierce: u16,
    pub slash: u16,
}

#[derive(Debug, Clone)]
pub enum EffectType {
    Offensive,
    Defensive,
}

#[derive(Debug, Clone)]
pub struct StatusEffect {
    pub baseID: UUID,
    pub effectType: EffectType, // What's the overall intent of this effect?
    pub physical: Option<PhysicalDamage>, // How much physical damage to apply PER TICK.
    pub elemental: Option<MetadataTable>, // How much elemental damage to apply PER TICK.
    pub stats: Option<(PersonalStats, BattleStats)>, // How much stats are changed FOR DURATION
    pub length: u32, // How long should this last for? Ignored if this is in the damageQueue

    // inputs: Receiving char, this effect
    pub tickFunc: Option<RPGVariantFunction>, // Custom tick handling. If None, normal logic applies.
    pub metadata: Option<MetadataTable>,
    pub giver: Option<UUID>, // Who gave this to us?
}

#[derive(Debug, Clone)]
pub enum WeaponType {
    Melee,
    Ranged,
    MagicMelee,
    MagicRanged,
}

#[derive(Debug, Clone)]
pub struct Weapon {
    pub baseID: Uuid,
    pub id: Uuid, // Index into a table of all items. Should be HASH(MOD_NAME + ':' + BASE_ITEM_NAME)
    pub name: String, // Unless customized, is the same as BASE_ITEM_NAME
    pub weaponType: WeaponType, // For determining range and dodge chances.
    pub damage: StatusEffect, // Gets applied to target
}
impl Weapon {
    pub fn new(module: String, name: String, weaponType: WeaponType, damage: StatusEffect) -> Weapon {
        let uuidName = format!("{}:{}", module, name);
        let uniqueUUIDName = format!("{}:{}:{}", module, name, Local::now());
        let baseID = UUID::new_v5(&uuid::NAMESPACE_OID, &uuidName);
        let id = UUID::new_v5(&uuid::NAMESPACE_OID,&uniqueUUIDName);

        return Weapon {
            baseID, id, name, weaponType, damage
        };
    }

    // TODO: pub fn deserialize(from: String) -> Weapon
}

#[derive(Debug, Clone)]
pub struct Character {
    pub baseID: Uuid,
    pub id: Uuid,
    pub name: String,
    pub personal: PersonalStats,
    pub battle: BattleStats,

    // Note: Both CAN act the same, using custom tick funcs, but damageQueue removes after calling.
    pub damageQueue: Vec<StatusEffect>, // For things applied once and then auto removed
    pub statusEffects: Vec<StatusEffect>, // Things that have a duration.

    pub customDamageHandler: Option<RPGVariantFunction>, // Custom handler for non custom status effects
    pub relations: HashMap<Uuid, i16>, // Relation with the party mentioned in here.
    pub metadata: Option<MetadataTable>,
}

impl Character {
    fn doTick(&mut self) {
        
    }
}

impl RPGType for Character{}