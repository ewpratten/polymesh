use clap::{App, Arg};
use libpolymesh::write::pack_pmf;

fn main() {
    let matches = App::new("PMFPack")
    .author("Evan Pratten <ewpratten@gmail.com>")
    .arg(
        Arg::with_name("from")
            .long("from")
            .takes_value(true)
            .help("Path to extracted PMF root")
            .required(true)
    ).arg(
        Arg::with_name("to")
            .long("to")
            .takes_value(true)
            .help("Output .pmf file path")
            .required(true)
    )
    .get_matches();

    let root_path = matches.value_of("from").unwrap();
    let output_path = matches.value_of("to").unwrap();

    // Pack the file
    println!("Packing PolyMesh...");
    let _ = pack_pmf(root_path, output_path).unwrap();
    println!("Done");

}
