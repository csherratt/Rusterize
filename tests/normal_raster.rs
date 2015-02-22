#![feature(path, io)]

extern crate rusterize;
extern crate genmesh;
extern crate cgmath;
extern crate image;
extern crate rand;

use rand::{thread_rng, Rng};
use genmesh::Triangle;
use rusterize::{
    FlatTriangleIter,
    Scanline,
    Frame,
    Fragment
};
use cgmath::*;
use image::Rgb;
use std::old_io::File;

fn save(name: String, mut expected: Frame, result: Frame) {
    let mut valid = true;
    for (exp, rst) in expected.frame.pixels_mut().zip(result.frame.pixels()) {
        if *exp != *rst && *exp == Rgb([255, 255, 255]) {
            valid = false;
            *exp = Rgb([255, 0, 0])
        } else if *exp != *rst && *rst == Rgb([255, 255, 255]) {
            valid = false;
            *exp = Rgb([0, 255, 0])
        }
    }

    let mut fout = File::create(&Path::new("test_data/results").join(format!("{}.png", name))).unwrap();
    let _= image::ImageRgb8(expected.frame.clone()).save(&mut fout, image::PNG);
    assert!(valid);
}


#[test]
fn top_right_angle_0() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(3., 0.),
        Vector2::new(0., 3.)
    );

    let mut top = FlatTriangleIter::new_top(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0., 3.))));
    assert_eq!(top.next(), Some((1, Scanline::new(0., 2.))));
    assert_eq!(top.next(), Some((2, Scanline::new(0., 1.))));
    assert_eq!(top.next(), Some((3, Scanline::new(0., 0.))));
}

#[test]
fn top_right_angle_1() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(3., 0.),
        Vector2::new(3., 3.)
    );

    let mut top = FlatTriangleIter::new_top(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0., 3.))));
    assert_eq!(top.next(), Some((1, Scanline::new(1., 3.))));
    assert_eq!(top.next(), Some((2, Scanline::new(2., 3.))));
    assert_eq!(top.next(), Some((3, Scanline::new(3., 3.))));
}

#[test]
fn bottom_right_angle_0() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(0., 3.),
        Vector2::new(3., 3.)
    );

    let mut top = FlatTriangleIter::new_bottom(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0., 0.))));
    assert_eq!(top.next(), Some((1, Scanline::new(0., 1.))));
    assert_eq!(top.next(), Some((2, Scanline::new(0., 2.))));
    assert_eq!(top.next(), Some((3, Scanline::new(0., 3.))));
}

#[test]
fn bottom_right_angle_1() {
    let triangle = Triangle::new(
        Vector2::new(3., 0.),
        Vector2::new(0., 3.),
        Vector2::new(3., 3.)
    );

    let mut top = FlatTriangleIter::new_bottom(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(3., 3.))));
    assert_eq!(top.next(), Some((1, Scanline::new(2., 3.))));
    assert_eq!(top.next(), Some((2, Scanline::new(1., 3.))));
    assert_eq!(top.next(), Some((3, Scanline::new(0., 3.))));
}

struct SetValue(Rgb<u8>);

impl Fragment<[f32; 4]> for SetValue {
    type Color = Rgb<u8>;

    fn fragment(&self, _: [f32; 4]) -> Rgb<u8> { self.0 }
}

#[test]
fn random_triangles() {
    for i in (0..100) {
        let mut expected = Frame::new(64, 64);
        let mut result = Frame::new(64, 64);
        let triangle = Some(Triangle::new(
            [thread_rng().gen_range(-1f32, 1.), thread_rng().gen_range(-1f32, 1.), 0., 1.],
            [thread_rng().gen_range(-1f32, 1.), thread_rng().gen_range(-1f32, 1.), 0., 1.],
            [thread_rng().gen_range(-1f32, 1.), thread_rng().gen_range(-1f32, 1.), 0., 1.]
        ));
        println!("{} {:?}", i, triangle);

        expected.debug_raster(triangle.iter().map(|x| *x), SetValue(Rgb([255, 255, 255])));
        result.raster(triangle.iter().map(|x| *x), SetValue(Rgb([255, 255, 255])));

        save(format!("random_{}", i), expected, result);
    }

}
