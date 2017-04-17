use scene::Scene;
use pixmap::Pixmap;
use color::Color;
use model::Model;
use algebra::{ Angle, Vec4, Mat4 };
use mesh;
use lighting;
use camera::Camera;

#[derive(Clone)]
struct Ray {
    start: Vec4,
    end: Vec4,
    direction: Vec4
}

impl Ray {
    fn new(start: Vec4, end: Vec4) -> Self {
        Self {
            start: start.clone(),
            end: end.clone(),
            direction: (end - start).normalize()
        }
    }
}

struct RayTriangleIntersection<'a> {
    model: &'a Model,
    face: &'a mesh::Face,
    hit_position: Vec4,
    t: f64
}

/// Renders a scene onto a provided pixmap.
pub fn render_scene(scene: &Scene, pixmap: &mut Pixmap) {
    let view_matrix = calc_view_matrix(&scene.camera);
    let projection_matrix = calc_projection_matrix(&scene.camera, &pixmap);

    for x in 0..pixmap.width {
        for y in 0..pixmap.height {
            let start = Vec4::unproject(Vec4::new(x as f64, y as f64, 0.0, 1.0),
                                        &view_matrix, &projection_matrix,
                                        pixmap.width, pixmap.height);
            let end = Vec4::unproject(Vec4::new(x as f64, y as f64, 1.0, 1.0),
                                      &view_matrix, &projection_matrix,
                                      pixmap.width, pixmap.height);

            let ray = Ray::new(start, end);

            if let Some(intersection) = shoot_ray_into_scene(&ray, &scene) {
                let mut color = Color { r: 0, g: 0, b: 0 };

                for ref light_source in scene.light_sources.iter() {
                    let color_part = lighting::apply_face_lighting(&intersection.model, &intersection.face,
                                                                   &intersection.hit_position, &light_source,
                                                                   lighting::Shading::Flat);
                    color = color.clone() + color_part;
                }

                pixmap.draw(x, y, color);
            }
        }
    }
}

/// Renders a scene in debug mode:
/// * TODO
pub fn render_scene_debug(scene: &Scene, pixmap: &mut Pixmap) {
    let view_matrix = calc_view_matrix(&scene.camera);
    let projection_matrix = calc_projection_matrix(&scene.camera, &pixmap);

    for x in 0..pixmap.width {
        for y in 0..pixmap.height {
            let start = Vec4::unproject(Vec4::new(x as f64, y as f64, 0.0, 1.0),
                                        &view_matrix, &projection_matrix,
                                        pixmap.width, pixmap.height);
            let end = Vec4::unproject(Vec4::new(x as f64, y as f64, 1.0, 1.0),
                                      &view_matrix, &projection_matrix,
                                      pixmap.width, pixmap.height);

            let ray = Ray::new(start, end);

            if let Some(intersection) = shoot_ray_into_scene(&ray, &scene) {
                let mut face_normal = intersection.model.get_face_world_normal(&intersection.face);
                face_normal.w = 0.0; // TODO: URGH!
                face_normal = face_normal.normalize();

                let mut color = Color {
                    r: ((face_normal.x * 0.5 + 0.5) * 255.0) as u8,
                    g: ((face_normal.y * 0.5 + 0.5) * 255.0) as u8,
                    b: ((face_normal.z * 0.5 + 0.5) * 255.0) as u8,
                };

                pixmap.draw(x, y, color);
            }
        }
    }
}

fn calc_projection_matrix(camera: &Camera, pixmap: &Pixmap) -> Mat4 {
    Mat4::perspective(Angle::Degrees(45.0),
                      (pixmap.width / pixmap.height) as f64,
                      camera.position.z / 10.0,
                      camera.position.z * 10.0)
}

fn calc_view_matrix(camera: &Camera) -> Mat4 {
    Mat4::look_at(&camera.position,
                  &camera.look_at,
                  &camera.up)
}

/// Shoots a ray into the scene and returns the mesh triangle intersection with the model
/// closest to the ray's origin.
fn shoot_ray_into_scene<'a>(ray: &Ray, scene: &'a Scene) -> Option<RayTriangleIntersection<'a>> {
    let mut result: Option<RayTriangleIntersection> = None;
    let mut distance: f64 = 0.0; // TODO: We could simply use the t field inside result

    for ref model in &scene.models {
        if let Some(intersection) = calculate_model_mesh_intersection(&model, &ray) {
            if result.is_none() || intersection.t < distance {
                distance = intersection.t;
                result = Some(intersection);
            }
        }
    }

    result
}

