use serde::{Deserialize, Serialize};
use serde_json::Result;
use super::vector::PolyVec;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq)]
pub struct PolyColor {

    /// Red color channel
    pub r: u8,

    /// Green color channel
    pub g: u8,

    /// Blue color channel
    pub b: u8,

    /// Alpha color channel
    pub a: u8
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PolyMesh {
    pub triangles: Vec<[PolyVec;3]>,
    pub color: PolyColor,
    pub emission: Option<f32>,
    pub albedo: Option<f32>
}

impl PolyMesh {

    pub fn build_transformed(other: &PolyMesh, transform: &PolyVec) -> PolyMesh {

        // Build new triangle list
        let mut triangles: Vec<[PolyVec;3]> = Vec::new();

        for triangle in &other.triangles {
            
            // Build new point list
            let mut vecs = [PolyVec { x:0.0, y:0.0, z:0.0 }; 3];

            // Add transformed points
            for i in 0..3 {
                vecs[i] = triangle[i] + *transform;
            }

            triangles.push(vecs);
        }

        PolyMesh {
            triangles, 
            color: other.color,
            emission: other.emission,
            albedo: other.albedo
        }

    }

}

pub fn mesh_from_file(file_path: &str) -> Result<PolyMesh> {

    // Read the file
    let file_contents = fs::read_to_string(file_path).unwrap();

    let poly_meta: PolyMesh = serde_json::from_str(&file_contents.to_string()).unwrap();

    Ok(poly_meta)

}