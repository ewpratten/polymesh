use std::fs::{
    File,
    create_dir_all
};
use flate2::read::GzDecoder;
use tar::Archive;

pub fn unpack_pmf(pmf_file: &str, output_dir: &str) -> Result<(), std::io::Error> {

    // Set up the output directory
    create_dir_all(output_dir).unwrap();

    // Unpack the pmf file
    let archive = File::open(pmf_file).unwrap();
    let decoder = GzDecoder::new(archive);
    let mut decoder = Archive::new(decoder);
    decoder.unpack(output_dir).unwrap();

    Ok(())
    
}