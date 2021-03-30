use super::super::super::transform::{
    PolyVector,
    PolyColor
};
use serde::{Deserialize, Serialize};

/// Definition of a mesh, and its geometry
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MeshDef {

    /// Color of the mesh
    pub color: PolyColor,

    /// Triangle geometry
    pub triangles: Option<Vec<[PolyVector;3]>>

}