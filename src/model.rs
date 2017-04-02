use mesh::Mesh;
use algebra::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Model {
    pub mesh: Mesh,
    pub position: Vec4,
}
