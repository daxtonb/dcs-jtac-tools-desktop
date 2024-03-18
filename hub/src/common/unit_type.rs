use serde_repr::{Deserialize_repr, Serialize_repr};

/// Level-1 unit types as represented by DCS World
/// TODO: look into Level-2
#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum Level1UnitType {
    AIR = 1,
    GROUND = 2,
    SEA = 3
}