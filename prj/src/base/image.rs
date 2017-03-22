use base::color::Color;
use imagefmt;
use imagefmt::{ColFmt, ColType};


pub struct Image {
    pub data: Vec<u8>,
    #[allow(dead_code)]
    zbuffer: Vec<isize>,
    pub width: usize,
    pub height: usize,
}

pub trait IImage {
    fn new(width: usize, height: usize) -> Image;
    fn set_pixel(&mut self, x: usize, y: usize, color: &Color);
    fn set_pixel_with_depth(&mut self, x: usize, y: usize, color: &Color, depth: isize);
    fn get_pixel(&self, x: usize, y: usize) -> Color;
    fn write(&self, filename: &str) -> imagefmt::Result<()>;
    fn from(filename: &str) -> Image;
}



impl IImage for Image {
    fn set_pixel(&mut self, x: usize, y: usize, color: &Color) {
        if x < self.width && y < self.height {
            let index = ((self.height - y - 1) * self.width + x) * 3;

            self.data[index + 0] = color.r;
            self.data[index + 1] = color.g;
            self.data[index + 2] = color.b;
        }
    }

    fn set_pixel_with_depth(&mut self, x: usize, y: usize, color: &Color, depth: isize) {
        if x < self.width && y < self.height {
            let zbufferindex = (self.height - y - 1) * self.width + x;
            let index = zbufferindex * 3;

            if self.zbuffer[zbufferindex] < depth {
                self.zbuffer[zbufferindex] = depth;

                self.data[index + 0] = color.r;
                self.data[index + 1] = color.g;
                self.data[index + 2] = color.b;
            }
        }
    }

    fn get_pixel(&self, x: usize, y: usize) -> Color {
        let index = ((self.height - y - 1) * self.width + x) * 3;
        Color::new(self.data[index], self.data[index + 1], self.data[index + 2])
    }

    fn write(&self, filename: &str) -> imagefmt::Result<()> {
        imagefmt::write(filename,
                        self.width,
                        self.height,
                        ColFmt::RGB,
                        &self.data,
                        ColType::Color)
    }

    fn new(width: usize, height: usize) -> Image {
        let mut data = Vec::with_capacity(width * height * 3);
        data.resize(width * height * 3, 0);
        let mut zbuffer = Vec::with_capacity(width * height);
        zbuffer.resize(width * height, isize::min_value());

        Image {
            data: data,
            zbuffer: zbuffer,
            width: width,
            height: height,
        }
    }

    fn from(filename: &str) -> Image {
        let img = imagefmt::read(filename, ColFmt::RGB).unwrap();

        Image {
            data: img.buf,
            zbuffer: Vec::new(),
            width: img.w,
            height: img.h,
        }
    }
}
