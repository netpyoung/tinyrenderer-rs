extern crate imagefmt;
extern crate rand;
//extern crate cpuprofiler;


mod base;
use base::image::{Image, IImage};
use base::color;
use base::color::Color;
use base::model::Model;
use base::vec::{Vec2, Vec2i, Vec2f, Vec3f};

use rand::Rng;
use std::mem;
use std::cmp;
use std::f32;


#[allow(dead_code)]
fn line(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: &Color) {
    // ref: https://www.rosettacode.org/wiki/Loops/For_with_a_specified_step#Rust

    let x0f = x0 as f32;
    let y0f = y0 as f32;
    let x1f = x1 as f32;
    let y1f = y1 as f32;

    let mut t: f32 = 0.0;
    while t < 1.0 {
        let x = (x0f * (1.0 - t) + x1f * t) as usize;
        let y = (y0f * (1.0 - t) + y1f * t) as usize;
        image.set_pixel(x, y, color);

        t += 0.1;
    }
}

#[allow(dead_code)]
fn line2(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: &Color) {
    let x0f = x0 as f32;
    let y0f = y0 as f32;
    let x1f = x1 as f32;
    let y1f = y1 as f32;

    let mut x: f32 = x0f;
    while x <= x1f {
        let t: f32 = (x - x0f) / (x1f - x0f);
        let y = y0f * (1.0 - t) + (y1f * t);
        image.set_pixel(x as usize, y as usize, color);

        x += 1.0;
    }
}

#[allow(dead_code)]
fn line3(x0: i32, y0: i32, x1: i32, y1: i32, image: &mut Image, color: &Color) {
    let mut is_steep = false;
    let mut x0f = x0 as f32;
    let mut y0f = y0 as f32;
    let mut x1f = x1 as f32;
    let mut y1f = y1 as f32;

    // if the line is steep, we transpose the image
    if (x0 - x1).abs() < (y0 - y1).abs() {
        mem::swap(&mut x0f, &mut y0f);
        mem::swap(&mut x1f, &mut y1f);
        is_steep = true;
    }

    // make it left−to−right
    if x0f > x1f {
        mem::swap(&mut x0f, &mut x1f);
        mem::swap(&mut y0f, &mut y1f);
    }

    let mut x: f32 = x0f;
    while x <= x1f {
        let t: f32 = (x - x0f) / (x1f - x0f);
        let y = y0f * (1.0 - t) + (y1f * t);

        if is_steep {
            image.set_pixel(y as usize, x as usize, color);
        } else {
            image.set_pixel(x as usize, y as usize, color);
        }

        x += 1.0;
    }
}

#[allow(dead_code)]
fn draw_face() {
    let width = 800;
    let height = 800;

    let mut image = Image::new(width, height);
    let model = Model::new("obj/african_head.obj").unwrap();
    for face in model.faces.iter() {
        for j in 0..3 {
            let v0 = model.vert(face[j]);
            let v1 = model.vert(face[(j + 1) % 3]);
            let x0 = ((v0.x + 1.0) * width as f32 / 2.0) as i32;
            let y0 = ((v0.y + 1.0) * height as f32 / 2.0) as i32;
            let x1 = ((v1.x + 1.0) * width as f32 / 2.0) as i32;
            let y1 = ((v1.y + 1.0) * height as f32 / 2.0) as i32;
            line3(x0, y0, x1, y1, &mut image, &color::WHITE);
        }
    }
    image.write("out.tga").unwrap();
}

/*
//PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();
//PROFILER.lock().unwrap().stop().unwrap();

//line(13, 20, 80, 40, &mut image, &color::RED);

//line2(13, 20, 80, 40, &mut image, &color::WHITE);
//line2(20, 13, 40, 80, &mut image, &color::RED);
//line2(80, 40, 13, 20, &mut image, &color::RED);


//line3(13, 20, 80, 40, &mut image, &color::WHITE);
//line3(20, 13, 40, 80, &mut image, &color::RED);
//line3(80, 40, 13, 20, &mut image, &color::RED);


//image.set_pixel(52, 41, &color::GREEN);
*/

