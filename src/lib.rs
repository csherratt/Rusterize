#![feature(unboxed_closures)]

extern crate image;
extern crate genmesh;
extern crate cgmath;

use image::{GenericImage, ImageBuffer, Rgb, Luma};
use cgmath::*;
use genmesh::{Triangle, MapVertex};
use std::num::Float;
use std::ops::Range;

pub use pipeline::{Fragment, Vertex};
pub use interpolate::{Flat, Interpolate};

mod interpolate;
mod pipeline;
mod f32x16;
pub mod group;

#[cfg(dump)]
fn dump(idx: usize, frame: &Frame) {
    use std::old_io::File;
    // Save the image output just incase the test fails
    let mut fout = File::create(&Path::new("dump").join(format!("{:05}.png", idx))).unwrap();
    let _= image::ImageRgb8(frame.frame.clone()).save(&mut fout, image::PNG);
}

#[derive(Clone)]
pub struct Frame {
    pub frame: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub depth: ImageBuffer<Luma<f32>, Vec<f32>>
}

#[inline]
fn is_backface(v: Triangle<Vector3<f32>>)-> bool {
    let e0 = v.z - v.x;
    let e1 = v.z - v.y;
    let normal = e1.cross(&e0);
    Vector3::new(0., 0., 1.).dot(&normal) >= 0.
}

#[inline]
fn sort_vertex_y(v: &mut Triangle<Vector2<f32>>) {
    use std::mem::swap;

    if v.x.y >= v.y.y {
        swap(&mut v.x, &mut v.y);
    }
    if v.y.y >= v.z.y {
        swap(&mut v.y, &mut v.z);
    }
    if v.x.y >= v.y.y {
        swap(&mut v.x, &mut v.y);
    }
}

/// described a scanline
#[derive(Debug, Copy, PartialEq)]
pub struct Scanline {
    pub start: f32,
    pub end: f32,
}

pub type ScanlineIter = std::ops::Range<i32>;

impl Scanline {
    /// create a new scanline between two points, a & b
    /// this is inclusive, so if a == b one point is valid
    #[inline]
    pub fn new(a: f32, b: f32) -> Scanline {
        if a > b {
            Scanline { start: b, end: a }
        } else {
            Scanline { start: a, end: b }
        }
    }

    /// limit, limit a scanline between to points
    /// min and max are inclusive. If a screen buffer is 64 pixels wide
    /// you will need to call `limit(0, 63)` for example.
    #[inline]
    pub fn limit_iter(self, min: i32, max: i32) -> ScanlineIter {
        let mut start = self.start.ceil() as i32;
        let mut end = self.end.floor() as i32 + 1;
        if start < min {
            start = min;
        } else if start > max {
            start = max;
        }
        if end < min {
            end = min;
        } else if end > max {
            end = max;
        }
        start..end
    }

    /// create an ScanlineIter from 
    pub fn iter(self) -> ScanlineIter {
        (self.start.ceil() as i32)..(self.end.floor() as i32 +1)
    }
}

#[derive(Debug)]
pub struct RasterTriangle {
    bottom: FlatTriangleIter,
    top: FlatTriangleIter
}

impl RasterTriangle {
    pub fn new(mut a: Triangle<Vector2<f32>>) -> RasterTriangle {
        sort_vertex_y(&mut a);
        let v = Vector2::new(a.x.x + ((a.y.y - a.x.y) / (a.z.y - a.x.y)) * (a.z.x - a.x.x),
                             a.y.y);
   
        RasterTriangle {
            bottom: FlatTriangleIter::new_bottom(Triangle::new(a.x, a.y, v)),
            top: FlatTriangleIter::new_top(Triangle::new(a.y, v, a.z)),
        }
    }
}

impl Iterator for RasterTriangle {
    type Item = (i32, Scanline);

    fn next(&mut self) -> Option<(i32, Scanline)> {
        if let Some(x) = self.bottom.next() {
            return Some(x);
        }
        self.top.next()
    }
}

#[derive(Debug)]
pub struct FlatTriangleIter {
    range: ScanlineIter,
    slope: [f32; 2],
    cursor: [f32; 2],
    base: [f32; 2]
}

impl FlatTriangleIter {
    pub fn new_bottom(mut a: Triangle<Vector2<f32>>) -> FlatTriangleIter {
        use std::mem::swap;

        if a.y.x > a.z.x {
            swap(&mut a.y, &mut a.z);
        }

        FlatTriangleIter {
            range: Scanline::new(a.x.y, a.z.y).iter(),
            slope: [(a.x.x - a.y.x) / (a.x.y - a.y.y),
                    (a.x.x - a.z.x) / (a.x.y - a.z.y)],
            cursor: [a.x.x, a.x.x],
            base: [a.x.y, a.x.y]
        }
    }