/// Determines if a given ray somewhere intersects a model's mesh.
/// If so it returns information about the intersection.
fn calculate_model_mesh_intersection<'a>(model: &'a Model, ray: &Ray) -> Option<RayTriangleIntersection<'a>> {
    let mut result: Option<RayTriangleIntersection> = None;
    let mut distance: f64 = 0.0;

    for ref face in &model.mesh.faces {
        // Perform back-face culling.
        // TODO: This obviously only works in camera coordinate space...
        //if face.normal.z < 0.0 {
        //    continue;
        //}

        if let Some(t) = triangle_intersection(model.get_face_world_coords(&face), ray.start.clone(), ray.direction.clone()) {
            if result.is_none() || t < distance {
                let intersection = RayTriangleIntersection {
                    t: t,
                    hit_position: ray.start.clone() + t * ray.direction.clone(),
                    face: &face,
                    model: &model
                };
                result = Some(intersection);
                distance = t;
            }
        }
    }

    result
}

/// Implementation of the Möller-Trumbore intersection algorithm
/// Pseude code has been taken from Wikipedia and translated into Rust:
/// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
///
/// Maybe the GLM implementation is faster?
/// https://glm.g-truc.net/0.9.0/api/a00162.html#a0922c431baec628c6955011c79d39cd9
fn triangle_intersection((v1, v2, v3): (Vec4, Vec4, Vec4), o: Vec4, d: Vec4) -> Option<f64> {
    // TODO: Use global epsilon?
    let epsilon: f64 = 0.000001;

    // find vectors for two edges sharing v1
    let e1 = v2.clone() - v1.clone();
    let e2 = v3.clone() - v1.clone();

    // begin calculating determinant - also used to calculate u parameter
    let p = Vec4::cross(&d, &e2);

    // if determinant is near zero, ray lies in plane of triangle or ray is parallel to plane of triangle
    let det = Vec4::dot(&e1, &p);
    if det > -epsilon && det < epsilon {
        return None;
    }

    // calculate invert determinant
    let inv_det = 1.0 / det;

    // calculate distance from v1 to ray origin
    let t = o.clone() - v1.clone();

    // calculate u parameter and test bound
    // and abort if the intersection lies outside of the triangle
    let u = Vec4::dot(&t, &p) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    // prepare to test v parameter
    let q = Vec4::cross(&t, &e1);

    // calculate V parameter and test bound
    let v = Vec4::dot(&d, &q) * inv_det;

    // the intersection lies outside of the triangle
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // now check again if we've found an intersection and calculate the result
    let t = Vec4::dot(&e2, &q) * inv_det;
    if t > epsilon {
        // TODO: assign out
        return Some(t)
    }

    // no hit, no win
    None
}

#[test]
fn test_raytrace() {
    use camera::Camera;
    use lighting::LightSource;
    use mesh;

    let model = Model::new(mesh::Mesh::try_load_from_off("meshes/teapot.off", mesh::PolygonWinding::Clockwise).unwrap(),
                           Vec4::new(0.0, -1.0, 0.0, 1.0),
                           Mat4::identity(),
                           Vec4::new(1.0, 1.0, 1.0, 1.0));

    let camera = Camera {
        position: Vec4 { x: 0.0, y: 0.0, z: 10.0, w: 1.0 },
        look_at: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        up: Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
    };

    let ambient_light_source = LightSource {
        position: Vec4::new(0.0, 0.0, 0.0, 1.0),
        ambient_color: Color { r: 255, g: 0, b: 0 },
        diffuse_color: Color { r: 0, g: 0, b: 0 },
        specular_color: Color { r: 0, g: 0, b: 0 }
    };

    let scene = Scene {
        models: vec![model],
        light_sources: vec![ambient_light_source],
        camera: camera
    };

    let mut pixmap = Pixmap::new(32, 32);

    render_scene(&scene, &mut pixmap);

    pixmap.save_as_ppm("./testdata/output/raytrace.ppm".to_string()).unwrap();

    let reference_pixmap = Pixmap::try_load_from_ppm("./testdata/raytrace.ppm".to_string()).unwrap();
    assert_eq!(pixmap, reference_pixmap);
}
