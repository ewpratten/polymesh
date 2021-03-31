use super::super::super::{
    mesh::MeshType,
    transform::PolyVector
};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs;

pub const LATEST_POLY_META_VERSION: f32 = 1.1;

/// A JSON reference to another object.
/// 
/// ### Definition
/// This is the rust representation of the `child` elements found in `polymeta.json` files.
/// The equivilent polymeta and rust definitions would look as follows:
///
/// ```json
/// {
///     path": "/faces_gr/positive_x_geo",
///     "translation": {
///         "x":1.0,
///         "y":0.0,
///         "z":0.0
///     }   
/// }
/// ```
///
/// ```rust
/// use libpolymesh::prelude as pmf;
/// use libpolymesh::common::serialization::data::polymeta::PolyChildReference;
///
/// let reference = PolyChildReference {
///     path: "/faces_gr/positive_x_geo".to_string(),
///     translation: Some(pmf::PolyVector {
///         x: 1.0,
///         y: 0.0,
///         z: 0.0,
///     })
/// };
/// ```
/// 
/// In terms of parsing, the data in this structure is used to tell the pmf loader where to position which 
/// children. You can think of a `PolyChildReference` as an "include statement, with metadata". 
///
/// ### Some notes
///  - The translation is applied as a transform on everything under and including the child
///  - The path is relative to the `polymeta.json` that defines it, technically meaning meshes above the parent can be used as children (please don't do this)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PolyChildReference {

    /// Path describing where to find the child. This is relative to the parent
    pub path: String,

    /// Possible translation to apply to the child. This applies to all meshes under the child by proxy
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