
extern crate rusterize;
extern crate genmesh;
extern crate cgmath;

use genmesh::Triangle;
use rusterize::{
    RasterTriangle,
    TriangleBottomIter,
    FlatTopIter,
    Scanline
};
use cgmath::*;


#[test]
fn top_right_angle_0() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(3., 0.),
        Vector2::new(0., 3.)
    );

    let mut top = FlatTopIter::new(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0, 3))));
    assert_eq!(top.next(), Some((1, Scanline::new(0, 2))));
    assert_eq!(top.next(), Some((2, Scanline::new(0, 1))));
    assert_eq!(top.next(), Some((3, Scanline::new(0, 0))));
}

#[test]
fn top_right_angle_1() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(3., 0.),
        Vector2::new(3., 3.)
    );

    let mut top = FlatTopIter::new(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0, 3))));
    assert_eq!(top.next(), Some((1, Scanline::new(1, 3))));
    assert_eq!(top.next(), Some((2, Scanline::new(2, 3))));
    assert_eq!(top.next(), Some((3, Scanline::new(3, 3))));
}

#[test]
fn bottom_right_angle_0() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(3., 0.),
        Vector2::new(0., 3.)
    );

    let mut top = FlatTopIter::new(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0, 3))));
    assert_eq!(top.next(), Some((1, Scanline::new(0, 2))));
    assert_eq!(top.next(), Some((2, Scanline::new(0, 1))));
    assert_eq!(top.next(), Some((3, Scanline::new(0, 0))));
}

#[test]
fn bottom_right_angle_1() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(3., 0.),
        Vector2::new(3., 3.)
    );

    let mut top = FlatTopIter::new(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0, 3))));
    assert_eq!(top.next(), Some((1, Scanline::new(1, 3))));
    assert_eq!(top.next(), Some((2, Scanline::new(2, 3))));
    assert_eq!(top.next(), Some((3, Scanline::new(3, 3))));
}