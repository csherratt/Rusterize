#![feature(path, io, core)]

extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate rusterize;
extern crate obj;

use rusterize::{Frame, Flat, Fragment};
use cgmath::*;
use genmesh::generators;
use genmesh::{Triangulate, MapToVertices};
use std::old_io::File;
use image::{Rgba, Luma, ImageBuffer};

const SIZE: u32 = 64;

fn check(name: &str, frame: Frame) {
    let frame = frame.to_image();

    // Save the image output just incase the test fails
    let mut fout = File::create(&Path::new("test_data/results").join(format!("{}.frame.png", name))).unwrap();
    let _= image::ImageRgba8(frame.clone()).save(&mut fout, image::PNG);

    let expected = image::open(&Path::new("test_data/expected").join(format!("{}.frame.png", name))).unwrap();
    assert!(expected.raw_pixels() == frame.as_slice());
}

fn proj() -> Matrix4<f32> {
    ortho(-1., 1., -1., 1., -2., 2.)
}

#[derive(Clone)]
struct SetValue(Rgba<u8>);

impl Fragment<[f32; 4]> for SetValue {
    type Color = Rgba<u8>;

    fn fragment(&self, v: [f32; 4]) -> Rgba<u8> { self.0 }
}

#[test]
fn plane_simple() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 2.).mul_s(0.5)).into_fixed());

    frame.raster(cube, SetValue(Rgba([255, 255, 255, 255])));
    check("plane", frame);
}

#[test]
fn plane_backface() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(-v.0, v.1, 0., 2.).mul_s(0.5)).into_fixed());

    frame.raster(cube, SetValue(Rgba([255, 255, 255, 255])));
    check("plane_backface", frame);
}

#[test]
fn plane_fill() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube,SetValue(Rgba([255, 255, 255, 255])));
    check("plane_fill", frame);
}

#[test]
fn plane_overfill() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0 * 100., v.1 * 100., 0., 2.)).into_fixed());

    frame.raster(cube,SetValue(Rgba([255, 255, 255, 255])));
    check("plane_overfill", frame);
}

#[test]
fn plane_back_front() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube, SetValue(Rgba([255, 255, 255, 255])));

    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 1., 1.)).into_fixed());

    frame.raster(cube, SetValue(Rgba([128, 128, 128, 255])));

    check("plane_back_front", frame);
}

#[test]
fn plane_front_back() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 1., 1.)).into_fixed());

    frame.raster(cube, SetValue(Rgba([255, 255, 255, 255])));
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube, SetValue(Rgba([128, 128, 128, 255])));
    check("plane_front_back", frame);
}

#[test]
fn cube() {
    use genmesh::Triangle;

    let angle = deg(45.).to_rad();
    let rot: Quaternion<f32> = Rotation3::from_euler(angle, angle, rad(0.));
    let rot = rot.to_matrix4();

    let triangle = [
        [255.0, 0.0,   0.0],
        [0.0,   255.0, 0.0],
        [0.0,   0.0,   255.0],
        [255.0, 255.0, 0.0],
        [0.0,   255.0, 255.0],
        [255.0, 0.0,   255.0],
    ];
    let mut i = 0;

    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Cube::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&rot.mul_v(&Vector4::new(v.0 * 0.5, v.1 * 0.5, v.2 * 0.5, 1.))).into_fixed())
        .map(|p| {
            let color = triangle[i % 6];
            i += 1;
            Triangle::new(
                (p.x, Flat(color)),
                (p.y, Flat(color)),
                (p.z, Flat(color))
            )
        });

    #[derive(Clone)]
    struct V;

    impl Fragment<([f32; 4], [f32; 3])> for V {
        type Color = Rgba<u8>;

        fn fragment(&self, (_, color) : ([f32; 4], [f32; 3])) -> Rgba<u8> {
            Rgba([color[0] as u8, color[1] as u8, color[2] as u8, 255])
        }
    }

    frame.raster(cube, V);
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

    #[derive(Clone)]
    struct V;

    impl Fragment<([f32; 4], [f32; 3])> for V {
        type Color = Rgba<u8>;

        fn fragment(&self, (_, color) : ([f32; 4], [f32; 3])) -> Rgba<u8> {
            Rgba([(color[0] * 255.) as u8, (color[1] * 255.) as u8, (color[2] * 255.) as u8, 255])
        }
    }

    let mut frame = Frame::new(SIZE, SIZE);
    frame.raster(triangle.iter().map(|x| *x), V);
    check("triangle", frame);
}

#[test]
fn triangle_flat() {
    use genmesh::Triangle;

    let triangle = [Triangle::new(
        ([ -0.5, -0.5, 0., 1., ], Flat([1.0, 0.0, 0.0])),
        ([  0.5, -0.5, 0., 1., ], Flat([0.0, 1.0, 0.0])),
        ([  0.0,  0.5, 0., 1., ], Flat([0.0, 0.0, 1.0])),
    )];

    #[derive(Clone)]
    struct V;

    impl Fragment<([f32; 4], [f32; 3])> for V {
        type Color = Rgba<u8>;

        fn fragment(&self, (_, color) : ([f32; 4], [f32; 3])) -> Rgba<u8> {
            Rgba([(color[0] * 255.) as u8, (color[1] * 255.) as u8, (color[2] * 255.) as u8, 255])
        }
    }

    let mut frame = Frame::new(SIZE, SIZE);
    frame.raster(triangle.iter().map(|x| *x), V);
    check("triangle_flat", frame);
}

#[test]
fn monkey() {
    let obj = obj::load(&Path::new("test_assets/monkey.obj")).unwrap();
    let monkey = obj.object_iter().next().unwrap().group_iter().next().unwrap();

    let proj = ortho(-1.5, 1.5, -1.5, 1.5, -10., 10.);

    let light_normal = Vector4::new(10., 10., 10., 0.).normalize();
    let kd = Vector4::new(64., 128., 64., 1.);
    let ka = Vector4::new(16., 16., 16., 1.);

    let vertex = monkey.indices().iter().map(|x| *x)
                       .vertex(|(p, _, n)| { (obj.position()[p], obj.normal()[n.unwrap()]) })
                       .vertex(|(p, n)| (proj.mul_v(&Vector4::new(p[0], p[1], p[2], 1.)).into_fixed(), n))
                       .triangulate();

    #[derive(Clone)]
    struct V {
        ka: Vector4<f32>,
        kd: Vector4<f32>,
        light_normal: Vector4<f32>
    }

    impl Fragment<([f32; 4], [f32; 3])> for V {
        type Color = Rgba<u8>;

        fn fragment(&self, (_, n) : ([f32; 4], [f32; 3])) -> Rgba<u8> {
            let normal = Vector4::new(n[0], n[1], n[2], 0.);
            let v = self.kd.mul_s(self.light_normal.dot(&normal).partial_max(0.)) + self.ka;
            Rgba([v.x as u8, v.y as u8, v.z as u8, 255])
        }
    }

    let mut frame = Frame::new(SIZE, SIZE);
    frame.raster(vertex, V{ka: ka, kd: kd, light_normal: light_normal});
    check("monkey", frame);
}

#[test]
fn buffer_clear() {
    let mut frame = Frame::new(SIZE, SIZE);
    let cube = generators::Plane::new()
        .triangulate()
        .vertex(|v| proj().mul_v(&Vector4::new(v.0, v.1, 0., 1.)).into_fixed());

    frame.raster(cube, SetValue(Rgba([255, 255, 255, 255])));
    frame.clear();
    check("buffer_clear", frame);
}
