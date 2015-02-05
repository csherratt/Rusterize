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

#[test]
fn test_first() {
    let mut frame = Frame::new(512, 512);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| Vector4::new(v.0, v.1, 0., 1.));

    let mut i = 0;

    let r = Vector3::new(255.0, 0.0, 0.0);
    let g = Vector3::new(0.0, 255.0, 0.0);
    let b = Vector3::new(0.0, 0.0, 255.0);


    frame.raster(cube, |x, y, z| {
        let v = r.mul_s(x) + g.mul_s(y) + b.mul_s(z);
        Rgb([v.x as u8, v.y as u8, v.z as u8])
    });

    let fout = File::create(&Path::new("first.png")).unwrap();

    //We must indicate the image's color type and what format to save as.
    let _ = image::ImageRgb8(frame.frame).save(fout, image::PNG);
}