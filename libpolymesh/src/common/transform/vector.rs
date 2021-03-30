use serde::{Deserialize, Serialize};
use std::ops::Add;

/// Simple, JSON-Serializable vector
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct PolyVector {
    pub x: f32,
    pub y: f32,
    pub z: f32
}

impl PolyVector {

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn unit() -> Self {
        Self {
            x: 1.0,
            y: 1.0,
            z: 1.0
        }
    }

}

impl Add for PolyVector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

}