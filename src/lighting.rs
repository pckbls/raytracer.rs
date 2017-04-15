use algebra::*;
use color::Color;

#[allow(dead_code)]
#[derive(Clone)]
pub struct LightSource {
    pub position: Vec4,
    pub ambient_color: Color,
    pub diffuse_color: Color
}
