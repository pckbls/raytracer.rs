// https://m4rw3r.github.io/rust-questionmark-operator

use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use algebra::Vec4;

#[allow(dead_code)]
#[derive(Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub normal: Vec4
}

/// Faces consist of exactly three vertices.
/// a, b and c contain indices for our vertices vector.
#[allow(dead_code)]
#[derive(Clone)]
pub struct Face {
    pub a: usize,
    pub b: usize,
    pub c: usize,
    pub normal: Vec4
}

#[allow(dead_code)]
pub enum PolygonWinding {
    Clockwise,
    CounterClockwise
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub faces: Vec<Face>
}

#[allow(dead_code)]
impl Mesh {
    /// Loads a mesh from an OFF file.
    /// See: http://shape.cs.princeton.edu/benchmark/documentation/off_format.html
    pub fn try_load_from_off(path: &str, polygon_winding: PolygonWinding) -> Result<Self, String> {
        // TODO: use String instead of &str for consistency with pixmap?
        // TODO: do not use unwrap() statements, use try!() instead, NOPE: use ? question mark
        // operator instead.
        // TODO: do not use panic!() statements, see above
       
        let f = File::open(path).map_err(|e| e.to_string())?;
        let f = BufReader::new(f);

        let mut num_vertices: u32 = 0;
        let mut num_faces: u32 = 0;

        let mut vertices: Vec<Vec4> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        enum FSM { OffKeyword, Header, Vertices, Faces, Accepted };
        let mut current_state = FSM::OffKeyword;

        for line in f.lines() {
            let line = line.unwrap();

            match current_state {
                FSM::OffKeyword => {
                    if line != "OFF" {
                        panic!("Cannot find OFF keyword");
                    }
                    current_state = FSM::Header;
                }
                FSM::Header => {
                    let splits: Vec<&str> = line.split_whitespace().collect();

                    if splits.len() != 3 {
                        return Err("Header has to consist of 3 elements".to_string())
                    }

                    num_vertices = splits[0].parse::<u32>().map_err(|e| e.to_string())?;
                    num_faces = splits[1].parse::<u32>().map_err(|e| e.to_string())?;
                    let num_edges: u32 = splits[2].parse::<u32>().map_err(|e| e.to_string())?;

                    if num_edges != 0 {
                        return Err("The OFF loader only supports numEdges == 0".to_string())
                    }

                    current_state = FSM::Vertices;
                }
                FSM::Vertices => {
                    let splits: Vec<&str> = line.split_whitespace().collect();

                    if splits.len() != 3 {
                        return Err("Vertices need to have exactly 3 coordinates".to_string())
                    }
                    
                    let vertex = Vec4 {
                        x: splits[0].parse::<f64>().map_err(|e| e.to_string())?,
                        y: splits[1].parse::<f64>().map_err(|e| e.to_string())?,
                        z: splits[2].parse::<f64>().map_err(|e| e.to_string())?,
                        w: 1.0
                    };

                    vertices.push(vertex);

                    if vertices.len() as u32 == num_vertices {
                        current_state = FSM::Faces;
                    }
                }
                FSM::Faces => {
                    let splits: Vec<&str> = line.split_whitespace().collect();

                    // TODO: check splits length

                    let n = splits[0].parse::<u32>().unwrap();
                    if n != (splits.len() as u32) - 1 {
                        panic!("something is wrong");
                    }
                    if n != 3 {
                        // TODO
                        panic!("Our OFF loader can only handle 3 foo bar");
                    }

                    let a: usize = splits[1].parse::<usize>().unwrap();
                    let b: usize = splits[2].parse::<usize>().unwrap();
                    let c: usize = splits[3].parse::<usize>().unwrap();

                    // TODO: check if vertex ids exists

                    let face = match polygon_winding {
                        PolygonWinding::Clockwise => Face { a: a, b: b, c: c, normal: Vec4::new(0.0, 0.0, 0.0, 0.0) },
                        PolygonWinding::CounterClockwise => Face { a: a, b: c, c: b, normal: Vec4::new(0.0, 0.0, 0.0, 0.0) },
                    };
                    faces.push(face);

                    if faces.len() as u32 == num_faces {
                        current_state = FSM::Accepted;
                    }
                }
                _ => return Err("Something bad happened.".to_string())
            }
        }

        match current_state {
            FSM::Accepted => Ok(Mesh::calculate_normals(vertices, faces)),
            _ => return Err("Something bad happened.".to_string())
        }
    }

