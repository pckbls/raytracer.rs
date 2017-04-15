use std::ops;
use std::f64::consts::PI;

pub enum Angle {
    Radians(f64),
    Degrees(f64)
}

#[derive(Clone,PartialEq,Debug)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self {
            x: x,
            y: y,
            z: z,
            w: w
        }
    }

    #[allow(dead_code)]
    pub fn normalize(mut self) -> Self {
        if self.w != 0.0 {
            panic!("Normalizing a Vec4 with w-component != 0.0 does not make sense.");
        }

        let length = (self.clone() * self.clone()).sqrt();
        self.x /= length;
        self.y /= length;
        self.z /= length;
        self
    }

    #[allow(dead_code)]
    pub fn invert(mut self) -> Self {
        // TODO: maybe we should check if w == 0 because inverting points makes no sense.
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w; // TODO: is this right?
        self
    }

    #[allow(dead_code)]
    pub fn cross(a: &Vec4, b: &Vec4) -> Self {
        if a.w != 0.0 || b.w != 0.0 {
            panic!("Cross product for a Vec4 with w-component != 0.0 does not make sense.");
        }

        Vec4 {
            x: a.y * b.z - b.y * a.z,
            y: a.z * b.x - b.z * a.x,
            z: a.x * b.y - b.x * a.y,
            w: 0.0
        }
    }

    #[allow(dead_code)]
    pub fn dot(a: &Vec4, b: &Vec4) -> f64 {
        a.clone() * b.clone()
    }

    #[allow(dead_code)]
    pub fn unproject(vec: Vec4, model_matrix: &Mat4, projection_matrix: &Mat4, width: u32, height: u32) -> Vec4 {
        let inverse = (projection_matrix.clone() * model_matrix.clone()).inverse();
        let mut tmp = vec.clone();

        tmp.x /= width as f64;
        tmp.y /= height as f64;

        tmp.x = tmp.x * 2.0 - 1.0;
        tmp.y = tmp.y * 2.0 - 1.0;
        tmp.z = tmp.z * 2.0 - 1.0;
        tmp.w = tmp.w * 2.0 - 1.0;

        let mut obj = inverse * tmp;
        obj = obj.clone() / obj.w;
        obj
    }

    /// TODO
    pub fn epsilon_compare(a: &Self, b: &Self, epsilon: f64) -> bool {
        if     (a.x-b.x).abs() > epsilon
            || (a.y-b.y).abs() > epsilon
            || (a.z-b.z).abs() > epsilon
            || (a.w-b.w).abs() > epsilon {
            false
        }
        else {
            true
        }
    }
}

impl ops::Add<Vec4> for Vec4 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Vec4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl ops::Sub<Vec4> for Vec4 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Vec4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl ops::Mul<Vec4> for Vec4 {
    type Output = f64;

