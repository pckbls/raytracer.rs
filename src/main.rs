mod mesh;
mod scene;
mod algebra;
mod camera;
mod model;
mod color;
mod pixmap;
mod raytrace;

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
    raytrace.run();
    raytrace.pixmap.save_as_ppm("./output.ppm".to_string()).unwrap();
}

