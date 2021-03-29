use raylib::prelude::*;
use clap::{App, Arg};
use libpolymesh::file::{
    compressed::unpack_pmf,
    data::polymeta::parse_poly_meta
};
use std::env;
use raylib::ffi::KeyboardKey::KEY_W;

fn main() {
    let matches = App::new("PMFView")
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
    let unpack_output = env::temp_dir();
    let unpack_output_path = &unpack_output.display().to_string();
    unpack_pmf(pmf_file_path, unpack_output_path);

    // Load the root file metadata
    let root_pmf_metadata = parse_poly_meta(&format!("{}/polymeta.json", unpack_output_path).to_string()).unwrap();
    let pmf_name = &root_pmf_metadata.metadata["name"];

    // Parse the PMF file
    // TODO:

    println!("Loaded PolyMesh: {}", pmf_name);

    // Set up GUI
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title(&pmf_name.to_string())
        .build();

    // Configure a camera
    let mut camera = Camera3D::perspective(
        Vector3::new(10.0, 10.0, 10.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        60.0,
    );

    // Configure rendering
    rl.set_camera_mode(&camera, CameraMode::CAMERA_FREE);
    rl.set_target_fps(60);

    // Mutable state
    let mut wireframe_mode = false;

    while !rl.window_should_close() {
        // Update the camera
        rl.update_camera(&mut camera);

        // Check for W key (toggle wireframe)
        if rl.is_key_pressed(KEY_W) {
            wireframe_mode = !wireframe_mode;
        }

        // Get a drawing context
        let mut d = rl.begin_drawing(&thread);

        // Clear the background
        d.clear_background(Color::WHITE);

        // 3D context 
        {
            let mut d_3d = d.begin_mode3D(camera);

            // Graw ground plane
            d_3d.draw_grid(1000, 1.0);

            // Render test cube
            if wireframe_mode {

                d_3d.draw_cube_wires(Vector3::new(0.0, 0.0, 0.0), 1.0, 5.0, 32.0, Color::BLUE);
            }else {

                d_3d.draw_cube(Vector3::new(0.0, 0.0, 0.0), 1.0, 5.0, 32.0, Color::BLUE);
            }
        }

        // Render help text
        d.draw_text("Navigation: (ALT +) Middle click", 12, 12, 20, Color::BLACK);
        d.draw_text("Toggle wireframe: W", 12, 30, 20, Color::BLACK);
    }

}
