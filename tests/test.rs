#![feature(path, io, core)]

extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;
extern crate obj;

use rusterize::{Frame, Flat};
use cgmath::*;
use genmesh::generators;
use genmesh::{Triangulate, MapToVertices};
use std::old_io::File;
use image::{Rgb, Luma, ImageBuffer};

const SIZE: u32 = 64;

fn check(name: &str, frame: Frame) {
    // Save the image output just incase the test fails
    let fout = File::create(&Path::new("test_data/results").join(format!("{}.frame.png", name))).unwrap();
    let _= image::ImageRgb8(frame.frame.clone()).save(fout, image::PNG);

    let (width, height) = frame.depth.dimensions();
    let mut out = ImageBuffer::new(width, height);
    for y in (0..height) {
        for x in (0..width) {
            let &Luma([p]) = frame.depth.get_pixel(x, y);
            out.put_pixel(x, y, Luma([(p * 255.) as u8]));
        }
    }

    let fout = File::create(&Path::new("test_data/results").join(format!("{}.depth.png", name))).unwrap();
    let _= image::ImageLuma8(out).save(fout, image::PNG);

    let expected = image::open(&Path::new("test_data/expected").join(format!("{}.frame.png", name))).unwrap();
    assert!(expected.raw_pixels() == frame.frame.as_slice());
}

fn proj() -> Matrix4<f32> {
    ortho(-1., 1., -1., 1., -100., 10.)
}

#[test]
fn plane_simple() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 2.).mul_s(0.5)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([255, 255, 255])
    });

    check("plane", frame);
}

#[test]
fn plane_backface() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(-v.0, v.1, 0., 2.).mul_s(0.5)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([255, 255, 255])
    });

    check("plane_backface", frame);
}

#[test]
fn plane_fill() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([255, 255, 255])
    });

    check("plane_fill", frame);
}

#[test]
fn plane_overfill() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0 * 100., v.1 * 100., 0., 2.)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([255, 255, 255])
    });

    check("plane_overfill", frame);
}

#[test]
fn plane_back_front() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([255, 255, 255])
    });

    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 1., 1.)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([128, 128, 128])
    });

    check("plane_back_front", frame);
}

#[test]
fn plane_front_back() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 1., 1.)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([255, 255, 255])
    });

    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([128, 128, 128])
    });

    check("plane_front_back", frame);
}

#[test]
fn cube() {
    let angle = deg(45.).to_rad();
    let rot: Quaternion<f32> = Rotation3::from_euler(angle, angle, rad(0.));
    let rot = rot.to_matrix4();

    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Cube::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&rot.mul_v(&Vector4::new(v.0 * 0.5, v.1 * 0.5, v.2 * 0.5, 1.))).into_fixed());

    frame.raster(cube, |_| {
        Rgb([64, 64, 64])
    });

    check("cube", frame);
}

#[test]
fn triangle() {
    use genmesh::Triangle;

    let triangle = [Triangle::new(
        ([ -0.5, -0.5, 0., 1., ], [1.0, 0.0, 0.0]),
        ([  0.5, -0.5, 0., 1., ], [0.0, 1.0, 0.0]),
        ([  0.0,  0.5, 0., 1., ], [0.0, 0.0, 1.0]),
    )];

    let mut frame = Frame::new(SIZE, SIZE);
    frame.raster(triangle.iter().map(|x| *x), |(_, color)| {
        Rgb([(color[0] * 255.) as u8, (color[1] * 255.) as u8, (color[2] * 255.) as u8])
    });

    check("triangle", frame);
}

#[test]
fn triangle_flat() {
    use genmesh::Triangle;

    let triangle = [Triangle::new(
        ([ -0.5, -0.5, 0., 1., ], Flat([1.0, 0.0, 0.0])),
        ([  0.5, -0.5, 0., 1., ], Flat([0.0, 1.0, 0.0])),
        ([  0.0,  0.5, 0., 1., ], Flat([0.0, 0.0, 1.0])),
    )];

    let mut frame = Frame::new(SIZE, SIZE);
    frame.raster(triangle.iter().map(|x| *x), |(_, color)| {
        Rgb([(color[0] * 255.) as u8, (color[1] * 255.) as u8, (color[2] * 255.) as u8])
    });

    check("triangle_flat", frame);
}

#[test]
fn monkey() {
    use genmesh::Triangle;
    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let proj = ortho(-1.5, 1.5, -1.5, 1.5, -10., 10.);

    let light_normal = Vector4::new(10., 10., 10., 0.).normalize();
    let kd = Vector4::new(64., 128., 64., 1.);
    let ka = Vector4::new(16., 16., 16., 1.);

    let vertex = monkey.indices().iter().map(|x| *x)
                       .vertex(|(p, _, n)| { (obj.position()[p], obj.normal()[n.unwrap()]) })
                       .vertex(|(p, n)| (proj.mul_v(&Vector4::new(p[0], p[1], p[2], 1.)).into_fixed(), n))
                       .triangulate();

    let mut frame = Frame::new(SIZE, SIZE);
    frame.raster(vertex, |(_, n)| {
        let normal = Vector4::new(n[0], n[1], n[2], 0.);
        let v = kd.mul_s(light_normal.dot(&normal).partial_max(0.))  + ka;
        Rgb([v.x as u8, v.y as u8, v.z as u8])
    });
    check("monkey", frame);
}

#[test]
fn buffer_clear() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube, |_| {
        Rgb([255, 255, 255])
    });
    frame.clear();
    check("buffer_clear", frame);
}