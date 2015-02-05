extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;
extern crate test;

use rusterize::Frame;
use cgmath::*;
use genmesh::generators;
use genmesh::{Triangulate, MapToVertices};
use std::old_io::File;
use test::{Bencher, black_box};
use image::Rgb;

#[bench]
fn plane(bench: &mut Bencher) {
    let mut frame = Frame::new(1024, 1024);

    bench.iter(|| {
        let cube = generators::Plane::new();
        frame.raster(cube.triangulate()
                         .vertex(|v| Vector4::new(v.0, v.1, 0., 1.)),
            |a, b, c| { Rgb([a as u8, b as u8, c as u8]) }
        );
    });
}