    fn calculate_normals(vertex_positions: Vec<Vec4>, faces: Vec<Face>) -> Self {
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Face> = faces.clone();

        // TODO: Use map?
        for vertex_position in vertex_positions.clone() {
            vertices.push(Vertex {
                position: vertex_position,
                normal: Vec4::new(0.0, 0.0, 0.0, 0.0)
            });
        }

        for ref mut face in faces.iter_mut() {
            let v0 = vertex_positions[face.a].clone();
            let v1 = vertex_positions[face.b].clone();
            let v2 = vertex_positions[face.c].clone();

            let u = v1.clone() - v0.clone();
            let v = v2.clone() - v0.clone();

            let normal = Vec4::cross(&u, &v).normalize();
            face.normal = normal.clone();

            vertices[face.a].normal = vertices[face.a].normal.clone() + normal.clone();
            vertices[face.b].normal = vertices[face.b].normal.clone() + normal.clone();
            vertices[face.c].normal = vertices[face.c].normal.clone() + normal.clone();
        }

        for ref mut vertex in vertices.iter_mut() {
            vertex.normal = vertex.normal.clone().normalize();
        }

        // TODO: Wait a minute, this section is complete bullshit, isn't it?
        /*for ref mut face in faces.iter_mut() {
            let normal = vertices[face.a].normal.clone() + vertices[face.b].normal.clone() + vertices[face.c].normal.clone();
            face.normal = normal.normalize(); // TODO: correct?
        }*/

        Mesh {
            vertices: vertices,
            faces: faces
        }
    }
}

#[test]
fn test_good_mesh() {
    let mesh = Mesh::try_load_from_off("testdata/meshes/good.off", PolygonWinding::Clockwise);
    assert!(mesh.is_ok());
}

#[test]
fn test_bad_mesh() {
    let mesh = Mesh::try_load_from_off("testdata/meshes/bad_1.off", PolygonWinding::Clockwise);
    assert!(mesh.is_err());

    let mesh = Mesh::try_load_from_off("testdata/meshes/bad_2.off", PolygonWinding::Clockwise);
    assert!(mesh.is_err());

    let mesh = Mesh::try_load_from_off("testdata/meshes/bad_3.off", PolygonWinding::Clockwise);
    assert!(mesh.is_err());

    let mesh = Mesh::try_load_from_off("testdata/meshes/bad_4.off", PolygonWinding::Clockwise);
    assert!(mesh.is_err());

    let mesh = Mesh::try_load_from_off("testdata/meshes/bad_5.off", PolygonWinding::Clockwise);
    assert!(mesh.is_err());
}

#[test]
fn test_mesh_not_exists() {
    let mesh = Mesh::try_load_from_off("testdata/non_existing_mesh.off", PolygonWinding::Clockwise);
    assert!(mesh.is_err());
}

#[test]
fn test_calculate_normals() {
    let mesh = Mesh::try_load_from_off("meshes/teapot.off", PolygonWinding::Clockwise).unwrap();
    assert!(Vec4::epsilon_compare(&mesh.vertices[0].normal, &Vec4::new(0.075661, -0.997133, -0.000855, 0.0), 1e-6f64));
    assert!(Vec4::epsilon_compare(&mesh.faces[0].normal, &Vec4::new(-0.185414, -0.724806, 0.351416, 0.0), 1e-6f64));
}
