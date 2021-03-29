use clap::{App, Arg};
use libpolymesh::file::{
    compressed::unpack_pmf,
    data::polymeta::parse_poly_meta
};

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
    let _ = unpack_pmf(root_path, output_path).unwrap();

    // Ensure there is a ploymeta inside the root
    let polymeta = parse_poly_meta(&format!("{}/polymeta.json", output_path).to_string()).unwrap();

    // Log file information
    println!("PMF version: {}", polymeta.version);
    println!("Children: {}", polymeta.children.len());

}