    fn mul(self, rhs: Self) -> f64 {
        if self.w != 0.0 {
            panic!("Dot product for a Vec4 with w-component != 0.0 does not make sense.");
        }

        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl ops::Mul<Vec4> for f64 {
    type Output = Vec4;

    fn mul(self, mut rhs: Vec4) -> Vec4 {
        rhs.x *= self;
        rhs.y *= self;
        rhs.z *= self;
        rhs.w *= self;
        rhs
    }
}

impl ops::Div<f64> for Vec4 {
    type Output = Vec4;

    fn div(mut self, rhs: f64) -> Vec4 {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs; // TODO: is this correct?
        self
    }
}

#[derive(Clone,PartialEq,Debug)]
pub struct Mat4 {
    pub data: [f64; 16]
}

impl Mat4 {
    /// TODO
    pub fn new(data: [f64; 16]) -> Self {
        Self {
            data: data
        }
    }

    /// Generates a matrix filled with 0.0
    pub fn zeros() -> Self {
        Self::new([0.0; 16])
    }

    /// Generates a matrix filled with 1.0
    pub fn ones() -> Self {
        Self::new([1.0; 16])
    }

    /// TODO
    pub fn identity() -> Mat4 {
        let mut matrix = Mat4::zeros();
        matrix.data[0] = 1.0;
        matrix.data[5] = 1.0;
        matrix.data[10] = 1.0;
        matrix.data[15] = 1.0;
        matrix
    }

    /// Generates a translation matrix by filling up the last column
    /// with the given input vector.
    pub fn translate(v: &Vec4) -> Mat4 {
        let mut matrix = Mat4::identity();
        matrix.data[3] = v.x;
        matrix.data[7] = v.y;
        matrix.data[11] = v.z;
        matrix.data[15] = v.w;
        matrix
    }

    /// Transposes a matrix.
    pub fn transpose(self) -> Self {
        let mut m2 = Mat4::zeros();
        for i in 0..4 {
            for j in 0..4 {
                m2.data[i * 4 + j] = self.data[j * 4 + i];
            }
        }
        m2
    }

    /// Calculates the matrix' determinant.
    pub fn determinant(&self) -> f64 {
        // This has helped me a lot:
        // https://github.com/stackgl/gl-mat4/blob/master/determinant.js
        let a00 = self.data[0];
        let a01 = self.data[1];
        let a02 = self.data[2];
        let a03 = self.data[3];
        let a10 = self.data[4];
        let a11 = self.data[5];
        let a12 = self.data[6];
        let a13 = self.data[7];
        let a20 = self.data[8];
        let a21 = self.data[9];
        let a22 = self.data[10];
        let a23 = self.data[11];
        let a30 = self.data[12];
        let a31 = self.data[13];
        let a32 = self.data[14];
        let a33 = self.data[15];

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06
    }

    /// TODO
    pub fn adjugate(self) -> Self {
        // Again, thanks to:
        // https://github.com/stackgl/gl-mat4/blob/master/adjoint.js
        let a00 = self.data[0];
        let a01 = self.data[1];
        let a02 = self.data[2];
        let a03 = self.data[3];
        let a10 = self.data[4];
        let a11 = self.data[5];
        let a12 = self.data[6];
        let a13 = self.data[7];
        let a20 = self.data[8];
        let a21 = self.data[9];
        let a22 = self.data[10];
        let a23 = self.data[11];
        let a30 = self.data[12];
        let a31 = self.data[13];
        let a32 = self.data[14];
        let a33 = self.data[15];

        Mat4::new([
             (a11 * (a22 * a33 - a23 * a32) - a21 * (a12 * a33 - a13 * a32) + a31 * (a12 * a23 - a13 * a22)),
            -(a01 * (a22 * a33 - a23 * a32) - a21 * (a02 * a33 - a03 * a32) + a31 * (a02 * a23 - a03 * a22)),
             (a01 * (a12 * a33 - a13 * a32) - a11 * (a02 * a33 - a03 * a32) + a31 * (a02 * a13 - a03 * a12)),
            -(a01 * (a12 * a23 - a13 * a22) - a11 * (a02 * a23 - a03 * a22) + a21 * (a02 * a13 - a03 * a12)),
            -(a10 * (a22 * a33 - a23 * a32) - a20 * (a12 * a33 - a13 * a32) + a30 * (a12 * a23 - a13 * a22)),
             (a00 * (a22 * a33 - a23 * a32) - a20 * (a02 * a33 - a03 * a32) + a30 * (a02 * a23 - a03 * a22)),
            -(a00 * (a12 * a33 - a13 * a32) - a10 * (a02 * a33 - a03 * a32) + a30 * (a02 * a13 - a03 * a12)),
             (a00 * (a12 * a23 - a13 * a22) - a10 * (a02 * a23 - a03 * a22) + a20 * (a02 * a13 - a03 * a12)),
             (a10 * (a21 * a33 - a23 * a31) - a20 * (a11 * a33 - a13 * a31) + a30 * (a11 * a23 - a13 * a21)),
            -(a00 * (a21 * a33 - a23 * a31) - a20 * (a01 * a33 - a03 * a31) + a30 * (a01 * a23 - a03 * a21)),
             (a00 * (a11 * a33 - a13 * a31) - a10 * (a01 * a33 - a03 * a31) + a30 * (a01 * a13 - a03 * a11)),
            -(a00 * (a11 * a23 - a13 * a21) - a10 * (a01 * a23 - a03 * a21) + a20 * (a01 * a13 - a03 * a11)),
            -(a10 * (a21 * a32 - a22 * a31) - a20 * (a11 * a32 - a12 * a31) + a30 * (a11 * a22 - a12 * a21)),
             (a00 * (a21 * a32 - a22 * a31) - a20 * (a01 * a32 - a02 * a31) + a30 * (a01 * a22 - a02 * a21)),
            -(a00 * (a11 * a32 - a12 * a31) - a10 * (a01 * a32 - a02 * a31) + a30 * (a01 * a12 - a02 * a11)),
             (a00 * (a11 * a22 - a12 * a21) - a10 * (a01 * a22 - a02 * a21) + a20 * (a01 * a12 - a02 * a11))
        ])
    }

    /// Inverts a matrix.
    pub fn inverse(self) -> Self {
        1.0 / self.determinant() * self.adjugate()
    }

    /// TODO
    pub fn epsilon_compare(a: &Self, b: &Self, epsilon: f64) -> bool {
        let diff = a.clone() - b.clone();

        for i in 0..4 {
            for j in 0..4 {
                if diff.data[j*4+i].abs() > epsilon {
                    return false
                }
            }
        }

        true
    }

    /// TODO
    pub fn look_at(eye: &Vec4, center: &Vec4, up: &Vec4) -> Self {
        let f = (center.clone() - eye.clone()).normalize();
        let s = Vec4::cross(&f, &up).normalize();
        let u = Vec4::cross(&s, &f);

        Mat4::new([
            s.x, s.y, s.z, -(s * eye.clone()),
            u.x, u.y, u.z, -(u * eye.clone()),
            -f.x, -f.y, -f.z, (f * eye.clone()),
            0.0, 0.0, 0.0, 1.0
        ])
    }

    /// TODO
    pub fn perspective(fovy: Angle, aspect: f64, z_near: f64, z_far: f64) -> Self {
        let fovy_radians = match fovy {
            Angle::Radians(x) => x,
            Angle::Degrees(x) => x / 180.0 * PI
        };

        let tan_half_fovy = (fovy_radians / 2.0).tan();

        let mut m = Mat4::zeros();
        m.data[0] = 1.0 / (aspect * tan_half_fovy);
        m.data[5] = 1.0 / tan_half_fovy;
        m.data[14] = -1.0;
        m.data[10] = -(z_far + z_near) / (z_far - z_near);
        m.data[11] = -(2.0 * z_far * z_near) / (z_far - z_near);
        m
    }
}

impl ops::Mul<Mat4> for f64 {
    type Output = Mat4;

    fn mul(self, mut rhs: Mat4) -> Mat4 {
        for i in 0..4 {
            for j in 0..4 {
                rhs.data[j * 4 + i] *= self;
            }
        }
        rhs
    }
}

impl ops::Sub<Mat4> for Mat4 {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self {
        for i in 0..4 {
            for j in 0..4 {
                self.data[j*4+i] -= rhs.data[j*4+i];
            }
        }
        self
    }
}

impl ops::Mul<Mat4> for Mat4 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut m2 = Mat4::zeros();
        for i in 0..4 {
            for j in 0..4 {
                m2.data[j*4+i] = self.data[j*4+0] * rhs.data[0*4+i]
                               + self.data[j*4+1] * rhs.data[1*4+i]
                               + self.data[j*4+2] * rhs.data[2*4+i]
                               + self.data[j*4+3] * rhs.data[3*4+i];
            }
        }
        m2
    }
}

