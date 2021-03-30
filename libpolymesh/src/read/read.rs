use super::super::{
    common::{
        serialization::data::{
            polymeta::PolyMeta,
            mesh::MeshDef
        },
        mesh::{
            PolyMesh,
            MeshType,
            TransPolyMeshPtr
        }
    },
    util::{
        make_polymeta_file_path,
        make_mesh_file_path,
        make_child_file_path
    }
};
use serde_json::Result;
use std::collections::HashMap;

pub fn read_unpacked_polymesh(root_path: &str) -> Result<PolyMesh> {

    // Create a lookup table to be used for caching meshes
    let mut mesh_table = HashMap::new();

    // Read recursively
    return read_unpacked_polymesh_recursive(root_path, &mut mesh_table)
}

fn read_unpacked_polymesh_recursive(root_path: &str, resolved_lookup: &mut HashMap<String, Box<PolyMesh>>) -> Result<PolyMesh> {

    // Read the polymeta
    let polymeta_path = make_polymeta_file_path(root_path);
    let polymeta = PolyMeta::from_file(&polymeta_path).expect(&format!("Could not read polymeta.json from: {}", polymeta_path));

    // If needed, read geometry from mesh.json
    let mut geometry: Option<MeshDef> = None;
    if polymeta.mesh_type == MeshType::Geometry || polymeta.mesh_type == MeshType::GeoGroup {

        // Read the mesh
        let mesh_path = make_mesh_file_path(root_path);
        let mesh_def = MeshDef::from_file(&mesh_path).expect(&format!("Could not read mesh.json from: {}", mesh_path));
        geometry = Some(mesh_def);

    }

    // Create the base output mesh
    let mut output = PolyMesh::new(polymeta.mesh_type, geometry);

    // Copy needed data
    output.metadata = polymeta.metadata.clone();

    // Add all children
    for child in &polymeta.children {

        // Get the path to the child
        let child_path = make_child_file_path(root_path, &child.path);

        // If this child has not already been resolved, resolve it
        if !resolved_lookup.contains_key(&child_path) {

            // Read the mesh
            let read_mesh = read_unpacked_polymesh_recursive(&child_path, resolved_lookup).unwrap();

            // Add to the table
            resolved_lookup.insert(child_path.to_string(), Box::new(read_mesh));

        }

        // Get a box of the child mesh
        let child_mesh = resolved_lookup[&child_path].clone();

        // Build a reference to the child
        let child_ref = TransPolyMeshPtr {
            path: (*child.path).to_string(),
            mesh: child_mesh,
            translation: child.translation
        };

        // Add the child to the mesh
        output.add_child(child_ref);

    }

    Ok(output)

}