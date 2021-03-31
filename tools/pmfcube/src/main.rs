use clap::{App, Arg};
use libpolymesh::prelude as pmf;

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

    // Create a cube
    let cube = pmf::make_hexahedron(pmf::PolyVector::unit(), pmf::PolyVector { x: -1.0, y: -1.0, z: -1.0 }, pmf::PolyColor::green());

    // Write the cube to the output file
    pmf::write_pmf(&cube, output_path);

}
