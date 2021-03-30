use super::super::super::{
    mesh::MeshType,
    transform::PolyVector
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

pub const LATEST_POLY_META_VERSION: f32 = 1.1;

/// A JSON reference to another object
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolyChildReference {

    /// Path describing where to find the child
    pub path: String,

    /// Possible translation to apply to the child
    pub translation: Option<PolyVector>

}

/// Directly describes a PolyMeta file
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolyMeta {

    /// PolyMeta version
    pub version: f32,

    /// Type of this mesh
    #[serde(rename = "type")]
    pub mesh_type: MeshType,

    /// Arbitrary metadata
    pub metadata: HashMap<String, String>,

    /// Children of the mesh
    pub children: Vec<PolyChildReference>

}

impl PolyMeta {

    /// Read a PolyMeta object from a file
    pub fn from_file(file_path: &str) -> Result<PolyMeta> {
        // Read the file
        let file_contents = fs::read_to_string(file_path).unwrap();
        let poly_meta: PolyMeta = serde_json::from_str(&file_contents.to_string()).unwrap();

        Ok(poly_meta)
    }

}