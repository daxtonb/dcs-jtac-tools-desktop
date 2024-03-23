use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use serde::{Deserialize, Serialize};

/// Represents the three high-level DCS unit classifications.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct UnitTypeFlag(pub u8);

impl UnitTypeFlag {
    pub const GROUND: UnitTypeFlag = UnitTypeFlag(1);
    pub const AIR: UnitTypeFlag = UnitTypeFlag(2);
    pub const SEA: UnitTypeFlag = UnitTypeFlag(4);
    
    /// Returns `UnitTypeFlag(0)`.
    pub fn empty() -> UnitTypeFlag {
        UnitTypeFlag(0)
    }
}

impl BitOr for UnitTypeFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        UnitTypeFlag(self.0 | rhs.0)
    }
}

impl BitOrAssign for UnitTypeFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for UnitTypeFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        UnitTypeFlag(self.0 & rhs.0)
    }
}

impl BitAndAssign for UnitTypeFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}