#![feature(unboxed_closures)]

extern crate image;
extern crate genmesh;
extern crate cgmath;

use image::{GenericImage, ImageBuffer, Rgb, Luma};
use cgmath::*;
use genmesh::{Triangle, MapVertex};
use std::num::Float;

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

impl Frame {
    pub fn new(width: u32, height: u32) -> Frame {
        Frame {
            frame: ImageBuffer::new(width, height),
            depth: ImageBuffer::from_pixel(width, height, Luma([1.]))
        }
    }

    pub fn raster<S, F, T>(&mut self, poly: S, mut fragment: F)
        where S: Iterator<Item=Triangle<T>>,
              F: FnMut<(T,), Output=Rgb<u8>>,
              T: FetchPosition + Clone + Interpolate {

        let h = self.frame.height();
        let w = self.frame.width();
        let (hf, wf) = (h as f32, w as f32);
        let (hh, wh) = (hf/2., wf/2.);
        for or in poly {
            let t = or.clone().map_vertex(|v| {
                let v = v.position();
                Vector4::new(v[0], v[1], v[2], v[3])
            });

            // cull any backface triangles
            if is_backface(t.map_vertex(|v| Vector3::new(v.x, v.y, v.z))) {
                continue;
            }

            let clip4 = t.map_vertex(|v| {
                Vector4::new(
                    hh * (v.x / v.w) + hh,
                    wh * (v.y / v.w) + wh,
                    v.z / v.w,
                    v.w / v.w
                )
            });
            let clip = clip4.map_vertex(|v| Vector2::new(v.x, v.y));

            let max_x = clip.x.x.ceil().partial_max(clip.y.x.ceil().partial_max(clip.z.x.ceil())).partial_max(0.).partial_min(hf);
            let min_x = clip.x.x.floor().partial_min(clip.y.x.floor().partial_min(clip.z.x.floor())).partial_max(0.).partial_min(hf);
            let max_y = clip.x.y.ceil().partial_max(clip.y.y.ceil().partial_max(clip.z.y.ceil())).partial_max(0.).partial_min(wf);
            let min_y = clip.x.y.floor().partial_min(clip.y.y.floor().partial_min(clip.z.y.floor())).partial_max(0.).partial_min(wf);

            for y in (min_y as u32..max_y as u32) {
                for x in (min_x as u32..max_x as u32) {
                    let p = Vector2::new(x as f32, y as f32);
                    let &Luma(dz) = self.depth.get_pixel(x, h-y-1);

                    let v0 = clip.y - clip.x;
                    let v1 = clip.z - clip.x;
                    let v2 = p - clip.x;

                    let d00 = v0.dot(&v0);
                    let d01 = v0.dot(&v1);
                    let d02 = v0.dot(&v2);
                    let d11 = v1.dot(&v1);
                    let d12 = v1.dot(&v2);

                    let inv_denom = 1. / (d00 * d11 - d01 * d01);
                    let u = (d11 * d02 - d01 * d12) * inv_denom;
                    let v = (d00 * d12 - d01 * d02) * inv_denom;

                    let a = (1. - (u+v));
                    let b = u;
                    let c = v;

                    let z = a * clip4.x.z + b * clip4.y.z + c * clip4.z.z;

                    if u >= 0. && v >= 0. && (u + v) <= 1. && z >= 0. && z <= 1. && dz[0] > z {
                        let frag = Interpolate::interpolate(&or, [a, b, c]);
                        self.frame.put_pixel(x, h-y-1, fragment(frag));
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

pub trait Interpolate {
    fn interpolate(src: &Triangle<Self>, w: [f32; 3]) -> Self;
}

impl Interpolate for f32 {
    fn interpolate(src: &Triangle<f32>, w: [f32; 3]) -> f32 {
        src.x * w[0] + src.y * w[1] + src.z * w[2]
    }
}

impl Interpolate for [f32; 2] {
    fn interpolate(src: &Triangle<[f32; 2]>, w: [f32; 3]) -> [f32; 2] {
        [Interpolate::interpolate(&Triangle::new(src.x[0], src.y[0], src.z[0]), w),
         Interpolate::interpolate(&Triangle::new(src.x[1], src.y[1], src.z[1]), w)]
    }
}

impl Interpolate for [f32; 3] {
    fn interpolate(src: &Triangle<[f32; 3]>, w: [f32; 3]) -> [f32; 3] {
        [Interpolate::interpolate(&Triangle::new(src.x[0], src.y[0], src.z[0]), w),
         Interpolate::interpolate(&Triangle::new(src.x[1], src.y[1], src.z[1]), w),
         Interpolate::interpolate(&Triangle::new(src.x[2], src.y[2], src.z[2]), w)]
    }
}

impl Interpolate for [f32; 4] {
    fn interpolate(src: &Triangle<[f32; 4]>, w: [f32; 3]) -> [f32; 4] {
        [Interpolate::interpolate(&Triangle::new(src.x[0], src.y[0], src.z[0]), w),
         Interpolate::interpolate(&Triangle::new(src.x[1], src.y[1], src.z[1]), w),
         Interpolate::interpolate(&Triangle::new(src.x[2], src.y[2], src.z[2]), w),
         Interpolate::interpolate(&Triangle::new(src.x[3], src.y[3], src.z[3]), w)]
    }
}

impl<A, B> Interpolate for (A, B)
    where A: Interpolate + Clone,
          B: Interpolate + Clone {
    fn interpolate(src: &Triangle<(A, B)>, w: [f32; 3]) -> (A, B) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w))
    }
}

impl<A, B, C> Interpolate for (A, B, C)
    where A: Interpolate + Clone,
          B: Interpolate + Clone,
          C: Interpolate + Clone {
    fn interpolate(src: &Triangle<(A, B, C)>, w: [f32; 3]) -> (A, B, C) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w))
    }
}