fn line4(p0: &Vec2i, p1: &Vec2i, image: &mut Image, color: &Color) {
    line3(p0.x, p0.y, p1.x, p1.y, image, color);
}

#[allow(dead_code)]
fn triangle_1(p0: &Vec2<i32>, p1: &Vec2<i32>, p2: &Vec2<i32>, image: &mut Image, color: &Color) {
    line4(p0, p1, image, color);
    line4(p1, p2, image, color);
    line4(p2, p0, image, color);
}

#[allow(dead_code)]
#[allow(unused_variables)]
fn triangle_2(p0: &Vec2<i32>, p1: &Vec2<i32>, p2: &Vec2<i32>, image: &mut Image, color: &Color) {
    let mut v = vec![p0, p1, p2];
    v.sort();
    {
        let (p0, p1, p2) = (v[0], v[1], v[2]);
        line4(p0, p1, image, &color::GREEN);
        line4(p1, p2, image, &color::GREEN);
        line4(p2, p0, image, &color::RED);
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle_3(v0: Vec2f, v1: Vec2f, v2: Vec2f, image: &mut Image, color: &Color) {
    let mut vs = vec![v0, v1, v2];
    vs.sort_by(base::vec::cmp);
    let (v0, v1, v2) = (vs[0], vs[1], vs[2]);

    let total_height = v2.y - v0.y;
    for y in v0.y as i32..(v1.y as i32 + 1) {
        let segment_height = v1.y - v0.y + 1.0;

        let alpha = (y as f32 - v0.y) / total_height;
        let beta = (y as f32 - v0.y) / segment_height;
        let A = v0 + ((v2 - v0) * alpha);
        let B = v0 + ((v1 - v0) * beta);

        image.set_pixel(A.x as usize, y as usize, &color::RED);
        image.set_pixel(B.x as usize, y as usize, &color::GREEN);
    }
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle_4(v0: Vec2f, v1: Vec2f, v2: Vec2f, image: &mut Image, color: &Color) {
    let mut vs = vec![v0, v1, v2];
    vs.sort_by(base::vec::cmp);
    let (v0, v1, v2) = (vs[0], vs[1], vs[2]);

    let total_height = v2.y - v0.y;
    for y in v0.y as i32..(v1.y as i32 + 1) {
        let segment_height = v1.y - v0.y + 1.0;

        let alpha = (y as f32 - v0.y) / total_height;
        let beta = (y as f32 - v0.y) / segment_height;
        let mut A = v0 + ((v2 - v0) * alpha);
        let mut B = v0 + ((v1 - v0) * beta);
        if A.x > B.x {
            mem::swap(&mut A, &mut B);
        }

        for j in A.x as i32..B.x as i32 + 1 {
            image.set_pixel(j as usize, y as usize, color);
        }
    }

    for y in v1.y as i32..(v2.y as i32 + 1) {
        let segment_height = v2.y - v1.y + 1.0;

        let alpha = (y as f32 - v0.y) / total_height;
        let beta = (y as f32 - v1.y) / segment_height;
        let mut A = v0 + ((v2 - v0) * alpha);
        let mut B = v1 + ((v2 - v1) * beta);
        if A.x > B.x {
            mem::swap(&mut A, &mut B);
        }

        for j in A.x as i32..(B.x as i32 + 1) {
            image.set_pixel(j as usize, y as usize, color);
        }
    }
}


#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle_5(v0: Vec2f, v1: Vec2f, v2: Vec2f, image: &mut Image, color: &Color) {
    let mut vs = vec![v0, v1, v2];
    vs.sort_by(base::vec::cmp);
    let (v0, v1, v2) = (vs[0], vs[1], vs[2]);

    let total_height = v2.y - v0.y;

    for i in 0..total_height as i32 {
        let i: f32 = i as f32;

        let alpha = i / total_height;
        let mut A = v0 + (v2 - v0) * alpha;
        let mut B: Vec2f;

        if (i > v1.y - v0.y) || (v1.y == v0.y) {
            let segment_height = v2.y - v1.y;
            let beta = (i - (v1.y - v0.y)) / segment_height;
            B = v1 + (v2 - v1) * beta;
        } else {
            let segment_height = v1.y - v0.y;
            let beta = i / segment_height;
            B = v0 + (v1 - v0) * beta;
        }

        if A.x > B.x {
            mem::swap(&mut A, &mut B);
        }

        for j in A.x as i32..B.x as i32 + 1 {
            image.set_pixel(j as usize, (v0.y + i) as usize, color);
        }
    }
}

#[allow(non_snake_case)]
fn barycentric_i32(pts: [Vec2i; 3], P: Vec2i) -> Vec3f {
    let p0 = pts[0];
    let p1 = pts[1];
    let p2 = pts[2];

    let u = Vec3f::new((p2.x - p0.x) as f32,
                       (p1.x - p0.x) as f32,
                       (p0.x - P.x) as f32)
            .cross(Vec3f::new((p2.y - p0.y) as f32,
                              (p1.y - p0.y) as f32,
                              (p0.y - P.y) as f32));

    if u.z.abs() < 1.0 {
        Vec3f::new(-1.0, 1.0, 1.0)
    } else {
        Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

#[allow(non_snake_case)]
fn barycentric(pts: [Vec3f; 3], P: Vec3f) -> Vec3f {
    let p0 = pts[0];
    let p1 = pts[1];
    let p2 = pts[2];

    let u = Vec3f::new((p2.x - p0.x),
                       (p1.x - p0.x),
                       (p0.x - P.x))
        .cross(Vec3f::new((p2.y - p0.y),
                          (p1.y - p0.y),
                          (p0.y - P.y)));

    if u.z.abs() < 1.0 {
        Vec3f::new(-1.0, 1.0, 1.0)
    } else {
        Vec3f::new(1.0 - (u.x + u.y) / u.z, u.y / u.z, u.x / u.z)
    }
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle_old(pts: [Vec2i; 3], image: &mut Image, color: &Color) {
    let clamp = Vec2i::new(image.width as i32 - 1, image.height as i32 - 1);

    let mut bboxmin = Vec2i::new(image.width as i32 - 1, image.height as i32 - 1);
    let mut bboxmax = Vec2i::new(0, 0);
    for i in 0..3 {

        bboxmin.x = cmp::max(0, cmp::min(bboxmin.x, pts[i].x));
        bboxmax.x = cmp::min(clamp.x, cmp::max(bboxmax.x, pts[i].x));

        bboxmin.y = cmp::max(0, cmp::min(bboxmin.y, pts[i].y));
        bboxmax.y = cmp::min(clamp.y, cmp::max(bboxmax.y, pts[i].y));
    }

    let mut P: Vec2i = Vec2i::new(0, 0);
    for x in bboxmin.x..bboxmax.x + 1 {
        for y in bboxmin.y..bboxmax.y + 1 {
            P.x = x;
            P.y = y;

            let bc_screen = barycentric_i32(pts, P);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }
            image.set_pixel(P.x as usize, P.y as usize, &color);
        }
    }
}

fn flat_shading_render_1() {
    let mut rng = rand::thread_rng();

    let width = 800;
    let height = 800;

    let mut image = Image::new(width, height);
    let model = Model::new("obj/african_head.obj").unwrap();

    for face in model.faces.iter() {
        let mut screen_coords: [Vec2i; 3] = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
        for j in 0..3 {
            let world_coords = model.vert(face[j]);
            screen_coords[j as usize] =
                Vec2i::new(((world_coords.x + 1.0) * width as f32 / 2.0) as i32,
                           ((world_coords.y + 1.0) * height as f32 / 2.0) as i32);
        }

        let r = rng.gen_range(0, 255) as u8;
        let g = rng.gen_range(0, 255) as u8;
        let b = rng.gen_range(0, 255) as u8;
        triangle_old(screen_coords, &mut image, &Color::new(r, g, b));
    }
    image.write("out.tga").unwrap();
}

fn flat_shading_render() {
    let mut rng = rand::thread_rng();

    let width = 800;
    let height = 800;

    let mut image = Image::new(width, height);
    let model = Model::new("obj/african_head.obj").unwrap();

    let light_dir = Vec3f::new(0.0, 0.0, -1.0);
    for face in model.faces.iter() {
        let mut screen_coords: [Vec2i; 3] = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
        let mut world_coords: [Vec3f; 3] =
            [Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0)];

        for j in 0..3 {
            let v = model.vert(face[j]);
            screen_coords[j as usize] = Vec2i::new(((v.x + 1.0) * width as f32 / 2.0) as i32,
                                                   ((v.y + 1.0) * height as f32 / 2.0) as i32);
            world_coords[j as usize] = v;
        }

        let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]);
        let intensity = light_dir * n.normalized();
        if intensity > 0.0 {
            let c = (intensity * 255.0) as u8;
            triangle_old(screen_coords, &mut image, &Color::new(c, c, c));
        }
    }
    image.write("out.tga").unwrap();
}

fn rasterize(p0: &Vec2i,
             p1: &Vec2i,
             image: &mut Image,
             color: &Color,
             ybuffer: &mut [i32; WIDTH]) {
    let mut p0 = Vec2i::new(p0.x, p0.y);
    let mut p1 = Vec2i::new(p1.x, p1.y);

    if p0.x > p1.x {
        mem::swap(&mut p0, &mut p1);
    }

    for x in p0.x..p1.x {
        let t = (x - p0.x) as f32 / (p1.x - p0.x) as f32;
        let y = (p0.y as f32 * (1.0 - t) + p1.y as f32 * t) as i32;

        if ybuffer[x as usize] < y {
            ybuffer[x as usize] = y;
            image.set_pixel(x as usize, 0usize, color);
        }
    }
}

#[allow(non_snake_case)]
#[allow(unused_variables)]
fn triangle(pts: [Vec3f; 3], zbuffer: &mut [f32], image: &mut Image, color: &Color) {
    let clamp = Vec2f::new(image.width as f32 - 1.0, image.height as f32 - 1.0);

    let mut bboxmin = Vec2f::new(f32::MAX, f32::MAX);
    let mut bboxmax = Vec2f::new(f32::MIN, f32::MIN);
    for i in 0..3 {

        bboxmin.x = f32::max(0.0, f32::min(bboxmin.x, pts[i].x));
        bboxmax.x = f32::min(clamp.x, f32::max(bboxmax.x, pts[i].x));

        bboxmin.y = f32::max(0.0, f32::min(bboxmin.y, pts[i].y));
        bboxmax.y = f32::min(clamp.y, f32::max(bboxmax.y, pts[i].y));
    }

    let mut P = Vec3f::new(0.0, 0.0, 0.0);
    for x in bboxmin.x as i32 ..bboxmax.x as i32+ 1 {
        for y in bboxmin.y as i32 ..bboxmax.y as i32 + 1 {
            P.x = x as f32;
            P.y = y as f32;

            let bc_screen = barycentric(pts, P);
            if bc_screen.x < 0.0 || bc_screen.y < 0.0 || bc_screen.z < 0.0 {
                continue;
            }

            P.z = 0.0;
            P.z += pts[0].z * bc_screen.x;
            P.z += pts[1].z * bc_screen.y;
            P.z += pts[2].z * bc_screen.z;

            let idx = (P.x + P.y * image.width as f32) as usize;
            if zbuffer[idx] < P.z {
                zbuffer[idx] = P.z;
                image.set_pixel(P.x as usize, P.y as usize, &color);
            }

        }
    }
}

const WIDTH: usize = 800;
const HEIGHT: usize = 800;


fn world2screen(v: &Vec3f) -> Vec3f {
    Vec3f::new((v.x + 1.0) * WIDTH as f32 / 2.0 + 0.5,
               (v.y + 1.0) * HEIGHT as f32 / 2.0 + 0.5,
               v.z)
}

fn ch03() {
    let mut rng = rand::thread_rng();
    let light_dir = Vec3f::new(0.0, 0.0, -1.0);

    let mut zbuffer = [f32::MIN; WIDTH * HEIGHT];

    let mut render = Image::new(WIDTH, HEIGHT);
    let model = Model::new("obj/african_head.obj").unwrap();
    for face in model.faces.iter() {
        let mut pts = [Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0)];
        for i in 0..3 {
            let v = model.vert(face[i]);
            pts[i as usize] = world2screen(&v);
        }



        let mut screen_coords: [Vec2i; 3] = [Vec2i::new(0, 0), Vec2i::new(0, 0), Vec2i::new(0, 0)];
        let mut world_coords: [Vec3f; 3] =
            [Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0), Vec3f::new(0.0, 0.0, 0.0)];

        for j in 0..3 {
            let v = model.vert(face[j]);
            screen_coords[j as usize] = Vec2i::new(((v.x + 1.0) * WIDTH as f32 / 2.0) as i32,
                                                   ((v.y + 1.0) * HEIGHT as f32 / 2.0) as i32);
            world_coords[j as usize] = v;
        }

        let n = (world_coords[2] - world_coords[0]).cross(world_coords[1] - world_coords[0]);
        let intensity = light_dir * n.normalized();
        if intensity > 0.0 {
            let c = (intensity * 255.0) as u8;
            //triangle_old(screen_coords, &mut image, &Color::new(c, c, c));
            triangle(pts, &mut zbuffer, &mut render, &Color::new(c, c, c));
        }
        /*
        let r = rng.gen_range(0, 255) as u8;
        let g = rng.gen_range(0, 255) as u8;
        let b = rng.gen_range(0, 255) as u8;

        triangle(pts, &mut zbuffer, &mut render, &Color::new(r, g, b));
        */
    }

    render.write("out.tga").unwrap();
}

