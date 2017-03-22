#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[allow(dead_code)]
pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
};
#[allow(dead_code)]
pub const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
};
#[allow(dead_code)]
pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
};
#[allow(dead_code)]
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
};

pub trait IColor {
    fn multiply(self, other: &Color) -> Color;
}


impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r: r, g: g, b: b }
    }
}
impl IColor for Color {
    fn multiply(self, other: &Color) -> Color {
        Color {
            r: (self.r as usize * other.r as usize / 255) as u8,
            g: (self.g as usize * other.g as usize / 255) as u8,
            b: (self.b as usize * other.b as usize / 255) as u8,
        }
    }
}
