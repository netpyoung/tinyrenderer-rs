use std::cmp;
use std::cmp::Ordering;
use std::ops::{Sub, Add, Mul, Div};

#[allow(dead_code)]
pub type Vec2i = Vec2<i32>;
#[allow(dead_code)]
pub type Vec2f = Vec2<f32>;
#[allow(dead_code)]
pub type Vec3i = Vec3<i32>;
#[allow(dead_code)]
pub type Vec3f = Vec3<f32>;

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2::<T> { x: x, y: y }
    }
}

impl<T> Vec2<T>
    where T: Copy + Ord
{
    #[allow(dead_code)]
    pub fn clamp(&self, min: &Vec2<T>, max: &Vec2<T>) -> Vec2<T> {
        Vec2::<T> {
            x: cmp::max(min.x, cmp::min(max.x, self.x)),
            y: cmp::max(min.y, cmp::min(max.y, self.y)),
        }
    }
}

pub fn cmp(a: &Vec2f, b: &Vec2f) -> Ordering {
    let ycmp = a.y.partial_cmp(&b.y).unwrap();
    match ycmp {
        Ordering::Equal => a.x.partial_cmp(&b.x).unwrap(),
        _ => ycmp,
    }
}

impl<T> Ord for Vec2<T>
    where T: Eq + PartialOrd
{
    fn cmp(&self, other: &Self) -> Ordering {
        let ycmp = self.y.partial_cmp(&other.y).unwrap();
        match ycmp {
            Ordering::Equal => self.x.partial_cmp(&other.x).unwrap(),
            _ => ycmp,
        }
    }
}


impl<T> Add<Vec2<T>> for Vec2<T>
    where T: Add<T, Output = T> + Copy
{
    type Output = Vec2<T>;

    fn add(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub<Vec2<T>> for Vec2<T>
    where T: Sub<T, Output = T> + Copy
{
    type Output = Vec2<T>;

    fn sub(self, other: Self) -> Vec2<T> {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl<T> Mul<T> for Vec2<T>
    where T: Mul<T, Output = T> + Copy
{
    type Output = Vec2<T>;

    fn mul(self, other: T) -> Vec2<T> {
        Vec2 {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

impl<T> Div<T> for Vec2<T>
    where T: Div<T, Output = T> + Copy
{
    type Output = Vec2<T>;

    fn div(self, other: T) -> Vec2<T> {
        Vec2 {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vec3<T> {
    #[allow(dead_code)]
    pub fn new(x: T, y: T, z: T) -> Self {
        Vec3::<T> { x: x, y: y, z: z }
    }
}

impl<T: Copy> Vec3<T> {
    // Swizzle
    pub fn xy(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl<T> Add<Vec3<T>> for Vec3<T>
    where T: Add<T, Output = T>
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub<Vec3<T>> for Vec3<T>
    where T: Sub<T, Output = T>
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Vec3<T> {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}


/*
impl<T> Mul<T> for Vec3<T>
    where T: Mul<T, Output = T> + Copy
{
    type Output = Vec3<T>;

    fn mul(self, other: T) -> Vec3<T> {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}
*/
impl Mul<f32> for Vec3f {
    type Output = Vec3f;

    fn mul(self, f: f32) -> Vec3f {
        Vec3f {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl Mul<Vec3f> for Vec3f {
    type Output = f32;

    fn mul(self, other: Vec3f) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl<T: Copy + Mul<T, Output = T> + Sub<T, Output = T>> Vec3<T> {
    pub fn cross(self, other: Vec3<T>) -> Vec3<T> {
        Vec3::<T> {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T: Copy + Mul<T, Output = T> + Add<T, Output = T>> Vec3<T> {
    #[allow(dead_code)]
    pub fn dot(self, other: Vec3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl Vec3<f32> {
    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(self) -> Vec3<f32> {
        let length = self.length();

        Vec3 {
            x: self.x / length,
            y: self.y / length,
            z: self.z / length,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Copy> Vec4<T> {
    #[allow(dead_code)]
    pub fn xy(&self) -> Vec2<T> {
        Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}
