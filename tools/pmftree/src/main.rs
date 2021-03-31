extern crate colored;

use clap::{App, Arg};
use libpolymesh::prelude as pmf;
use colored::*;

fn recurse_display(mesh: &pmf::PolyMesh, path: &str, level: usize, unique_only: bool, seen_meshes: &mut Vec<String>) {

    // Skip non-unique meshes if needed
    if unique_only {
        if seen_meshes.contains(&path.to_string()) && mesh.mesh_type != pmf::MeshType::Geometry {
            return;
        }
        seen_meshes.push(path.to_string());
    }

    // Build the display string
    let padding = std::iter::repeat(" ").take(level * 2).collect::<String>();
    let mut name = path.white();
    let nice_name = mesh.get_name().white();

    // Handle coloring the name based on type    
    if mesh.mesh_type == pmf::MeshType::Group {
        name = name.blue();
    } else if mesh.mesh_type == pmf::MeshType::Geometry {
        name = name.green();
    } else {
        name = name.red();
    }

    // Show entry
    println!("{} - {} [{}]", padding, name, nice_name);

    // Recurse every child
    for child in &mesh.children {

        // Recurse
        recurse_display(child.mesh.as_ref(), &child.path, level + 1, unique_only, seen_meshes);
    }

}

fn main() {
    let matches = App::new("PMFTree")
    .author("Evan Pratten <ewpratten@gmail.com>")
    .arg(
        Arg::with_name("file")
            .long("file")
            .takes_value(true)
            .help("PMF file")
            .required(true)
    )
    .arg(
        Arg::with_name("unique")
            .long("unique")
            .short("u")
            .takes_value(false)
            .help("Only Show unique meshes")
    )
    .get_matches();

    let pmf_file_path = matches.value_of("file").unwrap();
    let unique_only: bool = matches.is_present("unique");

    // Read the pmf file
    let root_mesh = pmf::read_pmf(pmf_file_path).unwrap();

    // A list of already seen meshes for uniqueness check
    let mut seen_meshes = Vec::new();

    // Display the tree
    recurse_display(&root_mesh, "/", 0, unique_only, &mut seen_meshes);
}
