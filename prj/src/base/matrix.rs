use base::vec::{Vec3, Vec4};

#[derive(Debug)]
pub struct Matrix4x4<T> {
    data: [T; 4 * 4]
}

impl Matrix4x4 {
    fn identity(i: i32) -> Matrix4x4 {
        //Matrix4x4 {
        //    data:
        //}
    }
}

pub fn viewport(x: i32, y: i32, w: i32, h: i32) {

}
