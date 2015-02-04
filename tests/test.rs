extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;

use rusterize::Frame;
use cgmath::*;
use genmesh::generators::{SphereUV};
use genmesh::{Triangulate, MapToVertices};
use std::old_io::File;

#[test]
fn test_first() {
    let mut frame = Frame::new(1024, 1024);
    let sphere = SphereUV::new(32, 32);

    frame.raster(sphere.triangulate()
                       .vertex(|v| Vector4::new(v.0, v.1, v.2, 1.).mul_s(0.5)));

    let fout = File::create(&Path::new("first.png")).unwrap();

    //We must indicate the image's color type and what format to save as.
    let _ = image::ImageRgb8(frame.frame).save(fout, image::PNG);
}