use serde::{Serialize, Deserialize};

/// Represents the 3 DCS coalitions.
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum CoalitionFlag {
    NEUTRAL = 1,
    REDFOR = 2,
    BLUFOR = 4,
}
