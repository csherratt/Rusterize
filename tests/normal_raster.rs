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
use image::Rgba;
use std::old_io::File;

fn save(name: String, mut expected: Frame, result: Frame) {
    let mut valid = true;
    for (exp, rst) in expected.frame.pixels_mut().zip(result.frame.pixels()) {
        if *exp != *rst && *exp == Rgba([255, 255, 255, 255]) {
            valid = false;
            *exp = Rgba([255, 0, 0, 255])
        } else if *exp != *rst && *rst == Rgba([255, 255, 255, 255]) {
            valid = false;
            *exp = Rgba([0, 255, 0, 255])
        }
    }

    let mut fout = File::create(&Path::new("test_data/results").join(format!("{}.png", name))).unwrap();
    let _= image::ImageRgba8(expected.frame.clone()).save(&mut fout, image::PNG);
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
