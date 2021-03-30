extern crate tempdir;
extern crate hex;

use clap::{App, Arg};
use std::{
    collections::HashMap,
    fs
};
use libpolymesh::file::{
    extracted::pack_pmf,
    data::{
        polymeta::{
            PolyMeta,
            PolyChild,
            LATEST_POLYMETA_VERSION
        },
        vector::PolyVec,
        mesh::PolyColor,
        generate::write::create_aabb
    },
};
use tempdir::TempDir;
use serde_json::Result;
use hex::FromHex;

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

    // Get a temporary output directory
    let temp_output_dir = TempDir::new("vox2pmf").unwrap();
    let temp_output_dir_path = &temp_output_dir.path().to_str().unwrap();

    // Parse the vox file
    let vox_data = dot_vox::load(vox_file_path).unwrap();

    // Create a list of children
    let mut vox_children = Vec::new();

    println!("Importing VOX. This may take a minute...");

    for (i, model) in vox_data.models.iter().enumerate() {
        vox_children.push(PolyChild {
            path: format!("/model_{}_gr", i).to_string(),
            transform: PolyVec {
                x: 0.0,
                y: 0.0,
                z: 0.0
            }
        });
    }
    
    // Build a top-level polymeta
    let root_meta = PolyMeta {
        version: LATEST_POLYMETA_VERSION,
        group: true,
        name: "Imported VOX Data".to_string(),
        metadata: HashMap::new(),
        children: vox_children
    };

    // Write top-level polymeta
    let root_meta_json = serde_json::to_string(&root_meta).unwrap();
    let _ = fs::write(&format!("{}/polymeta.json", temp_output_dir_path).to_string(), root_meta_json).unwrap();

    // Construct every child
    for (i, child) in root_meta.children.iter().enumerate() {

        // Build child directory
        let child_dir = &format!("{}{}", temp_output_dir_path, child.path).to_string();
        let _ = fs::create_dir_all(child_dir).unwrap();

        // List of voxels as children
        let mut voxel_children = Vec::new();

        // We handle the child's children first to keep the code a little simpler
        for (i, voxel) in vox_data.models[i].voxels.iter().enumerate() {

            // Get the voxel's material and color
            let material = &vox_data.materials[voxel.i as usize];
            let color = &vox_data.palette[voxel.i as usize];

            // Convert color to a hex string
            let color = format!("{:x}", color);
            
            // Convert hex to components
            let color_components = <[u8; 4]>::from_hex(color).unwrap();
            let color = PolyColor {
                r: color_components[0],
                g: color_components[1],
                b: color_components[2],
                a: color_components[3],
            };

            // Get material properties
            // TODO
            let emission = 0.0;
            let albedo = 0.0;

            // Create a directory for this voxel
            let voxel_rel_path = format!("/voxel_{}_gr", i).to_string();
            let voxel_dir = &format!("{}{}", child_dir, voxel_rel_path).to_string();
            let _ = fs::create_dir_all(voxel_dir).unwrap();

            // Write the voxel
            create_aabb(0.5, 0.5, color, Some(emission), Some(albedo), voxel_dir);

            // Create a PolyChild for the model to hold
            voxel_children.push(PolyChild {
                path: voxel_rel_path,
                transform: PolyVec {
                    x: voxel.x as f32,
                    y: voxel.y as f32,
                    z: voxel.z as f32,
                }
            })
        }

        // Build a polymeta for the child
        let child_meta = PolyMeta {
            version: LATEST_POLYMETA_VERSION,
            group: true,
            name: format!("VOX Model {}", i).to_string(),
            metadata: HashMap::new(),
            children: voxel_children
        };

        // Write child polymeta
        let child_meta_json = serde_json::to_string(&child_meta).unwrap();
        let _ = fs::write(&format!("{}/polymeta.json", child_dir).to_string(), child_meta_json).unwrap();

    }

    // Pack the pmf file
    pack_pmf(temp_output_dir_path, pmf_file_path);

    println!("Done!");

    Ok(())

}
