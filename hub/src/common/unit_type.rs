use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Deserialize_repr, Serialize_repr, PartialEq)]
#[repr(u8)]
pub enum Level1UnitType {
    AIR = 1,
    GROUND = 2,
    SEA = 3
}