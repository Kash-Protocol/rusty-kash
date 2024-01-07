use serde::{Deserialize, Serialize};
use std::ops::{Add, AddAssign, Sub, SubAssign};

/// Represents the circulating supply for each asset type.
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct AssetCirculatingSupply {
    pub ksh_supply: u64,
    pub kusd_supply: u64,
    pub krv_supply: u64,
}

impl PartialEq for AssetCirculatingSupply {
    fn eq(&self, other: &Self) -> bool {
        self.ksh_supply == other.ksh_supply && self.kusd_supply == other.kusd_supply && self.krv_supply == other.krv_supply
    }
}

impl AddAssign<AssetCirculatingSupplyDiffs> for AssetCirculatingSupply {
    fn add_assign(&mut self, other: AssetCirculatingSupplyDiffs) {
        // Adjusting ksh_supply considering positive and negative differences
        self.ksh_supply = if other.ksh_supply_diff.is_negative() {
            // If the difference is negative, subtract its absolute value
            self.ksh_supply.checked_sub(other.ksh_supply_diff.wrapping_abs() as u64).expect("Underflow in ksh_supply")
        } else {
            // If the difference is positive, add it directly
            self.ksh_supply.checked_add(other.ksh_supply_diff as u64).expect("Overflow in ksh_supply")
        };

        // Adjusting kusd_supply with the same logic as ksh_supply
        self.kusd_supply = if other.kusd_supply_diff.is_negative() {
            self.kusd_supply.checked_sub(other.kusd_supply_diff.wrapping_abs() as u64).expect("Underflow in kusd_supply")
        } else {
            self.kusd_supply.checked_add(other.kusd_supply_diff as u64).expect("Overflow in kusd_supply")
        };

        // Adjusting krv_supply with the same logic as ksh_supply
        self.krv_supply = if other.krv_supply_diff.is_negative() {
            self.krv_supply.checked_sub(other.krv_supply_diff.wrapping_abs() as u64).expect("Underflow in krv_supply")
        } else {
            self.krv_supply.checked_add(other.krv_supply_diff as u64).expect("Overflow in krv_supply")
        };
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct AssetCirculatingSupplyDiffs {
    pub ksh_supply_diff: i64,
    pub kusd_supply_diff: i64,
    pub krv_supply_diff: i64,
}

impl Add for AssetCirculatingSupplyDiffs {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            ksh_supply_diff: self.ksh_supply_diff + other.ksh_supply_diff,
            kusd_supply_diff: self.kusd_supply_diff + other.kusd_supply_diff,
            krv_supply_diff: self.krv_supply_diff + other.krv_supply_diff,
        }
    }
}

impl Sub for AssetCirculatingSupplyDiffs {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            ksh_supply_diff: self.ksh_supply_diff - other.ksh_supply_diff,
            kusd_supply_diff: self.kusd_supply_diff - other.kusd_supply_diff,
            krv_supply_diff: self.krv_supply_diff - other.krv_supply_diff,
        }
    }
}

impl AddAssign for AssetCirculatingSupplyDiffs {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl SubAssign for AssetCirculatingSupplyDiffs {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl AssetCirculatingSupplyDiffs {
    /// Returns true if the circulating supply difference is zero.
    pub fn is_unchanged(&self) -> bool {
        self.ksh_supply_diff == 0 && self.kusd_supply_diff == 0 && self.krv_supply_diff == 0
    }
}

/// Type for circulating supply
pub type CirculatingSupply = u64;
