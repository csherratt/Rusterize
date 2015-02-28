#![feature(test, old_path, core)]

extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;
extern crate test;
extern crate obj;

use rusterize::{Frame, Fragment};
use cgmath::*;
use genmesh::*;
use test::{Bencher, black_box};
use image::Rgb;

const SIZE: u32 = 1024;

struct SetValue(Rgb<u8>);

impl Fragment<[f32; 4]> for SetValue {
    type Color = Rgb<u8>;

    fn fragment(&self, _: [f32; 4]) -> Rgb<u8> { self.0 }
}

impl Fragment<([f32; 4], [f32; 3])> for SetValue {
    type Color = Rgb<u8>;

    fn fragment(&self, _: ([f32; 4], [f32; 3])) -> Rgb<u8> { self.0 }
}

#[bench]
fn plane_simple_normal(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        frame.clear();
        let plane = generators::Plane::new();
        frame.normal_raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            SetValue(Rgb([255, 255, 255]))
        );
    });
}

#[bench]
fn plane_simple_simd(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        frame.clear();
        let plane = generators::Plane::new();
        frame.simd_raster(plane.triangulate()
                         .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            SetValue(Rgb([255, 255, 255]))
        );
    });
}

#[bench]
fn plane_subdivide(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        frame.clear();
        let plane = generators::Plane::subdivide(128, 128);
        frame.simd_raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            SetValue(Rgb([255, 255, 255]))
        );
    });
}

#[bench]
fn plane_backface(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        frame.clear();
        let plane = generators::Plane::new();
        frame.simd_raster(plane.triangulate()
                          .vertex(|v| Vector4::new(-v.0, v.1, 0., 1.).into_fixed()),
            SetValue(Rgb([255, 255, 255]))
        );
    });
}

#[bench]
fn plane_front_back(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        frame.clear();
        let plane = generators::Plane::new();
        frame.simd_raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 1., 1.).into_fixed()),
            SetValue(Rgb([255, 255, 255]))
        );
        frame.simd_raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            SetValue(Rgb([128, 128, 128]))
        );
    });
}

#[bench]
fn plane_back_front(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        frame.clear();
        let plane = generators::Plane::new();
        frame.simd_raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 0., 1.).into_fixed()),
            SetValue(Rgb([255, 255, 255]))
        );
        frame.simd_raster(plane.triangulate()
                          .vertex(|v| Vector4::new(v.0, v.1, 1., 1.).into_fixed()),
            SetValue(Rgb([128, 128, 128]))
        );
    });
}

#[bench]
fn buffer_clear(bench: &mut Bencher) {
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| { frame.clear(); });
}

#[bench]
fn monkey_normal(bench: &mut Bencher) {
    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let proj = ortho(-1.5, 1.5, -1.5, 1.5, -10., 10.);
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        let vertex = monkey.indices().iter().map(|x| *x)
                           .vertex(|(p, _, n)| { (obj.position()[p], obj.normal()[n.unwrap()]) })
                           .vertex(|(p, n)| (proj.mul_v(&Vector4::new(p[0], p[1], p[2], 1.)).into_fixed(), n))
                           .triangulate();

        frame.clear();
        frame.normal_raster(vertex, SetValue(Rgb([255, 255, 255])));
    });
}

#[bench]
fn monkey_simd(bench: &mut Bencher) {
    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let proj = ortho(-1.5, 1.5, -1.5, 1.5, -10., 10.);
    let mut frame = Frame::new(SIZE, SIZE);

    bench.iter(|| {
        let vertex = monkey.indices().iter().map(|x| *x)
                           .vertex(|(p, _, n)| { (obj.position()[p], obj.normal()[n.unwrap()]) })
                           .vertex(|(p, n)| (proj.mul_v(&Vector4::new(p[0], p[1], p[2], 1.)).into_fixed(), n))
                           .triangulate();

        frame.clear();
        frame.simd_raster(vertex, SetValue(Rgb([255, 255, 255])));
    });
}

