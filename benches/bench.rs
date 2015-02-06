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

const SIZE: u32 = 1024;

#[bench]
fn plane_simple(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        let plane = generators::Plane::new();
        frame.raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            |_| { Rgb([255, 255, 255]) }
        );
    });
}

#[bench]
fn plane_subdivide(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        let plane = generators::Plane::subdivide(128, 128);
        frame.raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            |_| { Rgb([255, 255, 255]) }
        );
    });
}

#[bench]
fn plane_backface(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        let plane = generators::Plane::new();
        frame.raster(plane.triangulate()
                          .vertex(|v| Vector4::new(-v.0, v.1, 0., 1.).into_fixed()),
            |_| { Rgb([255, 255, 255]) }
        );
    });
}

#[bench]
fn plane_front_back(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        let plane = generators::Plane::new();
        frame.raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 1., 1.).into_fixed()),
            |_| { Rgb([255, 255, 255]) }
        );
        frame.raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            |_| { Rgb([255, 255, 255]) }
        );
    });
}

#[bench]
fn plane_back_front(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        let plane = generators::Plane::new();
        frame.raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            |_| { Rgb([255, 255, 255]) }
        );
        frame.raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 1., 1.).into_fixed()),
            |_| { Rgb([255, 255, 255]) }
        );
    });
}

