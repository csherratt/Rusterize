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
use image::Rgb;

fn check(name: &str, img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    // Save the image output just incase the test fails
    let fout = File::create(&Path::new("test_data/results").join(Path::new(name))).unwrap();
    let _= image::ImageRgb8(img.clone()).save(fout, image::PNG);

    let expected = image::open(&Path::new("test_data/expected").join(Path::new(name))).unwrap();
    assert!(expected.raw_pixels() == img.as_slice());
}

#[test]
fn plane() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).mul_s(0.5));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    check("plane.png", frame.frame.clone());
}

#[test]
fn plane_backface() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(-v.0, v.1, 0., 1.).mul_s(0.5));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    check("plane_backface.png", frame.frame.clone());
}

#[test]
fn plane_fill() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 0., 1.));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    check("plane_fill.png", frame.frame.clone());
}

#[test]
fn plane_overfill() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).mul_s(100.0));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    check("plane_overfill.png", frame.frame.clone());
}

#[test]
fn plane_back_front() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 0., 1.));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 1., 1.));

    frame.raster(cube, |_, _, _| {
        Rgb([128, 128, 128])
    });

    check("plane_back_front.png", frame.frame.clone());
}

#[test]
fn plane_front_back() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 1., 1.));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 0., 1.));

    frame.raster(cube, |_, _, _| {
        Rgb([128, 128, 128])
    });

    check("plane_front_back.png", frame.frame.clone());
}