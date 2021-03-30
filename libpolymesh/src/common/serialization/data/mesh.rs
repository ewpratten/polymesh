use super::super::super::{
    transform::{
        PolyVector,
        PolyColor
    },
    TransPolyMeshPtr
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

    /// Check if this mesh can be culled by another mesh
    pub fn culled_by(&self, other: &MeshDef) -> bool {

        if self.triangles.is_some() && other.triangles.is_some() {
            return self.triangles.as_ref() == other.triangles.as_ref();
        }

        return false;

    }

    /// Copy this mesh to be absolutely transformed by its parent (used in rendering mostly)
    pub fn transformed_by(&self, parent: &TransPolyMeshPtr) -> Self {

        // Handle triangles
        let mut triangles = None;
        if self.triangles.is_some() {

            // Build new triangle list
            let mut tmp_triangles: Vec<[PolyVector;3]> = Vec::new();

            for triangle in self.triangles.as_ref().unwrap() {
                
                // Build new point list
                let mut vecs = [PolyVector { x:0.0, y:0.0, z:0.0 }; 3];

                // Add transformed points
                for i in 0..3 {
                    if parent.translation.is_some() {
                        vecs[i] = triangle[i] + *parent.translation.as_ref().unwrap();
                    } else {
                        vecs[i] = triangle[i];
                    }
                }

                tmp_triangles.push(vecs);
            }

            triangles = Some(tmp_triangles);
        }

        Self {
            triangles, 
            color: self.color,
            // emission: self.emission,
            // albedo: self.albedo
        }

    }

}