impl ops::Mul<Vec4> for Mat4 {
    type Output = Vec4;

    fn mul(self, rhs: Vec4) -> Vec4 {
        Vec4 {
            x: self.data[0] * rhs.x + self.data[1] * rhs.y + self.data[2] * rhs.z + self.data[3] * rhs.w,
            y: self.data[4] * rhs.x + self.data[5] * rhs.y + self.data[6] * rhs.z + self.data[7] * rhs.w,
            z: self.data[8] * rhs.x + self.data[9] * rhs.y + self.data[10] * rhs.z + self.data[11] * rhs.w,
            w: self.data[12] * rhs.x + self.data[13] * rhs.y + self.data[14] * rhs.z + self.data[15] * rhs.w,
        }
    }
}

#[test]
fn test_vec4_dot() {
    let a = Vec4::new(1.0, 2.0, 3.0, 0.0);
    let b = Vec4::new(4.0, 5.0, 6.0, 0.0);
    assert_eq!(Vec4::dot(&a, &b), 32.0);
}

#[test]
fn test_vec4_cross() {
    let a = Vec4 { x: 3.0, y: -3.0, z: 1.0, w: 0.0 };
    let b = Vec4 { x: 4.0, y: 9.0, z: 2.0, w: 0.0 };
    let res = Vec4::cross(&a, &b);
    assert_eq!(res, Vec4 { x: -15.0, y: -2.0, z: 39.0, w: 0.0 });
}