fn ch03_2() {
    //let (width, height) = (800, 16);
    let height = 16;

    let mut render = Image::new(WIDTH, height);
    let mut ybuffer = [i32::min_value(); WIDTH];
    rasterize(&Vec2i::new(20, 34),
              &Vec2i::new(744, 400),
              &mut render,
              &color::RED,
              &mut ybuffer);
    rasterize(&Vec2i::new(120, 434),
              &Vec2i::new(444, 400),
              &mut render,
              &color::GREEN,
              &mut ybuffer);
    rasterize(&Vec2i::new(330, 463),
              &Vec2i::new(594, 200),
              &mut render,
              &color::BLUE,
              &mut ybuffer);

    // 1-pixel wide image is bad for eyes, lets widen it
    for i in 0..WIDTH {
        for j in 1..16 {
            let color = render.get_pixel(i, 0);
            render.set_pixel(i, j, &color);
        }
    }

    render.write("out.tga").unwrap();
}

fn ch03_1() {
    let (width, height) = (800, 500);

    let mut image = Image::new(width, height);

    line4(&Vec2i::new(20, 34),
          &Vec2i::new(744, 400),
          &mut image,
          &color::RED);
    line4(&Vec2i::new(120, 434),
          &Vec2i::new(444, 400),
          &mut image,
          &color::GREEN);
    line4(&Vec2i::new(330, 463),
          &Vec2i::new(594, 200),
          &mut image,
          &color::BLUE);

    line4(&Vec2i::new(10, 10),
          &Vec2i::new(790, 10),
          &mut image,
          &color::WHITE);

    image.write("out.tga").unwrap();
}

fn main() {
    ch03();
    /*
    let (width, height) = (200, 200);

    let mut image = Image::new(width, height);
    //     Vec2i pts[3] = {Vec2i(10,10), Vec2i(100, 30), Vec2i(190, 160)};
    let pts = [Vec2i::new(10, 10), Vec2i::new(100, 30), Vec2i::new(190, 160)];
    triangle(pts, &mut image, &color::RED);

    /*
    let t0 = [Vec2::new(10.0, 70.0), Vec2::new(50.0, 160.0), Vec2::new(70.0, 80.0)];
    let t1 = [Vec2::new(180.0, 50.0), Vec2::new(150.0, 1.0), Vec2::new(70.0, 180.0)];
    let t2 = [Vec2::new(180.0, 150.0), Vec2::new(120.0, 160.0), Vec2::new(130.0, 180.0)];
    triangle(t0[0], t0[1], t0[2], &mut image, &color::RED);
    triangle(t1[0], t1[1], t1[2], &mut image, &color::WHITE);
    triangle(t2[0], t2[1], t2[2], &mut image, &color::GREEN);
    */
    image.write("out.tga").unwrap();
    */
}
