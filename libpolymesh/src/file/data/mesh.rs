use serde::{Deserialize, Serialize};
use serde_json::Result;
use super::vector::PolyVec;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct PolyColor {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

#[derive(Serialize, Deserialize)]
pub struct PolyMesh {
    pub triangles: Vec<Vec<PolyVec>>,
    pub color: PolyColor
}

pub fn mesh_from_file(file_path: &str) -> Result<PolyMesh> {

    // Read the file
    let file_contents = fs::read_to_string(file_path).unwrap();

    let poly_meta: PolyMesh = serde_json::from_str(&file_contents.to_string()).unwrap();

    Ok(poly_meta)

}