#[test]
fn test_vec4_normalize() {
    let x = Vec4 { x: 0.0, y: 3.0, z: 0.0, w: 0.0 };
    assert_eq!(x.normalize(), Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 });
}

#[test]
fn test_vec4_unproject() {
    let width: u32 = 640;
    let height: u32 = 480;
    let camera_z = 10.0;
    let eye = Vec4 { x: 0.0, y: 0.0, z: camera_z, w: 1.0 };
    let center = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    let up = Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };

    let view_matrix = Mat4::look_at(&eye, &center, &up);
    let reference_view_matrix = Mat4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, -10.0,
        0.0, 0.0, 0.0, 1.0
    ]);
    assert!(Mat4::epsilon_compare(&view_matrix, &reference_view_matrix, 1e-7f64));

    let projection_matrix = Mat4::perspective(Angle::Degrees(45.0),
                                              (width as f64) / (height as f64),
                                              camera_z / 10.0,
                                              camera_z * 10.0);
    let reference_projection_matrix = Mat4::new([
        1.810660, 0.000000, 0.000000, 0.000000,
        0.000000, 2.414214, 0.000000, 0.000000,
        0.000000, 0.000000, -1.020202, -2.020202,
        0.000000, 0.000000, -1.000000, 0.000000
    ]);
    assert!(Mat4::epsilon_compare(&projection_matrix, &reference_projection_matrix, 1e-6f64));

    let proj_view_matrix = projection_matrix.clone() * view_matrix.clone();
    let ref_proj_view_matrix = Mat4::new([
        1.810660, 0.000000, 0.000000, 0.000000,
        0.000000, 2.414214, 0.000000, 0.000000,
        0.000000, 0.000000, -1.020202, 8.181819,
        0.000000, 0.000000, -1.000000, 10.000000
    ]);
    assert!(Mat4::epsilon_compare(&proj_view_matrix, &ref_proj_view_matrix, 1e-6f64));

    let ref_inv_proj_view_matrix = Mat4::new([
        0.552285, 0.000000, -0.000000, 0.000000,
        0.000000, 0.414214, 0.000000, -0.000000,
        -0.000000, 0.000000, -4.950001, 4.050001,
        0.000000, -0.000000, -0.495000, 0.505000
    ]);
    assert!(Mat4::epsilon_compare(&proj_view_matrix.clone().inverse(), &ref_inv_proj_view_matrix, 1e-5f64));

    let ray_start = Vec4::unproject(Vec4::new(100.0, 200.0, 0.0, 1.0), &view_matrix, &projection_matrix, width, height);
    assert!(Vec4::epsilon_compare(&ray_start, &Vec4::new(-0.379696, -0.069036, 9.000000, 1.0), 1e-5f64));
}

#[test]
fn test_mat4_multiply() {
    let a = Mat4::new([
        1.0, 2.0, 1.0, 1.0,
        0.0, 1.0, 0.0, 1.0,
        2.0, 3.0, 4.0, 1.0,
        1.0, 1.0, 1.0, 1.0
    ]);

    let b = Mat4::new([
        2.0, 5.0, 1.0, 1.0,
        6.0, 7.0, 1.0, 1.0,
        1.0, 8.0, 1.0, 1.0,
        1.0, 1.0, 1.0, 1.0
    ]);

    assert!((a*b).data == [
        16.0, 28.0, 5.0, 5.0,
        7.0, 8.0, 2.0, 2.0,
        27.0, 64.0, 10.0, 10.0,
        10.0, 21.0, 4.0, 4.0
    ]);
}

