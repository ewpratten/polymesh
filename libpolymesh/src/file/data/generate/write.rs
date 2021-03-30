use std::{
    collections::HashMap,
    fs
};
use super::super::{
    polymeta::{
        PolyMeta,
        PolyChild,
        LATEST_POLYMETA_VERSION
    },
    mesh::PolyColor,
    vector::PolyVec
};
use super::shapes::{
    gen_aaplane,
    AxisDirection
};
use serde_json::Result;


pub fn create_aabb(width: f32, height: f32, color: PolyColor, emission: Option<f32>, albedo: Option<f32>, root_dir: &str) -> Result<()> {

    // Create the root dir
    let _ = fs::create_dir_all(root_dir).unwrap();

    // Build a root polymeta
    let root_meta = PolyMeta {
        version: LATEST_POLYMETA_VERSION,
        group: true,
        name: "AABB Group".to_string(),
        metadata: HashMap::new(),
        children: vec![
            PolyChild {
                path: "/faces/positive_x_face_geo".to_string(),
                transform: PolyVec {
                    x: width,
                    y: 0.0,
                    z: 0.0
                }
            },
            PolyChild {
                path: "/faces/positive_y_face_geo".to_string(),
                transform: PolyVec {
                    x: 0.0,
                    y: height,
                    z: 0.0
                }
            },
            PolyChild {
                path: "/faces/positive_z_face_geo".to_string(),
                transform: PolyVec {
                    x: 0.0,
                    y: 0.0,
                    z: width
                }
            },
            PolyChild {
                path: "/faces/negative_x_face_geo".to_string(),
                transform: PolyVec {
                    x: -(width),
                    y: 0.0,
                    z: 0.0
                }
            },
            PolyChild {
                path: "/faces/negative_y_face_geo".to_string(),
                transform: PolyVec {
                    x: 0.0,
                    y: -(height),
                    z: 0.0
                }
            },
            PolyChild {
                path: "/faces/negative_z_face_geo".to_string(),
                transform: PolyVec {
                    x: 0.0,
                    y: 0.0,
                    z: -(width)
                }
            }
        ]
    };

    // Write out the root metadata
    let root_meta_json = serde_json::to_string(&root_meta).unwrap();
    fs::write(&format!("{}/polymeta.json", root_dir).to_string(), root_meta_json);

    for child in &root_meta.children {

        // Create output root
        let child_root = &format!("{}{}", root_dir, child.path).to_string();

        // Init directory
        let _ = fs::create_dir_all(child_root).unwrap();

        // Build a simple polymeta for the face
        let child_meta = PolyMeta {
            version: LATEST_POLYMETA_VERSION,
            group: false,
            name: "".to_string(),
            metadata: HashMap::new(),
            children: Vec::new()
        };

        // Write out the child metadata
        let child_meta_json = serde_json::to_string(&child_meta).unwrap();
        fs::write(&format!("{}/polymeta.json", child_root).to_string(), child_meta_json);

        // Init a mesh
        let mut child_mesh;

        // Handle faces
        if child.path.contains("positive_x") {
            child_mesh = gen_aaplane(AxisDirection::PositiveX, width, height, color, emission, albedo);
        } else if child.path.contains("positive_y") {
            child_mesh = gen_aaplane(AxisDirection::PositiveY, width, width, color, emission, albedo);
        } else if child.path.contains("positive_z") {
            child_mesh = gen_aaplane(AxisDirection::PositiveZ, width, height, color, emission, albedo);
        } else if child.path.contains("negative_x") {
            child_mesh = gen_aaplane(AxisDirection::NegativeX, width, height, color, emission, albedo);
        } else if child.path.contains("negative_y") {
            child_mesh = gen_aaplane(AxisDirection::NegativeY, width, width, color, emission, albedo);
        } else {
            child_mesh = gen_aaplane(AxisDirection::NegativeZ, width, height, color, emission, albedo);
        }

        // Write mesh
        let child_mesh_json = serde_json::to_string(&child_mesh).unwrap();
        fs::write(&format!("{}/mesh.json", child_root).to_string(), child_mesh_json);

    }


    Ok(())

}