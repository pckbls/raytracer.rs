use algebra::*;
use color::Color;
use model::Model;
use mesh::Face;

#[allow(dead_code)]
#[derive(Clone)]
pub struct LightSource {
    pub position: Vec4,
    pub ambient_color: Color,
    pub diffuse_color: Color
}

#[allow(dead_code)]
pub enum Shading {
    Flat,
    Gourand,
    Phong
}

pub fn apply_face_lighting(model: &Model, face: &Face, position: &Vec4, light_source: &LightSource, shading_type: Shading) -> Color {
    // TODO: Apply normal_matrix

    let mut color = Color { r: 0, g: 0, b: 0 };

    // Apply the ambient part
    color = color.clone() + light_source.ambient_color.clone();

    // TODO
    let foo = model.calc_normal_matrix() * face.normal.clone().invert();

    // Apply the diffuse lighting part
    let mut intensity = Vec4::dot(&(position.clone() - light_source.position.clone()).normalize(), &foo);
    if intensity < 0.0 {
        intensity = 0.0;
    }
    color = color.clone() + intensity * light_source.diffuse_color.clone();

    // Apply the specular highlight
    // TODO

    color
}