impl<A, B, C, D> Interpolate for (A, B, C, D)
    where A: Interpolate + Clone,
          B: Interpolate + Clone,
          C: Interpolate + Clone,
          D: Interpolate + Clone {
    fn interpolate(src: &Triangle<(A, B, C, D)>, w: [f32; 3]) -> (A, B, C, D) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w))
    }
}

impl<A, B, C, D, E> Interpolate for (A, B, C, D, E)
    where A: Interpolate + Clone,
          B: Interpolate + Clone,
          C: Interpolate + Clone,
          D: Interpolate + Clone,
          E: Interpolate + Clone {
    fn interpolate(src: &Triangle<(A, B, C, D, E)>, w: [f32; 3]) -> (A, B, C, D, E) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w))
    }
}

impl<A, B, C, D, E, F> Interpolate for (A, B, C, D, E, F)
    where A: Interpolate + Clone,
          B: Interpolate + Clone,
          C: Interpolate + Clone,
          D: Interpolate + Clone,
          E: Interpolate + Clone,
          F: Interpolate + Clone {
    fn interpolate(src: &Triangle<(A, B, C, D, E, F)>, w: [f32; 3]) -> (A, B, C, D, E, F) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.5.clone(), src.y.5.clone(), src.z.5.clone()), w))
    }
}

impl<A, B, C, D, E, F, G> Interpolate for (A, B, C, D, E, F, G)
    where A: Interpolate + Clone,
          B: Interpolate + Clone,
          C: Interpolate + Clone,
          D: Interpolate + Clone,
          E: Interpolate + Clone,
          F: Interpolate + Clone,
          G: Interpolate + Clone {
    fn interpolate(src: &Triangle<(A, B, C, D, E, F, G)>, w: [f32; 3]) -> (A, B, C, D, E, F, G) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.5.clone(), src.y.5.clone(), src.z.5.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.6.clone(), src.y.6.clone(), src.z.6.clone()), w))
    }
}

impl<A, B, C, D, E, F, G, H> Interpolate for (A, B, C, D, E, F, G, H)
    where A: Interpolate + Clone,
          B: Interpolate + Clone,
          C: Interpolate + Clone,
          D: Interpolate + Clone,
          E: Interpolate + Clone,
          F: Interpolate + Clone,
          G: Interpolate + Clone,
          H: Interpolate + Clone {
    fn interpolate(src: &Triangle<(A, B, C, D, E, F, G, H)>, w: [f32; 3]) -> (A, B, C, D, E, F, G, H) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.5.clone(), src.y.5.clone(), src.z.5.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.6.clone(), src.y.6.clone(), src.z.6.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.7.clone(), src.y.7.clone(), src.z.7.clone()), w))
    }
}
