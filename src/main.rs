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
    let mesh = mesh::Mesh::try_load_from_off("./meshes/cowboyhut.off", mesh::PolygonWinding::Clockwise).unwrap();

    let model = model::Model::new(mesh.clone(),
                                  Vec4::new(-3.0, -1.0, 0.0, 1.0),
                                  algebra::Mat4::rotate(&algebra::Mat4::identity(),
                                                        algebra::Angle::Degrees(40.0),
                                                        &algebra::Vec4::new(0.0, 1.0, 0.0, 0.0)),
                                  Vec4::new(5.0, 5.0, 5.0, 1.0));

    let mesh_plane4x4 = mesh::Mesh::try_load_from_off("./meshes/plane4x4.off", mesh::PolygonWinding::Clockwise).unwrap();

    let floor = model::Model::new(mesh_plane4x4.clone(),
                                  Vec4::new(0.0, -1.0, 0.0, 1.0),
                                  algebra::Mat4::rotate(&algebra::Mat4::identity(),
                                                        algebra::Angle::Degrees(90.0),
                                                        &algebra::Vec4::new(1.0, 0.0, 0.0, 0.0)),
                                  Vec4::new(10.0, 10.0, 10.0, 1.0));

    let backw = model::Model::new(mesh_plane4x4.clone(),
                                  Vec4::new(0.0, 0.0, -5.0, 1.0),
                                  algebra::Mat4::rotate(&algebra::Mat4::identity(),
                                                        algebra::Angle::Degrees(0.0),
                                                        &algebra::Vec4::new(1.0, 0.0, 0.0, 0.0)),
                                  Vec4::new(10.0, 10.0, 10.0, 1.0));

    let camera = camera::Camera {
        position: Vec4 { x: 0.0, y: 0.0, z: 10.0, w: 1.0 },
        look_at: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        up: Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
    };

    let scene = scene::Scene {
        models: vec![
            model,
            backw,
            floor
        ],
        light_sources: vec![
            lighting::LightSource {
                position: Vec4::new(0.0, 0.0, 0.0, 1.0),
                ambient_color: color::Color { r: 20, g: 20, b: 20 },
                diffuse_color: color::Color { r: 0, g: 0, b: 0 },
                specular_color: color::Color { r: 0, g: 0, b: 0 },
            },
            lighting::LightSource {
                position: Vec4::new(3.0, -3.0, 10.0, 1.0),
                ambient_color: color::Color { r: 0, g: 0, b: 0 },
                diffuse_color: color::Color { r: 0, g: 100, b: 200 },
                specular_color: color::Color { r: 255, g: 255, b: 255 },
            },
            lighting::LightSource {
                position: Vec4::new(-3.0, 3.0, 3.0, 1.0),
                ambient_color: color::Color { r: 0, g: 0, b: 0 },
                diffuse_color: color::Color { r: 200, g: 0, b: 0 },
                specular_color: color::Color { r: 255, g: 255, b: 255 },
            }
        ],
        camera: camera
    };

    let mut pixmap = pixmap::Pixmap::new(1024, 1024);

    let ts_before = time::Instant::now();
    raytrace::render_scene(&scene, &mut pixmap);
    let ts_after = time::Instant::now();

    println!("Rendering time: {:?}", ts_after.duration_since(ts_before));

    pixmap.save_as_ppm("./output.ppm".to_string()).unwrap();
}

