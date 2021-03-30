extern crate derive_more;

use serde::{Deserialize, Serialize};
use derivative::Derivative;
use derive_more::{Add, Sub, Mul, Div};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Derivative, PartialOrd, Add, Sub, Mul, Div)]
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
