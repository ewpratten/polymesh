use crate::common::{
    PolyMesh,
    MeshDef,
    MeshType,
    transform::{
        PolyVector,
        PolyColor
    }
};

/// Make a plane-like shape with 4 sides
pub fn make_quad(top_left: PolyVector, top_right: PolyVector, bottom_left: PolyVector, bottom_right: PolyVector, color: PolyColor) -> PolyMesh {

    // Create a list of triangles
    let mut triangles: Vec<[PolyVector;3]> = Vec::new();

    // Add the left triangle
    triangles.push([
        top_left,
        bottom_left,
        bottom_right
    ]);

    // Add the right triangle
    triangles.push([
        top_right,
        top_left,
        bottom_right
    ]);

    // Create the geo
    let geometry = Some(MeshDef {
        color: color,
        triangles: Some(triangles)
    });

    // Create the PolyMesh
    let mut mesh = PolyMesh::new(MeshType::Geometry, geometry);
    mesh.set_name("Quad Face".to_string());
    return mesh;

}
