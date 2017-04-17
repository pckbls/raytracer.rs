use mesh::{ Mesh, Face };
use algebra::*;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Model {
    pub mesh: Mesh,
    pub position: Vec4,
    pub orientation: Mat4, // TODO: use dedicated data type for this
    pub scale: Vec4,

    model_matrix: Mat4,
    normal_matrix: Mat4
}

// TODO: We should add another (private) struct namend `PreCalculatedModel` or something similar
// That can be constructed out of a `Model` and contains the pre-calculated matrices.

impl Model {
    pub fn new(mesh: Mesh, position: Vec4, orientation: Mat4, scale: Vec4) -> Self {
        let model_matrix = Mat4::translate(&position) * orientation.clone() * Mat4::scale(&scale);

        // TODO: This is wrong, we have to apply inverse/transpose on the top-left 3x3 sub matrix
        // See: http://stackoverflow.com/questions/27600045/the-correct-way-to-calculate-normal-matrix
        let normal_matrix = model_matrix.clone().inverse().transpose();

        let r0 = Vec4 {
            x: model_matrix.data[0],
            y: model_matrix.data[1],
            z: model_matrix.data[2],
            w: 0.0
        };
        let r1 = Vec4 {
            x: model_matrix.data[4],
            y: model_matrix.data[5],
            z: model_matrix.data[6],
            w: 0.0
        };
        let r2 = Vec4 {
            x: model_matrix.data[8],
            y: model_matrix.data[9],
            z: model_matrix.data[10],
            w: 0.0
        };

        let t0 = Vec4::cross(&r1, &r2);
        let t1 = Vec4::cross(&r2, &r0);
        let t2 = Vec4::cross(&r0, &r1);

        let normal_matrix = Mat4::new([
            t0.x, t0.y, t0.z, 0.0,
            t1.x, t1.y, t1.z, 0.0,
            t2.x, t2.y, t2.z, 0.0,
            0.0, 0.0, 0.0, 0.0
        ]);

        Self {
            mesh: mesh,
            position: position.clone(),
            orientation: orientation,
            scale: scale,

            model_matrix: model_matrix,
            normal_matrix: normal_matrix
        }
    }

    // TODO: rename this bitch
    pub fn calc_model_matrix(&self) -> Mat4 {
        self.model_matrix.clone()
    }

    // TODO: rename this bitch
    pub fn calc_normal_matrix(&self) -> Mat4 {
        self.normal_matrix.clone()
    }

    pub fn get_face_world_coords(&self, face: &Face) -> (Vec4, Vec4, Vec4) {
        let a = self.calc_model_matrix() * self.mesh.vertices[face.a].position.clone();
        let b = self.calc_model_matrix() * self.mesh.vertices[face.b].position.clone();
        let c = self.calc_model_matrix() * self.mesh.vertices[face.c].position.clone();
        (a, b, c)
    }

    pub fn get_face_world_normal(&self, face: &Face) -> Vec4 {
        self.calc_normal_matrix() * face.normal.clone()
    }
}

#[test]
#[ignore]
fn test_calc_model_matrix() {
    panic!("Not implemented yet.")
}

#[test]
fn test_calc_normal_matrix() {
    use mesh::PolygonWinding;

    let model = Model::new(Mesh::try_load_from_off("meshes/teapot.off", PolygonWinding::Clockwise).unwrap(),
                           Vec4::new(0.0, -1.0, 0.0, 1.0),
                           Mat4::identity(),
                           Vec4::new(1.0, 1.0, 1.0, 0.0));

    println!("Debug {:?}", model.calc_normal_matrix());

    let reference_matrix = Mat4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 1.0, 0.0, 1.0 // TODO: Why is there a 1 on the 2nd column? Because we are doing the calculation wrong. :)
    ]);
    assert!(Mat4::epsilon_compare(&model.calc_normal_matrix(), &reference_matrix, 1e-6f64));
}
