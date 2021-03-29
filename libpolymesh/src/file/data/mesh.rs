use serde::{Deserialize, Serialize};
use serde_json::Result;
use super::vector::PolyVec;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct PolyColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PolyMesh {
    pub triangles: Vec<Vec<PolyVec>>,
    pub color: PolyColor
}

impl PolyMesh {

    pub fn build_transformed(other: &PolyMesh, transform: &PolyVec) -> PolyMesh {

        // Build new triangle list
        let mut triangles = Vec::new();

        for triangle in &other.triangles {
            
            // Build new point list
            let mut vecs = Vec::new();

            // Add transformed points
            for vec in triangle {
                vecs.push(*vec + *transform);
            }
            triangles.push(vecs);
        }

        PolyMesh {
            triangles, 
            color: other.color
        }

    }

}

pub fn mesh_from_file(file_path: &str) -> Result<PolyMesh> {

    // Read the file
    let file_contents = fs::read_to_string(file_path).unwrap();

    let poly_meta: PolyMesh = serde_json::from_str(&file_contents.to_string()).unwrap();

    Ok(poly_meta)

}