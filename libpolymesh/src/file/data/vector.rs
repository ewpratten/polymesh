use serde::{Deserialize, Serialize};
use std::ops::Add;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct PolyVec {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl PolyVec {

    pub fn zero() -> PolyVec {
        PolyVec {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

}

impl Add for PolyVec {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

}