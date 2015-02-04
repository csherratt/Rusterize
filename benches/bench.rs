extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;
extern crate test;

use rusterize::Frame;
use cgmath::*;
use genmesh::generators::{Cube};
use genmesh::{Triangulate, MapToVertices};
use std::old_io::File;
use test::{Bencher, black_box};

#[bench]
fn cube(bench: &mut Bencher) {
    let mut frame = Frame::new(1024, 1024);

    bench.iter(|| {
        let cube = Cube::new();
        frame.raster(cube.triangulate()
                         .vertex(|v| Vector4::new(v.0, v.1, v.2, 1.).mul_s(0.25)));
    });
}