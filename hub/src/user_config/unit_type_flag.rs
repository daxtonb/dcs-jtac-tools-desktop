use serde::{Serialize, Deserialize};

/// Represents the three high-level DCS unit classifications.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum UnitTypeFlag {
    GROUND = 1,
    AIR = 2,
    SEA = 3
}