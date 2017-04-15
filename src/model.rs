use mesh::Mesh;
use algebra::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Model {
    pub mesh: Mesh,
    pub position: Vec4,
}

impl Model {
    pub fn calc_model_matrix(&self) -> Mat4 {
        Mat4::translate(&self.position)
    }

    pub fn calc_normal_matrix(&self) -> Mat4 {
        self.calc_model_matrix().inverse().transpose()
    }
}

#[test]
#[ignore]
fn test_calc_model_matrix() {
    panic!("Not implemented yet.")
}

#[test]
fn test_calc_normal_matrix() {
    use Mesh::PolygonWinding;

    let model = Model {
        position: Vec4::new(0.0, -1.0, 0.0, 1.0),
        mesh: Mesh::try_load_from_off("meshes/teapot.off", PolygonWinding::Clockwise).unwrap()
    };
    let reference_matrix = Mat4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 1.0, 0.0, 1.0 // TODO: Why is there a 1 on the 2nd column?
    ]);
    assert!(Mat4::epsilon_compare(&model.calc_normal_matrix(), &reference_matrix, 1e-6f64));
}
