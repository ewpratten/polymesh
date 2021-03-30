use serde::{Deserialize, Serialize};
use std::{
    ops::{
        Add,
        Sub
    },
    cmp::{
        min,
        max
    }
};


/// Simple, JSON-Serializable color
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, PartialOrd)]
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

impl Add for PolyColor {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            r: min(self.r + other.r, 255),
            g: min(self.g + other.g, 255),
            b: min(self.b + other.b, 255),
            a: min(self.a + other.a, 255),
        }
    }

}

impl Sub for PolyColor {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            r: min(self.r - other.r, 255),
            g: min(self.g - other.g, 255),
            b: min(self.b - other.b, 255),
            a: min(self.a - other.a, 255),
        }
    }

}