extern crate tempdir;
extern crate itertools;

use raylib::prelude::*;
use clap::{App, Arg};
use libpolymesh::{
    read::{
        unpack_pmf,
        read_unpacked_polymesh
    },
    util::flatlist::get_flat_geometry
};
use tempdir::TempDir;
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
    let unpack_output = TempDir::new("pmfview").unwrap();
    let unpack_output_path = &unpack_output.path().to_str().unwrap();
    let _ = unpack_pmf(pmf_file_path, unpack_output_path).unwrap();

    // Load the root file metadata
    let mesh = read_unpacked_polymesh(unpack_output_path).unwrap();
    let pmf_name = mesh.get_name();
    println!("Loaded PolyMesh: {}", pmf_name);

    // Get a flat list of all geometry that needs to be rendered
    let flat_geometry = get_flat_geometry(mesh);

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
            d_3d.draw_grid(100, 1.0);

            // Render every poly in the mesh
            for (i, mesh) in flat_geometry.iter().enumerate() {

                // Skip any invisible meshes
                if mesh.color.a == 0 {
                    continue;
                }
                
                if mesh.triangles.is_some(){
                    for poly_triangle in mesh.triangles.as_ref().unwrap() {

                        // Create vectors
                        let point_1 = Vector3::new(
                            poly_triangle[0].x,
                            poly_triangle[0].y,
                            poly_triangle[0].z
                        );
                        let point_2 = Vector3::new(
                            poly_triangle[1].x,
                            poly_triangle[1].y,
                            poly_triangle[1].z
                        );
                        let point_3 = Vector3::new(
                            poly_triangle[2].x,
                            poly_triangle[2].y,
                            poly_triangle[2].z
                        );

                        // Mesh color
                        let color = Color{ 
                            r: mesh.color.r,
                            g: mesh.color.g,
                            b: mesh.color.b,
                            a: 255
                            // a: mesh.color.a 
                        };

                        // Handle rendering
                        if wireframe_mode {
                            d_3d.draw_line_3D(point_1, point_2, Color::RED);
                            d_3d.draw_line_3D(point_2, point_3, Color::RED);
                            d_3d.draw_line_3D(point_3, point_1, Color::RED);
                        } else {
                            d_3d.draw_triangle3D(point_1, point_2, point_3, color);
                        }
                    }
                }
            }
        }

        // Render help text
        d.draw_text("Navigation: (ALT +) Middle click", 12, 10, 20, Color::BLACK);
        d.draw_text("Toggle wireframe: W", 12, 30, 20, Color::BLACK);
        d.draw_fps(12, 50);
    }

}
