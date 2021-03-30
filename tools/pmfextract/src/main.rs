use clap::{App, Arg};
use libpolymesh::read::unpack_pmf;

fn main() {
    let matches = App::new("PMFExtract")
    .author("Evan Pratten <ewpratten@gmail.com>")
    .arg(
        Arg::with_name("from")
            .long("from")
            .takes_value(true)
            .help("Path to PMF file")
            .required(true)
    ).arg(
        Arg::with_name("to")
            .long("to")
            .takes_value(true)
            .help("extraction root")
            .required(true)
    )
    .get_matches();

    let root_path = matches.value_of("from").unwrap();
    let output_path = matches.value_of("to").unwrap();

    // Unpack the file
    println!("Unpacking PolyMesh...");
    let _ = unpack_pmf(root_path, output_path).unwrap();
    println!("Done");

}
