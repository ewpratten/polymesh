extern crate colored;
extern crate tempdir;

use clap::{App, Arg};
use libpolymesh::file::{
    compressed::unpack_pmf,
    data::polymeta::parse_poly_meta
};
use tempdir::TempDir;
use colored::*;

fn recurse_display(path: &str, level: usize) {

    // Get the metadata for this submesh
    let mesh_meta = parse_poly_meta(&format!("{}/polymeta.json", path).to_string()).unwrap();

    // Build the display string
    let padding = std::iter::repeat(" ").take(level * 2).collect::<String>();
    let path = String::from(path);
    let mut name = path.split("/").last().unwrap().white();
    let nice_name = mesh_meta.name.white();

    // Handle root
    if level == 0 {
        name = "/".white();
    }
    
    if mesh_meta.group {
        name = name.blue();
    } else {
        name = name.green();
    }

    // Show entry
    println!("{} - {} [{}]", padding, name, nice_name);

    // Recurse every child
    for child in &mesh_meta.children {

        // Build the child's path
        let child_path = format!("{}{}", path, child.path);

        // Recurse
        recurse_display(&child_path.to_string(), level + 1);
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
    .get_matches();

    let pmf_file_path = matches.value_of("file").unwrap();

    // Unpack the file to /tmp
    let unpack_output = TempDir::new("pmftree").unwrap();
    let unpack_output_path = &unpack_output.path().to_str().unwrap();
    let _ = unpack_pmf(pmf_file_path, unpack_output_path).unwrap();

    // Display the tree
    recurse_display(unpack_output_path, 0);
}
