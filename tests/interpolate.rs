#![feature(path, io)]

extern crate rusterize;
extern crate genmesh;

use genmesh::Triangle;
use rusterize::Interpolate;

#[test]
fn test_f32() {
    let v001 = Triangle::new(0., 0., 1.);
    let v010 = Triangle::new(0., 1., 0.);
    let v100 = Triangle::new(1., 0., 0.);

    let s001 = [0., 0., 1.];
    let s010 = [0., 1., 0.];
    let s100 = [1., 0., 0.];

    assert_eq!(Interpolate::interpolate(v001, s001), 1.);
    assert_eq!(Interpolate::interpolate(v001, s010), 0.);
    assert_eq!(Interpolate::interpolate(v001, s100), 0.);

    assert_eq!(Interpolate::interpolate(v010, s001), 0.);
    assert_eq!(Interpolate::interpolate(v010, s010), 1.);
    assert_eq!(Interpolate::interpolate(v010, s100), 0.);

    assert_eq!(Interpolate::interpolate(v100, s001), 0.);
    assert_eq!(Interpolate::interpolate(v100, s010), 0.);
    assert_eq!(Interpolate::interpolate(v100, s100), 1.);
}

#[test]
fn test_f32_2() {
    let v001 = Triangle::new([0., 0.], [0., 0.], [1., 1.]);
    let v010 = Triangle::new([0., 0.], [1., 1.], [0., 0.]);
    let v100 = Triangle::new([1., 1.], [0., 0.], [0., 0.]);

    let s001 = [0., 0., 1.];
    let s010 = [0., 1., 0.];
    let s100 = [1., 0., 0.];

    assert_eq!(Interpolate::interpolate(v001, s001), [1., 1.]);
    assert_eq!(Interpolate::interpolate(v001, s010), [0., 0.]);
    assert_eq!(Interpolate::interpolate(v001, s100), [0., 0.]);

    assert_eq!(Interpolate::interpolate(v010, s001), [0., 0.]);
    assert_eq!(Interpolate::interpolate(v010, s010), [1., 1.]);
    assert_eq!(Interpolate::interpolate(v010, s100), [0., 0.]);

    assert_eq!(Interpolate::interpolate(v100, s001), [0., 0.]);
    assert_eq!(Interpolate::interpolate(v100, s010), [0., 0.]);
    assert_eq!(Interpolate::interpolate(v100, s100), [1., 1.]);
}


#[test]
fn test_f32_3() {
    let v001 = Triangle::new([0., 0., 0.], [0., 0., 0.], [1., 1., 1.]);
    let v010 = Triangle::new([0., 0., 0.], [1., 1., 1.], [0., 0., 0.]);
    let v100 = Triangle::new([1., 1., 1.], [0., 0., 0.], [0., 0., 0.]);

    let s001 = [0., 0., 1.];
    let s010 = [0., 1., 0.];
    let s100 = [1., 0., 0.];

    assert_eq!(Interpolate::interpolate(v001, s001), [1., 1., 1.]);
    assert_eq!(Interpolate::interpolate(v001, s010), [0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v001, s100), [0., 0., 0.]);

    assert_eq!(Interpolate::interpolate(v010, s001), [0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v010, s010), [1., 1., 1.]);
    assert_eq!(Interpolate::interpolate(v010, s100), [0., 0., 0.]);

    assert_eq!(Interpolate::interpolate(v100, s001), [0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v100, s010), [0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v100, s100), [1., 1., 1.]);
}

#[test]
fn test_f32_4() {
    let v001 = Triangle::new([0., 0., 0., 0.], [0., 0., 0., 0.], [1., 1., 1., 1.]);
    let v010 = Triangle::new([0., 0., 0., 0.], [1., 1., 1., 1.], [0., 0., 0., 0.]);
    let v100 = Triangle::new([1., 1., 1., 1.], [0., 0., 0., 0.], [0., 0., 0., 0.]);

    let s001 = [0., 0., 1.];
    let s010 = [0., 1., 0.];
    let s100 = [1., 0., 0.];

    assert_eq!(Interpolate::interpolate(v001, s001), [1., 1., 1., 1.]);
    assert_eq!(Interpolate::interpolate(v001, s010), [0., 0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v001, s100), [0., 0., 0., 0.]);

    assert_eq!(Interpolate::interpolate(v010, s001), [0., 0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v010, s010), [1., 1., 1., 1.]);
    assert_eq!(Interpolate::interpolate(v010, s100), [0., 0., 0., 0.]);

    assert_eq!(Interpolate::interpolate(v100, s001), [0., 0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v100, s010), [0., 0., 0., 0.]);
    assert_eq!(Interpolate::interpolate(v100, s100), [1., 1., 1., 1.]);
}