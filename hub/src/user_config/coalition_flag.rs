use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

use serde::{Serialize, Deserialize};

/// Represents the 3 DCS coalitions.
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct CoalitionFlag(pub u8);

impl CoalitionFlag {
    pub const NEUTRAL: CoalitionFlag = CoalitionFlag(1);
    pub const REDFOR: CoalitionFlag = CoalitionFlag(2);
    pub const BLUFOR: CoalitionFlag = CoalitionFlag(4);
    
    /// Returns `CoalitionFlag(0)`.
    pub fn empty() -> CoalitionFlag {
        CoalitionFlag(0)
    }
}

impl BitOr for CoalitionFlag {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        CoalitionFlag(self.0 | rhs.0)
    }
}

impl BitOrAssign for CoalitionFlag {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitAnd for CoalitionFlag {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        CoalitionFlag(self.0 & rhs.0)
    }
}

impl BitAndAssign for CoalitionFlag {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}
