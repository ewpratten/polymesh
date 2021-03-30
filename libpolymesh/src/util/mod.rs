
pub fn make_polymeta_file_path(root: &str) -> String {
    return format!("{}/polymeta.json", root).to_string();
}

pub fn make_mesh_file_path(root: &str) -> String {
    return format!("{}/mesh.json", root).to_string();
}

pub fn make_child_file_path(root: &str, child: &str) -> String {
    return format!("{}{}", root, child).to_string();
}