extern crate derive_more;

use serde::{Deserialize, Serialize};
use derive_more::{Add, Sub, Mul, Div};

/// Simple, JSON-Serializable color
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd, Add, Sub, Mul, Div)]
pub struct PolyColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl PolyColor {

    pub fn black() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub fn red() -> Self {
        Self {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn green() -> Self {
        Self {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub fn blue() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub fn clear() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }

}