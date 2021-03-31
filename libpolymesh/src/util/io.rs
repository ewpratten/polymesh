extern crate tempdir;

use crate::{
    read::{
        unpack_pmf,
        read_unpacked_polymesh
    },
    write::{
        write_unpacked_polymesh,
        pack_pmf
    },
    common::PolyMesh
};
use tempdir::TempDir;
use serde_json::Result;

/// Read a PMF file into a PolyMesh object
pub fn read_pmf(file_path: &str) -> Result<PolyMesh> {

    // Set up a workspace directory
    let workspace = TempDir::new("libpolymesh").unwrap();
    let workspace_path = &workspace.path().to_str().unwrap();

    // Unpack the file into the workspace
    let _ = unpack_pmf(file_path, workspace_path).unwrap();

    // Load the PMF into memory and return it
    return read_unpacked_polymesh(workspace_path);

}

/// Write a PolyMesh to a .pmf file
pub fn write_pmf(mesh: &PolyMesh, output_path: &str) {

    // Set up a workspace directory
    let workspace = TempDir::new("libpolymesh").unwrap();
    let workspace_path = &workspace.path().to_str().unwrap();

    // Write the mesh
    let _ = write_unpacked_polymesh(mesh, workspace_path).unwrap();

    // Pack the mesh
    let _ = pack_pmf(workspace_path, output_path).unwrap();

}