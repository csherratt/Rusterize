#![feature(path, io)]

extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;

use rusterize::Frame;
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

    frame.raster(cube, |a| {
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