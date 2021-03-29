use serde::{Deserialize, Serialize};
use serde_json::Result;
use super::vector::PolyVec;
use std::{
    collections::HashMap,
    fs
};

#[derive(Serialize, Deserialize)]
pub struct PolyChild {
    pub path: String,
    pub transform: PolyVec
}

#[derive(Serialize, Deserialize)]
pub struct PolyMeta {
    pub version: f32,
    pub group: bool,
    pub metadata: HashMap<String, String>,
    pub children: Vec<PolyChild>
}

pub fn parse_poly_meta(file_path: &str) -> Result<PolyMeta> {

    // Read the file
    let file_contents = fs::read_to_string(file_path).unwrap();

    let poly_meta: PolyMeta = serde_json::from_str(&file_contents.to_string()).unwrap();

    Ok(poly_meta)

}