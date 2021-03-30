use serde::{Deserialize, Serialize};
use std::ops::Add;
use derivative::Derivative;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Derivative, PartialOrd)]
#[derivative(Hash)]
pub struct PolyVec {
    #[derivative(Hash="ignore")]
    pub x: f32,
    #[derivative(Hash="ignore")]
    pub y: f32,
    #[derivative(Hash="ignore")]
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