    pub fn new_top(mut a: Triangle<Vector2<f32>>) -> FlatTriangleIter {
        use std::mem::swap;

        if a.x.x > a.y.x {
            swap(&mut a.x, &mut a.y);
        }

        FlatTriangleIter {
            range: Scanline::new(a.x.y, a.z.y).iter(),
            slope: [(a.z.x - a.x.x) / (a.z.y - a.x.y),
                    (a.z.x - a.y.x) / (a.z.y - a.y.y)],
            cursor: [a.x.x, a.y.x],
            base: [a.y.y, a.y.y]
        }
    }
}

impl Iterator for FlatTriangleIter {
    type Item = (i32, Scanline);
    fn next(&mut self) -> Option<(i32, Scanline)> {
        self.range.next().map(|y| {
            let yf = y as f32;
            let s = (yf - self.base[0]) * self.slope[0] + self.cursor[0];
            let e = (yf - self.base[1]) * self.slope[1] + self.cursor[1];
            (y, Scanline::new(s, e))
        })
    }
}

#[derive(Debug)]
pub struct Barycentric {
    pub v0: Vector2<f32>,
    pub v1: Vector2<f32>,
    pub base: Vector2<f32>,
    inv_denom: f32
}

#[derive(Debug)]
pub struct BarycentricCoordinate {
    pub u: f32,
    pub v: f32,
}

impl BarycentricCoordinate {
    /// check if the point is inside the triangle
    #[inline]
    pub fn inside(&self) -> bool {
        (self.u >= 0.) && (self.v >= 0.) && ((self.u + self.v) <= 1.)
    }

    #[inline]
    pub fn weights(&self) -> [f32; 3] {
        [1. - self.u - self.v, self.u, self.v]
    }
}

impl Barycentric {
    pub fn new(t: Triangle<Vector2<f32>>) -> Barycentric {
        let v0 = t.y - t.x;
        let v1 = t.z - t.x;

        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);

        let inv_denom = 1. / (d00 * d11 - d01 * d01);

