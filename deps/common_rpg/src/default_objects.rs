/* Provides easy default implementations of the various traits so addons aren't
*  required to implement everything themselves.
*
*  THESE SHOULD NOT BE REFERENCED OUTSIDE OF WHERE THEY ARE USED DIRECTLY, AS
*  THERE IS NO GUARENTEE THAT THIS IS WHAT IS USED IN EACH MODULE.
*/

use std::collections::*;
use std::ops::Index;

extern crate uuid;

use traits::*;
use types::*;

struct DefaultCharacter {
    name: String,
    personStats: PersonalStats,
    battleStats: BattleStats,
    affiliations: HashMap<String, u8>,
    metadata: HashMap<String, MetadataVariant>,
}