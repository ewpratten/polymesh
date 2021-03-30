extern crate tempdir;

use clap::{App, Arg};
use libpolymesh::{
    write::{
        write_unpacked_polymesh,
        pack_pmf
    },
    create::shapes::hexahedron::make_hexahedron,
    common::transform::{
        PolyVector,
        PolyColor
    }
};
use tempdir::TempDir;

fn main() {
    let matches = App::new("PMFCube")
    .author("Evan Pratten <ewpratten@gmail.com>")
    .arg(
        Arg::with_name("to")
            .long("to")
            .takes_value(true)
            .help("Output .pmf file path")
            .required(true)
    )
    .get_matches();

    let output_path = matches.value_of("to").unwrap();

    // Set up a workspace directory
    let workspace = TempDir::new("pmfcube").unwrap();
    let workspace_path = &workspace.path().to_str().unwrap();

    // Create a cube
    let cube = make_hexahedron(PolyVector::unit(), PolyVector { x: -1.0, y: -1.0, z: -1.0 }, PolyColor::green());

    // Write the cube to the workspace
    write_unpacked_polymesh(&cube, workspace_path);

    // Save the workspace to a pmf file
    pack_pmf(workspace_path, output_path);

}