        Barycentric {
            v0: v0,
            v1: v1,
            base: t.x,
            inv_denom: inv_denom
        }
    }

    #[inline]
    pub fn coordinate(&self, p: Vector2<f32>) -> BarycentricCoordinate {
        let v2 = p - self.base;

        let d00 = self.v0.dot(&self.v0);
        let d01 = self.v0.dot(&self.v1);
        let d02 = self.v0.dot(&v2);
        let d11 = self.v1.dot(&self.v1);
        let d12 = self.v1.dot(&v2);

        let u = (d11 * d02 - d01 * d12) * self.inv_denom;
        let v = (d00 * d12 - d01 * d02) * self.inv_denom;

        BarycentricCoordinate {
            u: u,
            v: v
        }
    }

    #[inline]
    pub fn coordinate_f32x16(&self, p: Vector2<f32>) -> [f32x16::f32x16; 2] {
        use f32x16::{f32x16, f32x16_vec2};
        let v2 = p - self.base;

        let v0 = f32x16_vec2::broadcast(self.v0);
        let v1 = f32x16_vec2::broadcast(self.v1);
        let v2 = f32x16_vec2::range(p.x, p.y);

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d02 = v0.dot(v2);
        let d11 = v1.dot(v1);
        let d12 = v1.dot(v2);

        let inv_denom = f32x16::broadcast(self.inv_denom);

        [(d11 * d02 - d01 * d12) * inv_denom,
         (d00 * d12 - d01 * d02) * inv_denom]

    }
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Frame {
        Frame {
            frame: ImageBuffer::new(width, height),
            depth: ImageBuffer::from_pixel(width, height, Luma([1.]))
        }
    }

    pub fn clear(&mut self) {
        for pixel in self.frame.as_mut_slice().iter_mut() {
            *pixel = 0;
        }
        for depth in self.depth.as_mut_slice().iter_mut() {
            *depth = 1.;
        }
    }

    pub fn normal_raster<S, F, T, O>(&mut self, poly: S, fragment: F)
        where S: Iterator<Item=Triangle<T>>,
              T: Clone + Interpolate<Out=O> + FetchPosition,
              F: Fragment<O, Color=Rgb<u8>> {

        let h = self.frame.height();
        let w = self.frame.width();
        let (hf, wf) = (h as f32, w as f32);
        let (hh, wh) = (hf/2., wf/2.);
        for or in poly {
            let t = or.clone().map_vertex(|v| {
                let v = v.position();
                Vector4::new(v[0], v[1], v[2], v[3])
            });

            let clip4 = t.map_vertex(|v| {
                Vector4::new(
                    hh * (v.x / v.w) + hh,
                    wh * (v.y / v.w) + wh,
                    v.z / v.w,
                    v.w / v.w
                )
            });

            // cull any backface triangles
            if is_backface(clip4.map_vertex(|v| Vector3::new(v.x, v.y, v.z))) {
                continue;
            }

            let clip = clip4.map_vertex(|v| Vector2::new(v.x, v.y));
            let bary = Barycentric::new(clip);

            for (y, line) in RasterTriangle::new(clip) {
                let y = y as u32;
                if y >= h { continue; }

                for x in line.limit_iter(0, w as i32) {
                    let x = x as u32;
                    let p = Vector2::new(x as f32, y as f32);
                    let &Luma(dz) = self.depth.get_pixel(x, h-y-1);

                    let cood = bary.coordinate(p);
                    let w = cood.weights();

                    let z = w[0] * clip4.x.z + w[1] * clip4.y.z + w[2] * clip4.z.z;

                    if cood.inside() && z >= -1. && dz[0] > z {
                        let frag = Interpolate::interpolate(&or, w);
                        self.frame.put_pixel(x, h-y-1, fragment.fragment(frag));
                        self.depth.put_pixel(x, h-y-1, Luma([z]));
                    }
                }
            }
        }
    }

    /// This is an extramly slow render that is designed to find missing fragments.
    pub fn debug_raster<S, F, T, O>(&mut self, poly: S, fragment: F)
        where S: Iterator<Item=Triangle<T>>,
              T: Clone + Interpolate<Out=O> + FetchPosition,
              F: Fragment<O, Color=Rgb<u8>> {

        let h = self.frame.height();
        let w = self.frame.width();
        let (hf, wf) = (h as f32, w as f32);
        let (hh, wh) = (hf/2., wf/2.);
        for or in poly {
            let t = or.clone().map_vertex(|v| {
                let v = v.position();
                Vector4::new(v[0], v[1], v[2], v[3])
            });

            let clip4 = t.map_vertex(|v| {
                Vector4::new(
                    hh * (v.x / v.w) + hh,
                    wh * (v.y / v.w) + wh,
                    v.z / v.w,
                    v.w / v.w
                )
            });

            // cull any backface triangles
            if is_backface(clip4.map_vertex(|v| Vector3::new(v.x, v.y, v.z))) {
                continue;
            }

            let clip = clip4.map_vertex(|v| Vector2::new(v.x, v.y));
            let bary = Barycentric::new(clip);

            for y in 0..h {
                for x in 0..w {
                    let x = x as u32;
                    let p = Vector2::new(x as f32, y as f32);
                    let &Luma(dz) = self.depth.get_pixel(x, h-y-1);

                    let cood = bary.coordinate(p);
                    let w = cood.weights();

                    let z = w[0] * clip4.x.z + w[1] * clip4.y.z + w[2] * clip4.z.z;

                    if cood.inside() && z >= -1. && dz[0] > z {
                        let frag = Interpolate::interpolate(&or, w);
                        self.frame.put_pixel(x, h-y-1, fragment.fragment(frag));
                        self.depth.put_pixel(x, h-y-1, Luma([z]));
                    }
                }
            }
        }
    }
}


pub trait FetchPosition {
    fn position(&self) -> [f32; 4];
}

impl FetchPosition for [f32; 4] {
    fn position(&self) -> [f32; 4] { *self }
}

impl<A> FetchPosition for ([f32; 4], A) {
    fn position(&self) -> [f32; 4] { self.0 }
}

impl<A, B> FetchPosition for ([f32; 4], A, B) {
    fn position(&self) -> [f32; 4] { self.0 }
}

impl<A, B, C> FetchPosition for ([f32; 4], A, B, C) {
    fn position(&self) -> [f32; 4] { self.0 }
}

impl<A, B, C, D> FetchPosition for ([f32; 4], A, B, C, D) {
    fn position(&self) -> [f32; 4] { self.0 }
}

impl<A, B, C, D, E> FetchPosition for ([f32; 4], A, B, C, D, E) {
    fn position(&self) -> [f32; 4] { self.0 }
}

impl<A, B, C, D, E, F> FetchPosition for ([f32; 4], A, B, C, D, E, F) {
    fn position(&self) -> [f32; 4] { self.0 }
}

impl<A, B, C, D, E, F, G> FetchPosition for ([f32; 4], A, B, C, D, E, F, G) {
    fn position(&self) -> [f32; 4] { self.0 }
}
