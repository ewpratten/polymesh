extern crate hex;

use clap::{App, Arg};
use std::collections::HashMap;
use libpolymesh::prelude as pmf;
use serde_json::Result;
use hex::FromHex;
use indicatif::ProgressIterator;

struct VoxelContainer {
    mesh: Box<pmf::PolyMesh>, 
    path: String
}

fn main() -> Result<()> {
    let matches = App::new("VOX2PMF")
    .author("Evan Pratten <ewpratten@gmail.com>")
    .arg(
        Arg::with_name("from")
            .long("from")
            .takes_value(true)
            .help("VOX file")
            .required(true)
    ).arg(
        Arg::with_name("to")
            .long("to")
            .takes_value(true)
            .help("PMF file")
            .required(true)
    )
    .get_matches();

    let vox_file_path = matches.value_of("from").unwrap();
    let pmf_file_path = matches.value_of("to").unwrap();

    // Parse the vox file
    let vox_data = dot_vox::load(vox_file_path).unwrap();

    // Build a root polymesh
    let mut root_mesh = pmf::PolyMesh::new(pmf::MeshType::Group, None);
    root_mesh.set_name("Imported VOX".to_string());

    // Handle every vox model
    println!("Performing format translation...");
    for (i, model) in vox_data.models.iter().enumerate() {

        // Create a model mesh
        let mut model_mesh = pmf::PolyMesh::new(pmf::MeshType::Group, None);
        model_mesh.set_name(format!("Model {}", i).to_string());
        model_mesh.enable_runtime_culling();

        // A map of known voxels. This is used for storage optimization, pre-compression
        let mut known_voxels: HashMap<String, VoxelContainer> = HashMap::new();

        // Handle every voxel in the model
        for (i, voxel) in model.voxels.iter().enumerate().progress() {

            // Get the voxel's material and color
            let material = &vox_data.materials[voxel.i as usize];
            let color = &vox_data.palette[voxel.i as usize];

            // Build a descriptor string for this voxel
            let descriptor = format!("{:?}-{}", material.properties, color);

            // If this voxel is not known, create a new mesh for it
            if !known_voxels.contains_key(&descriptor) {

                // Convert color to an object repr (Magica uses BGRA, I use RGBA)
                let color = format!("{:x}", color);
                let color_components = <[u8; 4]>::from_hex(color).unwrap();
                let color = pmf::PolyColor {
                    r: color_components[2],
                    g: color_components[1],
                    b: color_components[0],
                    a: color_components[3],
                };

                // Skip transparent voxels
                if color.a == 0 {
                    continue;
                }

                // Create a cube for the voxel
                let cube = pmf::make_hexahedron(pmf::PolyVector { x: 0.5, y: 0.5, z: 0.5 }, pmf::PolyVector { x: -0.5, y: -0.5, z: -0.5 }, color);

                // Save the cube
                known_voxels.insert(descriptor.to_string(), VoxelContainer { 
                    mesh: Box::new(cube), 
                    path: format!("/voxel_{}_gr", i).to_string()
                });

            }

            // Add a the voxel as a child
            model_mesh.add_child(pmf::TransPolyMeshPtr {
                path: (*known_voxels[&descriptor].path).to_string(),
                mesh: known_voxels[&descriptor].mesh.clone(),
                translation: Some(pmf::PolyVector {
                    x: voxel.x as f32,
                    y: voxel.y as f32,
                    z: voxel.z as f32,
                })
            })

        }

        // Add the model to the root mesh
        root_mesh.add_child(pmf::TransPolyMeshPtr {
            path: format!("/model_{}_gr", i).to_string(),
            mesh: Box::new(model_mesh),
            translation: None // TODO: I think this needs to change??
        })

    }

    // Write the generated mesh
    println!("Writing PolyMesh to disk...");
    pmf::write_pmf(&root_mesh, pmf_file_path);

    Ok(())

}