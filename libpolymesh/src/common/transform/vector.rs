extern crate derive_more;

use serde::{Deserialize, Serialize};
use derive_more::{Add, Sub, Mul, Div};

/// Simple, JSON-Serializable vector
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd, Add, Sub, Mul, Div)]
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

    pub fn unit_x() -> Self {
        Self {
            x: 1.0,
            y: 0.0,
            z: 0.0
        }
    }

    pub fn unit_y() -> Self {
        Self {
            x: 0.0,
            y: 1.0,
            z: 0.0
        }
    }

    pub fn unit_z() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 1.0
        }
    }

    pub fn max(a: PolyVector, b: PolyVector) -> PolyVector {
        if a>b {a} else {b}
    }

    pub fn min(a: PolyVector, b: PolyVector) -> PolyVector {
        if a<b {a} else {b}
    }

}