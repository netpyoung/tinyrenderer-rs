use base::vec::{Vec3, Vec2};

use std;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::ops::Index;


#[derive(Clone, Copy)]
pub struct FacePoint {
    pub vindex: usize,
    pub tindex: usize,
    pub nindex: usize,
}

#[allow(dead_code)]
pub struct Face(pub FacePoint, pub FacePoint, pub FacePoint);

#[allow(dead_code)]
pub struct Model {
    verts: Vec<Vec3<f32>>,
    tex_verts: Vec<Vec3<f32>>,
    norm_verts: Vec<Vec3<f32>>,
    pub faces: Vec<Face>,
}

#[allow(dead_code)]
fn parse_point(line: &String) -> Vec3<f32> {
    let mut iter = line.split_whitespace();

    iter.next();
    let vec: Vec<f32> = iter.map(|x| x.parse().unwrap()).collect();

    Vec3::<f32> {
        x: vec[0],
        y: vec[1],
        z: vec[2],
    }
}

impl Face {
    #[allow(dead_code)]
    pub fn normal(&self, obj: &Model) -> Vec3<f32> {
        let v0 = obj.vert(self.0.vindex);
        let v1 = obj.vert(self.1.vindex);
        let v2 = obj.vert(self.2.vindex);

        (v2 - v0).cross(v1 - v0).normalized()
    }
}

impl Index<i32> for Face {
    type Output = usize;

    fn index(&self, idx: i32) -> &usize {
        match idx {
            0 => &self.0.vindex,
            1 => &self.1.vindex,
            2 => &self.2.vindex,
            _ => panic!("Invalid idx: {}", idx)
        }
    }
}



impl Model {
    pub fn new(filename: &str) -> Result<Model, std::io::Error> {
        Model::from_file(&filename)
    }
    pub fn vert(&self, i: usize) -> Vec3<f32> {
        self.verts[i - 1]
    }

    #[allow(dead_code)]
    pub fn tex_vert(&self, i: usize) -> Vec2<f32> {
        self.tex_verts[i - 1].xy()
    }

    #[allow(dead_code)]
    pub fn norm_vert(&self, i: usize) -> Vec3<f32> {
        self.norm_verts[i - 1]
    }

    #[allow(dead_code)]
    pub fn from_file(filename: &str) -> Result<Model, std::io::Error> {
        let f = File::open(filename)?;

        let mut obj = Model {
            verts: Vec::new(),
            tex_verts: Vec::new(),
            norm_verts: Vec::new(),
            faces: Vec::new(),
        };

        let file = BufReader::new(&f);
        for line in file.lines() {
            let line = line?;

            if line.starts_with("v ") {
                obj.verts.push(parse_point(&line));
            } else if line.starts_with("vt ") {
                obj.tex_verts.push(parse_point(&line));
            } else if line.starts_with("vn ") {
                obj.norm_verts.push(parse_point(&line));
            } else if line.starts_with("f") {
                let mut iter = line.split_whitespace();

                iter.next();
                let vec: Vec<&str> = iter.collect();
                let mut face_vec: Vec<FacePoint> = Vec::new();

                for face in vec {
                    let vec: Vec<usize> = face.split("/").map(|x| x.parse().unwrap()).collect();

                    face_vec.push(FacePoint {
                                      vindex: vec[0],
                                      tindex: vec[1],
                                      nindex: vec[2],
                                  });
                }

                obj.faces.push(Face(face_vec[0], face_vec[1], face_vec[2]));
            }
        }

        Ok(obj)
    }
}
