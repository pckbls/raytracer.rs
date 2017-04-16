mod mesh;
mod scene;
mod algebra;
mod camera;
mod model;
mod lighting;
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
        light_sources: vec![
            lighting::LightSource {
                position: Vec4::new(0.0, 0.0, 0.0, 1.0),
                ambient_color: color::Color { r: 20, g: 20, b: 20 },
                diffuse_color: color::Color { r: 0, g: 0, b: 0 },
            },
            lighting::LightSource {
                position: Vec4::new(3.0, 3.0, 3.0, 1.0),
                ambient_color: color::Color { r: 0, g: 0, b: 0 },
                diffuse_color: color::Color { r: 0, g: 100, b: 200 },
            },
            lighting::LightSource {
                position: Vec4::new(-3.0, -3.0, -3.0, 1.0),
                ambient_color: color::Color { r: 0, g: 0, b: 0 },
                diffuse_color: color::Color { r: 150, g: 0, b: 0 },
            }
        ],
        camera: camera
    };

    let mut pixmap = pixmap::Pixmap::new(128, 128);

    let ts_before = time::Instant::now();
    raytrace::render_scene(&scene, &mut pixmap);
    let ts_after = time::Instant::now();

    println!("Rendering time: {:?}", ts_after.duration_since(ts_before));

    pixmap.save_as_ppm("./output.ppm".to_string()).unwrap();
}

