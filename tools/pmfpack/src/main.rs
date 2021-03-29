use clap::{App, Arg};
use libpolymesh::file::{
    extracted::pack_pmf,
    data::polymeta::parse_poly_meta
};

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

    // Ensure there is a ploymeta inside the root
    let polymeta = parse_poly_meta(&format!("{}/polymeta.json", root_path).to_string()).unwrap();

    // Log file information
    println!("PMF version: {}", polymeta.version);
    println!("Children: {}", polymeta.children.len());

    // Pack the file
    let _ = pack_pmf(root_path, output_path).unwrap();

}
