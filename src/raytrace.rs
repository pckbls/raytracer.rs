use std::vec::Vec;
use scene::Scene;
use pixmap::Pixmap;
use color::Color;
use model::Model;
use algebra::{ Angle, Vec4, Mat4 };

pub struct Raytrace {
    scene: Scene,
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
    hit_position: Vec4
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
        Mat4::look_at(self.scene.camera.position.clone(),
                      self.scene.camera.look_at.clone(),
                      self.scene.camera.up.clone())
    }

    fn calc_model_matrix(&self, model: &Model) -> Mat4 {
        Mat4::translate(model.position.clone())
    }

    fn generate_primary_rays(&self, view_matrix: Mat4, projection_matrix: Mat4) -> Vec<Ray> {
        let mut rays: Vec<Ray> = Vec::new();

        for x in 0..self.pixmap.width {
            for y in 0..self.pixmap.height {
                let ray = Ray {
                    start: Vec4::unproject(Vec4::new(x as f64, y as f64, 0.0, 1.0),
                                           view_matrix.clone(), projection_matrix.clone(),
                                           self.pixmap.width, self.pixmap.height),
                    end: Vec4::unproject(Vec4::new(x as f64, y as f64, 1.0, 1.0),
                                         view_matrix.clone(), projection_matrix.clone(),
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

        // TODO:
        // We just generate some intersections here to verify the rest of the
        // code works as intended.
        for ray in rays {
            let (x, y) = ray.pixmap_coords;

            for model in self.scene.models.clone() {
                if let Some(intersection) = self.calculate_model_mesh_intersection(&model, ray.clone()) {
                    intersections.push(intersection);
                }
            }
        }

        intersections
    }

    fn calculate_model_mesh_intersection(&self, ref model: &Model, ray: Ray) -> Option<RayTriangleIntersection> {
        let mm = self.calc_model_matrix(&model);

        // TODO: read about Box
        // let closest_intersection;

        for face in model.mesh.faces.clone() {
            let v0 = Vec4::new(model.mesh.vertices[face.a].x, model.mesh.vertices[face.a].y, model.mesh.vertices[face.a].z, model.mesh.vertices[face.a].w);
            let v1 = Vec4::new(model.mesh.vertices[face.b].x, model.mesh.vertices[face.b].y, model.mesh.vertices[face.b].z, model.mesh.vertices[face.b].w);
            let v2 = Vec4::new(model.mesh.vertices[face.c].x, model.mesh.vertices[face.c].y, model.mesh.vertices[face.c].z, model.mesh.vertices[face.c].w);

            if let Some(t) = triangle_intersection(v0, v1, v2, ray.start.clone(), (ray.end.clone()-ray.start.clone()).normalize()) {
                let intersection = RayTriangleIntersection {
                    ray: ray.clone(),
                    hit_position: Vec4::new(0.0, 0.0, 0.0, 0.0) // TODO
                };
                return Some(intersection);
            }
        }

        None
    }

    pub fn run(&mut self) {
        println!("Calculate matrices");
        let projection_matrix = self.calc_projection_matrix();
        let view_matrix = self.calc_view_matrix();

        println!("Generate primary rays");
        let rays = self.generate_primary_rays(view_matrix, projection_matrix);

        println!("Calculate triangle intersections");
        let intersections = self.calculate_triangle_intersections(rays);

        println!("Render intersections");
        for intersection in intersections {
            let (x, y) = intersection.ray.pixmap_coords;
            self.pixmap.draw(x, y, Color { r: 255, g: 0, b: 0 });
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
    let p = Vec4::cross(d.clone(), e2.clone());

    // if determinant is near zero, ray lies in plane of triangle or ray is parallel to plane of triangle
    let det = Vec4::dot(e1.clone(), p.clone());
    if det > -epsilon && det < epsilon {
        return None;
    }

    // calculate invert determinant
    let inv_det = 1.0 / det;

    // calculate distance from v1 to ray origin
    let t = o.clone() - v1.clone();

    // calculate u parameter and test bound
    // and abort if the intersection lies outside of the triangle
    let u = Vec4::dot(t.clone(), p.clone()) * inv_det;
    if u < 0.0 || u > 1.0 {
        return None;
    }

    // prepare to test v parameter
    let q = Vec4::cross(t.clone(), e1.clone());

    // calculate V parameter and test bound
    let v = Vec4::dot(d.clone(), q.clone()) * inv_det;

    // the intersection lies outside of the triangle
    if v < 0.0 || u + v > 1.0 {
        return None;
    }

    // now check again if we've found an intersection and calculate the result
    let t = Vec4::dot(e2.clone(), q.clone()) * inv_det;
    if t > epsilon {
        // TODO: assign out
        return Some(t)
    }

    // no hit, no win
    None
}
