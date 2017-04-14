mod mesh;
mod scene;
mod algebra;
mod camera;
mod model;
mod color;
mod pixmap;
mod raytrace;

use std::time;
use algebra::Vec4;

fn main() {
    let mesh = mesh::Mesh::try_load_from_off("./meshes/teapot.off", mesh::PolygonWinding::Clockwise).unwrap();

    let model = model::Model {
        mesh: mesh,
        position: Vec4 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
            w: 1.0
        }
    };

    let camera = camera::Camera {
        position: Vec4 { x: 0.0, y: 0.0, z: 10.0, w: 1.0 },
        look_at: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        up: Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
    };

    let scene = scene::Scene {
        models: vec![model],
        camera: camera
    };

    let pixmap = pixmap::Pixmap::new(64, 64);

    let mut raytrace = raytrace::Raytrace::new(scene, pixmap);

    let ts_before = time::Instant::now();
    raytrace.run();
    let ts_after = time::Instant::now();
    println!("Rendering time: {:?}", ts_after.duration_since(ts_before));

    raytrace.pixmap.save_as_ppm("./output.ppm".to_string()).unwrap();
}

