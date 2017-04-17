use algebra::*;
use color::Color;
use model::Model;
use mesh::Face;

#[allow(dead_code)]
#[derive(Clone)]
pub struct LightSource {
    pub position: Vec4,
    pub ambient_color: Color,
    pub diffuse_color: Color,
    pub specular_color: Color
}

#[allow(dead_code)]
pub enum Shading {
    Flat,
    Gourand,
    Phong
}

fn blinn_phong_reflection_intensity(light_source_position: &Vec4, position: &Vec4, normal: &Vec4) -> f64 {
    let E = position.clone() - Vec4::new(0.0, 0.0, 0.0, 1.0);
    let L = position.clone() - light_source_position.clone();

    let bisector = (E + L).normalize();

    let mut light = Vec4::dot(&bisector, &normal); // TODO: wrong normal direction?
    if light < 0.0 {
        light = 0.0;
    }

    return light;
}

pub fn apply_face_lighting(model: &Model, face: &Face, position: &Vec4, light_source: &LightSource, shading_type: Shading) -> Color {
    let mut color = Color { r: 0, g: 0, b: 0 };

    // Apply the ambient part
    color = color.clone() + light_source.ambient_color.clone();

    // Calculate face normal
    let face_normal = match shading_type {
        // TODO: The additional normalize is required because the elements of our normal matrix
        // have not been divided by the matrix's determinant.
        Shading::Flat => (model.calc_normal_matrix() * face.normal.clone().invert()).normalize(),
        _ => panic!("Shading type has not been implemented yet.")
    };

    // Apply the diffuse lighting part
    let mut intensity = Vec4::dot(&(position.clone() - light_source.position.clone()).normalize(), &face_normal);
    if intensity < 0.0 {
        intensity = 0.0;
    }
    color = color.clone() + intensity * light_source.diffuse_color.clone();

    // Apply the specular highlight
    intensity = blinn_phong_reflection_intensity(&light_source.position, &position, &face_normal);
    color = color.clone() + intensity.powf(5.0) * light_source.specular_color.clone(); // TODO: use shininess

    color
}
