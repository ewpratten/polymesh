use crate::common::{
    PolyMesh,
    MeshDef
};

pub fn get_flat_geometry(root_mesh: PolyMesh) -> Vec<MeshDef> {

    // Create a temporary lookup table for keeping track of duplicate meshes. This is used for runtime culling
    let mut runtime_culling_removals = Vec::new();

    // Create a list of all found geometry
    let mut all_geo = Vec::new();

    // Begin recursive search for geometry
    get_flat_geometry_recursive(&root_mesh, &mut all_geo, &runtime_culling_removals, root_mesh.uses_runtime_culling());

    return all_geo;

}

fn get_flat_geometry_recursive(root_mesh: &PolyMesh, all_geo: &mut Vec<MeshDef>, culling_list: &Vec<MeshDef>, culling_enabled: bool){

    // Store weather this mesh uses runtime culling
    let runtime_culling_enabled = root_mesh.uses_runtime_culling();

    // Search all children
    for child in &root_mesh.children {

        // Get the child's mesh
        let child_mesh = child.mesh.as_ref();

        // Check if the child contains geometry
        if child_mesh.contains_geometry() {

            // TODO
            // // Check if this child is in the culling list
            // if culling_list.iter().any(|&other| other.culled_by(child.mesh.as_ref()) ) {
            //     continue;
            // }
            // // Check if this child has a match in all_geo

            // Transform the child's geometry to an absolute position
            let abs_geometry = child_mesh.geometry.as_ref().unwrap().transformed_by(&child);

            // Add the geometry to the list
            all_geo.push(abs_geometry);

        }

    }

}