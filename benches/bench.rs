#![feature(test, path)]

extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;
extern crate test;
extern crate obj;

use rusterize::Frame;
use cgmath::*;
use genmesh::generators;
use genmesh::{Triangulate, MapToVertices};
use test::Bencher;
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

#[bench]
fn monkey(bench: &mut Bencher) {
    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let proj = ortho(-1.5, 1.5, -1.5, 1.5, -10., 10.);

    let light_normal = Vector4::new(10., 10., 10., 0.).normalize();
    let kd = Vector4::new(64., 128., 64., 1.);
    let ka = Vector4::new(16., 16., 16., 1.);

    bench.iter(|| {
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
    });
}