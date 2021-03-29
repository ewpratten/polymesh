use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PolyVec {
    pub x: f64,
    pub y: f64,
    pub z: f64
}