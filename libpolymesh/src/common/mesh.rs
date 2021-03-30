use serde::{Deserialize, Serialize};
use super::transform::PolyVector;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MeshType {
    Group,
    Geometry,
    GeoGroup
}

/// TransPolyMeshPtr is a small wrapper around a reference to a PolyMesh, describing a transformation on the mesh
#[derive(Debug, Clone, PartialEq)]
pub struct TransPolyMeshPtr {

    /// File path to the referenced mesh
    pub path: String,

    /// Mesh reference
    pub mesh: Box<PolyMesh>,

    /// Optional translation
    pub translation: Option<PolyVector>

}

/// A PolyMesh is any mesh, weather it contains geometry, other meshes, or a mix of both
#[derive(Debug, Clone, PartialEq)]
pub struct PolyMesh {

    /// The type of this mesh
    pub mesh_type: MeshType,

    /// Arbitrary metadata
    pub metadata: HashMap<String, String>,

    /// All children of this mesh
    pub children: Vec<TransPolyMeshPtr>

}

impl PolyMesh {

    /// Create a new PolyMesh
    pub fn new(mesh_type: MeshType) -> Self {
        PolyMesh {
            mesh_type,
            metadata: HashMap::new(),
            children: Vec::new()
        }
    }

    /// Add arbitrary data to the mesh
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    /// Add a child to the mesh
    pub fn add_child(&mut self, child: TransPolyMeshPtr) {
        self.children.push(child);
    }

    // Try to get arbitrary metadata
    pub fn try_get_meta_field(&self, key: &str) -> Result<&String, ()> {
        if self.metadata.contains_key(key) {
            Ok(&self.metadata[key])
        } else {
            Err(())
        }
    }

    /// Try to fetch the mesh name from metadata
    pub fn get_name(&self) -> Result<&String, ()> {
        return self.try_get_meta_field("name");
    }

    /// Get if this mesh is requesting the BETA "Runtime Culling" feature
    pub fn uses_runtime_culling(&self) -> bool {
        return match self.try_get_meta_field("name") {
            Ok(result) => result == "on",
            Err(_) => false
        };
    }

}