#[bench]
fn trailing_zeros(bench: &mut Bencher) {
    use std::num::Int;
    let mut i = 0u64;
    bench.iter(|| {
        black_box(if i.trailing_zeros() >= 16 { 0 } else { 1 });
        i += 1;
    });
}

#[bench]
fn barycentric_f32x4(bench: &mut Bencher) {
    use rusterize::Barycentric;

    let tri = Triangle::new(Vector4::new(0., 0., 0., 0.),
                            Vector4::new(1., 1., 0., 0.),
                            Vector4::new(0., 1., 0., 0.));

    let mut x = 0.;
    let mut y = 0.;

    let bary = Barycentric::new(tri.map_vertex(|v| Vector2::new(v.x, v.y)));

    bench.iter(|| {
        black_box(bary.coordinate_f32x4(Vector2::new(x, y), Vector2::new(7., 7.)));
        x += 1.;
        y += 1.;
    });
}

#[bench]
fn barycentric_f32x8x8(bench: &mut Bencher) {
    use rusterize::Barycentric;

    let tri = Triangle::new(Vector4::new(0., 0., 0., 0.),
                            Vector4::new(1., 1., 0., 0.),
                            Vector4::new(0., 1., 0., 0.));

    let mut x = 0.;
    let mut y = 0.;

    let bary = Barycentric::new(tri.map_vertex(|v| Vector2::new(v.x, v.y)));

    bench.iter(|| {
        black_box(bary.coordinate_f32x8x8(Vector2::new(x, y), Vector2::new(1., 1.)));
        x += 1.;
        y += 1.;
    });
}

#[bench]
fn group(bench: &mut Bencher) {
    use rusterize::Barycentric;
    use rusterize::group::Group;

    let tri = Triangle::new(Vector4::new(0., 0., 0., 0.),
                            Vector4::new(1., 1., 0., 0.),
                            Vector4::new(0., 1., 0., 0.));

    let mut x = 0.;
    let mut y = 0.;

    let bary = Barycentric::new(tri.map_vertex(|v| Vector2::new(v.x, v.y)));

    bench.iter(|| {
        black_box(Group::new(Vector2::new(x, y), &bary));
        x += 1.;
        y += 1.;
    });
}

#[bench]
fn mask_with_depth(bench: &mut Bencher) {
    use rusterize::Barycentric;
    use rusterize::group::Group;
    use rusterize::f32x8::f32x8x8;

    let tri = Triangle::new(Vector4::new(0., 0., 0., 0.),
                            Vector4::new(1., 1., 0., 0.),
                            Vector4::new(0., 1., 0., 0.));

    let mut x = 0.;
    let mut y = 0.;

    let bary = Barycentric::new(tri.map_vertex(|v| Vector2::new(v.x, v.y)));
    let group = Group::new(Vector2::new(x, y), &bary);

    bench.iter(|| {
        let mut depth = f32x8x8::broadcast(x);
        black_box(group.mask_with_depth(Vector3::new(x, y, x), &mut depth));
        x += 1.;
        y += 1.;
    });
}

#[bench]
fn full_mask(bench: &mut Bencher) {
    use rusterize::Barycentric;
    use rusterize::group::Group;
    use rusterize::f32x8::f32x8x8;

    let tri = Triangle::new(Vector4::new(0., 0., 0., 0.),
                            Vector4::new(1., 1., 0., 0.),
                            Vector4::new(0., 1., 0., 0.));

    let mut x = 0.;
    let mut y = 0.;

    let bary = Barycentric::new(tri.map_vertex(|v| Vector2::new(v.x, v.y)));
    let mut depth = f32x8x8::broadcast(x);

    bench.iter(|| {
        black_box(Group::new(Vector2::new(x, y), &bary)
                        .mask_with_depth(Vector3::new(x, y, x), &mut depth));
        x += 1.;
        y += 1.;
    });
}

