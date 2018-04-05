#![allow(non_snake_case)]
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

struct RPGTypeRegistry<'a> {
    types: HashMap<Uuid, &'a RPGType>,
    byModule: HashMap<&'a str, &'a RPGType>
}

impl<'a> RPGTypeRegistry<'a> {
    fn register(&mut self, module: &str, rpgType: &RPGType) {
        self.types.insert(Uuid::new_v5(&uuid::NAMESPACE_OID, rpgType.get_uuid_name()), &rpgType.clone());
    }
}


fn main() {
    
}