#[test]
fn test_mat4_translate() {
    let matrix = Mat4::translate(&Vec4::new(0.0, -1.0, 0.0, 1.0));
    let reference_matrix = Mat4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, -1.0,
        0.0, 0.0, 1.0, 0.0,
        0.0, 0.0, 0.0, 1.0,
    ]);
    assert_eq!(matrix, reference_matrix);
}

#[test]
fn test_mat4_transpose() {
    let m = Mat4::new([
         1.0,  2.0,  3.0,  4.0,
         5.0,  6.0,  7.0,  8.0,
         9.0, 10.0, 11.0, 12.0,
        13.0, 14.0, 15.0, 16.0
    ]);
    assert!(m.clone().transpose() != m);
    assert!(m.clone().transpose().transpose() == m);
}

#[test]
fn test_mat4_determinant() {
    let m = Mat4::new([
        5.0, 0.0, 3.0, -1.0,
        3.0, 0.0, 0.0, 4.0,
        -1.0, 2.0, 4.0, -2.0,
        1.0, 0.0, 0.0, 5.0
    ]);
    assert!(m.determinant() == 66.0);
}

#[test]
fn test_mat4_inverse() {
    let m = Mat4::new([
        2.0, 5.0, 0.0, 8.0,
        1.0, 4.0, 2.0, 6.0,
        7.0, 8.0, 9.0, 3.0,
        1.0, 5.0, 7.0, 8.0
    ]);

    // Make sure that the determinante and adjugate are correct.
    assert!(m.clone().determinant() == 179.0);
    assert!(m.clone().adjugate().data == [
        172.0, -343.0, 14.0, 80.0,
        -185.0, 422.0, 12.0, -136.0,
        -1.0, -49.0, 2.0, 37.0,
        95.0, -178.0, -11.0, 65.0
    ]);

    // Check if multiplication is commutative.
    // A^(-1) * A == A * A^(-1)
    assert!(Mat4::epsilon_compare(&(m.clone().inverse() * m.clone()),
                                  &(m.clone() * m.clone().inverse()), 1e-7f64));

    // Multiplication of a matrix with its inverse matrix should yield the identity matrix.
    // A^(-1) * A == I
    assert!(Mat4::epsilon_compare(&(m.clone().inverse() * m), &Mat4::identity(), 1e-7f64));
}

#[test]
fn test_mat4_look_at() {
    let camera_z = 10.0;
    let eye = Vec4 { x: 0.0, y: 0.0, z: camera_z, w: 1.0 };
    let center = Vec4 { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    let up = Vec4 { x: 0.0, y: 1.0, z: 0.0, w: 0.0 };

    let view_matrix = Mat4::look_at(&eye, &center, &up);
    let reference_view_matrix = Mat4::new([
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 1.0, -10.0,
        0.0, 0.0, 0.0, 1.0
    ]);
    assert!(Mat4::epsilon_compare(&view_matrix, &reference_view_matrix, 1e-7f64));
}

#[test]
fn test_mat4_perspective() {
    let camera_z = 10.0;
    let width: u32 = 640;
    let height: u32 = 480;

    let projection_matrix = Mat4::perspective(Angle::Degrees(45.0),
                                              (width as f64) / (height as f64),
                                              camera_z / 10.0,
                                              camera_z * 10.0);

    let reference_projection_matrix = Mat4::new([
        1.810660, 0.000000, 0.000000, 0.000000,
        0.000000, 2.414214, 0.000000, 0.000000,
        0.000000, 0.000000, -1.020202, -2.020202,
        0.000000, 0.000000, -1.000000, 0.000000
    ]);
    assert!(Mat4::epsilon_compare(&projection_matrix, &reference_projection_matrix, 1e-6f64));
}
