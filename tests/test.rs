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

fn save(name: &str, img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    // Save the image as "fractal.png"
    let fout = File::create(&Path::new(name)).unwrap();

    //We must indicate the image's color type and what format to save as.
    let _= image::ImageRgb8(img).save(fout, image::PNG);
}

#[test]
fn test_plane() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).mul_s(0.5));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    save("test_data/results/plane.png", frame.frame.clone());

    let img = image::open(&Path::new("test_data/expected/plane.png")).unwrap();
    assert!(img.raw_pixels() == frame.frame.as_slice());

}

#[test]
fn test_plane_backface() {
    let mut frame = Frame::new(16, 16);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(-v.0, v.1, 0., 1.).mul_s(0.5));

    frame.raster(cube, |_, _, _| {
        Rgb([255, 255, 255])
    });

    save("test_data/results/plane_backface.png", frame.frame.clone());

    let img = image::open(&Path::new("test_data/expected/plane_backface.png")).unwrap();
    assert!(img.raw_pixels() == frame.frame.as_slice());
}