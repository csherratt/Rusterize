
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
    Frame
};
use cgmath::*;
use image::Rgb;


#[test]
fn top_right_angle_0() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(3., 0.),
        Vector2::new(0., 3.)
    );

    let mut top = FlatTriangleIter::new_top(triangle);
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

    let mut top = FlatTriangleIter::new_top(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0, 3))));
    assert_eq!(top.next(), Some((1, Scanline::new(1, 3))));
    assert_eq!(top.next(), Some((2, Scanline::new(2, 3))));
    assert_eq!(top.next(), Some((3, Scanline::new(3, 3))));
}

#[test]
fn bottom_right_angle_0() {
    let triangle = Triangle::new(
        Vector2::new(0., 0.),
        Vector2::new(0., 3.),
        Vector2::new(3., 3.)
    );

    let mut top = FlatTriangleIter::new_bottom(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(0, 0))));
    assert_eq!(top.next(), Some((1, Scanline::new(0, 1))));
    assert_eq!(top.next(), Some((2, Scanline::new(0, 2))));
    assert_eq!(top.next(), Some((3, Scanline::new(0, 3))));
}

#[test]
fn bottom_right_angle_1() {
    let triangle = Triangle::new(
        Vector2::new(3., 0.),
        Vector2::new(0., 3.),
        Vector2::new(3., 3.)
    );

    let mut top = FlatTriangleIter::new_bottom(triangle);
    assert_eq!(top.next(), Some((0, Scanline::new(3, 3))));
    assert_eq!(top.next(), Some((1, Scanline::new(2, 3))));
    assert_eq!(top.next(), Some((2, Scanline::new(1, 3))));
    assert_eq!(top.next(), Some((3, Scanline::new(0, 3))));
}


#[test]
fn random_triangles() {
    for _ in (0..1000) {
        let mut known = Frame::new(64, 64);
        let mut result = Frame::new(64, 64);
        let triangle = Some(Triangle::new(
            [thread_rng().gen_range(-1f32, 1.), thread_rng().gen_range(-1f32, 1.), 0., 1.],
            [thread_rng().gen_range(-1f32, 1.), thread_rng().gen_range(-1f32, 1.), 0., 1.],
            [thread_rng().gen_range(-1f32, 1.), thread_rng().gen_range(-1f32, 1.), 0., 1.]
        ));
        println!("{:?}", triangle);

        known.debug_raster(triangle.iter().map(|x| *x), |_| {
            Rgb([255, 255, 255])
        });
        result.raster(triangle.iter().map(|x| *x), |_| {
            Rgb([255, 255, 255])
        });

        assert!(result.frame.as_slice() == known.frame.as_slice());
    }

}
