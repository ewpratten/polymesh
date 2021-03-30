use super::super::super::transform::{
    PolyVector,
    PolyColor
};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

/// Definition of a mesh, and its geometry
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MeshDef {

    /// Color of the mesh
    pub color: PolyColor,

    /// Triangle geometry
    pub triangles: Option<Vec<[PolyVector;3]>>

}

impl MeshDef {

    /// Read a MeshDef from a file
    pub fn from_file(file_path: &str) -> Result<MeshDef> {
        // Read the file
        let file_contents = fs::read_to_string(file_path).unwrap();
        let mesh_def: MeshDef = serde_json::from_str(&file_contents.to_string()).unwrap();

        Ok(mesh_def)
    }

}