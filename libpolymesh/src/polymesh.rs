use super::file::data::{
    mesh::{
        PolyMesh,
        mesh_from_file
    },
    polymeta::{
        PolyMeta,
        parse_poly_meta
    }
};

pub struct FlatPolyMesh {
    pub root_meta: PolyMeta,
    pub flat_meshes: Vec<PolyMesh>
}

fn recurse_collect_meshes(path: &str, meta: &PolyMeta) -> Vec<PolyMesh> {
    let mut output = Vec::new();

    // If this meta is not a group, return its mesh
    if !meta.group {

        // Parse the mesh file
        let mesh = mesh_from_file(&format!("{}/mesh.json", path).to_string()).unwrap();

        // Build and return the output
        
        output.push(mesh);
        return output;
    } else {

        // Otherwise, recurse through children
        for child in &meta.children {

            // Build new root path
            let new_path = format!("{}{}", path, child.path.to_string());

            // Parse the new polymeta
            let new_meta = parse_poly_meta(&format!("{}/polymeta.json", new_path).to_string()).unwrap();

            // Get child mesh
            let mut child_mesh = recurse_collect_meshes(&new_path, &new_meta);

            output.append(&mut child_mesh);
        }
    }

    return output;

}

impl FlatPolyMesh {

    pub fn new(root_path: &str) -> FlatPolyMesh {

        // Get the root metadata
        let root_meta = parse_poly_meta(&format!("{}/polymeta.json", root_path).to_string()).unwrap();

        // Crawl the tree of children
        let flat_meshes = recurse_collect_meshes(root_path, &root_meta);

        FlatPolyMesh {
            root_meta,
            flat_meshes
        }

    }
}