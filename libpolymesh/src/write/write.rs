use super::super::{
    common::mesh::PolyMesh,
    util::{
        make_polymeta_file_path,
        make_mesh_file_path,
        make_child_file_path
    }
};
use std::fs;
use serde_json::Result;

/// Write a PolyMesh to a root directory in "unpacked" mode
pub fn write_unpacked_polymesh(mesh: &PolyMesh, root_path: &str) -> Result<()> {

    // Create a list to keep track of already written meshes (this prevents infinite write lock)
    let mut written_children = Vec::new();

    // Handle writing
    return write_unpacked_polymesh_recursive(mesh, root_path, &mut written_children);

}

fn write_unpacked_polymesh_recursive(mesh: &PolyMesh, root_path: &str, written_children: &mut Vec<String>) -> Result<()> {

    // Ensure the root path exists
    let _ = fs::create_dir_all(root_path).unwrap();

    // Convert the mesh to a polymeta file
    let metadata = mesh.to_poly_meta();

    // Write the metadata to file
    let meta_path = make_polymeta_file_path(root_path);
    let meta_json = serde_json::to_string(&metadata).unwrap();
    fs::write(meta_path, meta_json);

    // If there is geometry, write it too
    if mesh.contains_geometry() {
        let mesh_path = make_mesh_file_path(root_path);
        let mesh_json = serde_json::to_string(&mesh.geometry).unwrap();
        fs::write(mesh_path, mesh_json);
    }

    // Write every child
    for child in &mesh.children {

        // Get the child's file path 
        let child_path = make_child_file_path(root_path, &child.path);

        // If this child has not been written, write it
        if !written_children.contains(&child_path) {
            let _ = write_unpacked_polymesh_recursive(child.mesh.as_ref(), &child_path, written_children).unwrap();

            // Track the child to prevent overwrite
            written_children.push(child_path);
        }
    }

    Ok(())
}