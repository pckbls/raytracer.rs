use std::vec::Vec;
use scene::Scene;
use pixmap::Pixmap;
use color::Color;
use model::Model;
use algebra::{ Angle, Vec4, Mat4 };
use mesh;

pub struct Raytrace {
    scene: Scene,
    // TODO: is it a good idea that the Raytrace struct consumes the Pixmap?
    pub pixmap: Pixmap
}

#[derive(Clone)]
struct Ray {
    start: Vec4,
    end: Vec4,
    pixmap_coords: (u32, u32)
}

struct RayTriangleIntersection {
    ray: Ray,
    model: Model, // TODO: use reference
    face: mesh::Face, // TODO: use reference
    hit_position: Vec4,
    color: Color
}

impl Raytrace {
    pub fn new(scene: Scene, pixmap: Pixmap) -> Self {
        Raytrace {
            scene: scene,
            pixmap: pixmap
        }
    }

    fn calc_projection_matrix(&self) -> Mat4 {
        Mat4::perspective(Angle::Degrees(45.0),
                          (self.pixmap.width / self.pixmap.height) as f64,
                          self.scene.camera.position.z / 10.0,
                          self.scene.camera.position.z * 10.0)
    }

    fn calc_view_matrix(&self) -> Mat4 {
        Mat4::look_at(&self.scene.camera.position,
                      &self.scene.camera.look_at,
                      &self.scene.camera.up)
    }

    fn generate_primary_rays(&self, view_matrix: Mat4, projection_matrix: Mat4) -> Vec<Ray> {
        let mut rays: Vec<Ray> = Vec::new();

        for x in 0..self.pixmap.width {
            for y in 0..self.pixmap.height {
                let ray = Ray {
                    start: Vec4::unproject(Vec4::new(x as f64, y as f64, 0.0, 1.0),
                                           &view_matrix, &projection_matrix,
                                           self.pixmap.width, self.pixmap.height),
                    end: Vec4::unproject(Vec4::new(x as f64, y as f64, 1.0, 1.0),
                                         &view_matrix, &projection_matrix,
                                         self.pixmap.width, self.pixmap.height),
                    pixmap_coords: (x, y)
                };

                rays.push(ray);
            }
        }

        rays
    }

    fn calculate_triangle_intersections(&self, rays: Vec<Ray>) -> Vec<RayTriangleIntersection> {
        let mut intersections: Vec<RayTriangleIntersection> = Vec::new();

        for ref ray in &rays {
            for ref model in &self.scene.models {
                if let Some(intersection) = self.calculate_model_mesh_intersection(&model, &ray) {
                    intersections.push(intersection);
                }
            }
        }

        intersections
    }

    fn calculate_model_mesh_intersection(&self, model: &Model, ray: &Ray) -> Option<RayTriangleIntersection> {
        let mm = model.calc_model_matrix();

        // TODO: read about Box
        // let closest_intersection;

        for ref face in &model.mesh.faces {
            // Back-face culling
            if face.normal.z < 0.0 {
                continue;
            }

            let v0 = mm.clone() * model.mesh.vertices[face.a].position.clone();
            let v1 = mm.clone() * model.mesh.vertices[face.b].position.clone();
            let v2 = mm.clone() * model.mesh.vertices[face.c].position.clone();

            let ray_direction = (ray.end.clone() - ray.start.clone()).normalize();

            if let Some(t) = triangle_intersection(v0, v1, v2, ray.start.clone(), ray_direction.clone()) {
                let intersection = RayTriangleIntersection {
                    ray: ray.clone(),
                    hit_position: ray.start.clone() + t * ray_direction.clone(),
                    face: (*face).clone(),
                    model: model.clone(),
                    color: Color { r: 0, g: 0, b: 0 }
                };
                return Some(intersection);
            }
        }

        None
    }

    fn calculate_intersection_colors(&self, intersections: &mut Vec<RayTriangleIntersection>) {
        for ref mut intersection in intersections.iter_mut() {
            let normal_matrix = intersection.model.calc_normal_matrix();

            intersection.color = Color { r: 0, g: 0, b: 0 };

            // TODO: Apply normal_matrix
            // TODO: Remove invert() ?
            let face_normal = intersection.face.normal.clone().invert().normalize();

            // Apply the diffuse lighting part
            let mut intensity = Vec4::dot(&(intersection.hit_position.clone() - Vec4::new(3.0, 3.0, 3.0, 1.0)).normalize(), &face_normal);
            if intensity < 0.0 {
                intensity = 0.0;
            }

            intersection.color.b = (255.0 * intensity) as u8;
        }
    }

    pub fn run(&mut self) {
        println!("Calculate matrices");
        let projection_matrix = self.calc_projection_matrix();
        let view_matrix = self.calc_view_matrix();

        println!("Generate primary rays");
        let rays = self.generate_primary_rays(view_matrix, projection_matrix);

        println!("Calculate triangle intersections");
        let mut intersections = self.calculate_triangle_intersections(rays);

        println!("Calculate lighting");
        self.calculate_intersection_colors(&mut intersections);

        println!("Render intersections");
        for intersection in intersections {
            let (x, y) = intersection.ray.pixmap_coords;
            self.pixmap.draw(x, y, intersection.color);
        }
    }
}

/// Implementation of the Möller-Trumbore intersection algorithm
/// Pseude code has been taken from Wikipedia and translated into Rust:
/// https://en.wikipedia.org/wiki/M%C3%B6ller%E2%80%93Trumbore_intersection_algorithm
fn triangle_intersection(v1: Vec4, v2: Vec4, v3: Vec4, o: Vec4, d: Vec4) -> Option<f64> {
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
    use mesh;

    let mesh = mesh::Mesh::try_load_from_off("./meshes/teapot.off", mesh::PolygonWinding::Clockwise).unwrap();

    let model = Model {
        mesh: mesh,
        position: Vec4 {
            x: 0.0,
            y: -1.0,
            z: 0.0,
            w: 1.0
        }
    };

    let camera = Camera {
        position: Vec4 { x: 0.0, y: 0.0, z: 10.0, w: 1.0 },
        look_at: Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 },
        up: Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 },
    };

    let scene = Scene {
        models: vec![model],
        camera: camera
    };

    let pixmap = Pixmap::new(32, 32);

    let mut raytrace = Raytrace::new(scene, pixmap);
    raytrace.run();

    let reference_pixmap = Pixmap::try_load_from_ppm("./testdata/raytrace.ppm".to_string()).unwrap();
    assert_eq!(raytrace.pixmap, reference_pixmap);
}
