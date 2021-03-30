use super::file::data::{
    vector::PolyVec,
    mesh::{
        PolyMesh,
        mesh_from_file
    },
    polymeta::{
        PolyMeta,
        parse_poly_meta
    }
};
use std::collections::HashMap;

pub struct FlatPolyMesh {
    pub root_meta: PolyMeta,
    pub flat_meshes: Vec<PolyMesh>
}

fn recurse_collect_meshes(path: &str, meta: &PolyMeta, transform: PolyVec, cache: &mut HashMap<String, PolyMesh>) -> Vec<PolyMesh> {
    let mut output = Vec::new();

    // If this meta is not a group, return its mesh
    if !meta.group {

        let file_path  = &format!("{}/mesh.json", path).to_string();

        // Try to save time using the cache
        if !cache.contains_key(file_path){
            
            // Parse the mesh file
            let mesh = mesh_from_file(file_path).unwrap();

            // Add cache entry
            cache.insert(file_path.to_string(), mesh);
        }

        // Transform the mesh to be absolutely positioned
        let new_mesh = PolyMesh::build_transformed(&cache[file_path], &transform);

        // Build and return the output
        output.push(new_mesh);
        return output;
    
    } else {

        // Otherwise, recurse through children
        for child in &meta.children {

            // Build new root path
            let new_path = format!("{}{}", path, child.path.to_string());

            // Parse the new polymeta
            let new_meta = parse_poly_meta(&format!("{}/polymeta.json", new_path).to_string()).unwrap();

            // Build on the the transform
            let new_transform = transform + child.transform;

            // Set up a temp cache
            let mut temp_cache = &mut *cache;

            // Get child mesh
            let mut child_mesh = recurse_collect_meshes(&new_path, &new_meta, new_transform, &mut temp_cache);

            output.append(&mut child_mesh);
        }
    }

    return output;

}

impl FlatPolyMesh {

    pub fn new(root_path: &str) -> FlatPolyMesh {

        // Get the root metadata
        let root_meta = parse_poly_meta(&format!("{}/polymeta.json", root_path).to_string()).unwrap();

        // Create a load cache
        let mut mesh_cache = HashMap::new();

        // Crawl the tree of children
        let flat_meshes = recurse_collect_meshes(root_path, &root_meta, PolyVec::zero(), &mut mesh_cache);

        FlatPolyMesh {
            root_meta,
            flat_meshes
        }

    }
}