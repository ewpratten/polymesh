use std::fs::File;
use flate2::{
    Compression,
    write::GzEncoder,
};

/// Pack a `PolyMesh` into a `.pmf` file.
///
/// This assumes the `directory` is the root directory of a PolyMesh mesh, and `output` is the path to the `.pmf` file to create.
///
/// Due to the design of the PolyMesh format, the root directory does not need to be the root of a mesh. If it is instead a sub-mesh, 
/// the sub-mesh will be exported as its own root mesh, relative to the origin
pub fn pack_pmf(directory: &str, output: &str) -> Result<(), std::io::Error> {

    // Create an output file
    let output_file = File::create(output).unwrap();

    // Compress the directory
    let encoder = GzEncoder::new(output_file, Compression::default());
    let mut encoder = tar::Builder::new(encoder);
    encoder.append_dir_all("", directory).unwrap();

    Ok(())

}
