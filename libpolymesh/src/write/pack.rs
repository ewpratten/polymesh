use std::fs::File;
use flate2::{
    Compression,
    write::GzEncoder,
};

pub fn pack_pmf(directory: &str, output: &str) -> Result<(), std::io::Error> {

    // Create an output file
    let output_file = File::create(output).unwrap();

    // Compress the directory
    let encoder = GzEncoder::new(output_file, Compression::default());
    let mut encoder = tar::Builder::new(encoder);
    encoder.append_dir_all("", directory).unwrap();

    Ok(())

}
