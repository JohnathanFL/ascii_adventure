#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate text_io;

use std::rc::Rc;
use std::collections::HashMap;

extern crate common_rpg;
use common_rpg::traits::*;
use common_rpg::types::*;


extern crate uuid;
use uuid::Uuid;

struct RPGTypeRegistry {
    types: HashMap<Uuid, Box<RPGType>>,
}

impl RPGTypeRegistry {
    fn register(&mut self, rpgType: &RPGType) {
        self.types.insert(Uuid::new_v5(&uuid::NAMESPACE_OID, &rpgType.get_uuid_name()[0..]), rpgType.make_clone());
    }
}


fn main() {
    
}
