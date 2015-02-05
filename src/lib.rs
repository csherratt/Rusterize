#![feature(unboxed_closures)]

extern crate image;
extern crate genmesh;
extern crate cgmath;

use image::{GenericImage, ImageBuffer, Rgb, Luma};
use cgmath::*;
use genmesh::{Triangle, MapVertex};
use std::num::Float;

pub struct Frame {
    pub frame: ImageBuffer<Rgb<u8>, Vec<u8>>,
    pub depth: ImageBuffer<Luma<f32>, Vec<f32>>
}

#[inline]
fn orient2d(a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>) -> f32 {
    (b.x-a.x) * (c.y-a.y) - (b.y-a.y) * (c.x-a.x)
}

#[inline]
fn area(a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>) -> f32 {
    ((a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y)) * 0.5).abs()
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
            depth: ImageBuffer::new(width, height)
        }
    }

    pub fn raster<S, F>(&mut self, poly: S, mut fragment: F)
        where S: Iterator<Item=Triangle<Vector4<f32>>>,
              F: FnMut<(f32, f32, f32), Output=Rgb<u8>> {

        let h = self.frame.height();
        let w = self.frame.width();
        let (hf, wf) = (h as f32, w as f32);
        let (hh, wh) = (hf/2., wf/2.);
        for t in poly {
            // cull any backface triangles
            if is_backface(t.map_vertex(|v| Vector3::new(v.x, v.y, v.z))) {
                continue;
            }

            let clip4 = t.map_vertex(|v| {
                Vector4::new(
                    hh * v.x + hh,
                    wh * v.y + wh,
                    v.z,
                    v.w
                )
            });
            let clip = clip4.map_vertex(|v| Vector2::new(v.x, v.y));

            let max_x = clip.x.x.partial_max(clip.y.x.partial_max(clip.z.x)).partial_max(0.).partial_min(hf);
            let min_x = clip.x.x.partial_min(clip.y.x.partial_min(clip.z.x)).partial_max(0.).partial_min(hf);
            let max_y = clip.x.y.partial_max(clip.y.y.partial_max(clip.z.y)).partial_max(0.).partial_min(wf);
            let min_y = clip.x.y.partial_min(clip.y.y.partial_min(clip.z.y)).partial_max(0.).partial_min(wf);

            let points = t.map_vertex(|v| Point3::new(v.x, v.y, v.z));
            let plane = if let Some(plane) = Plane::from_points(points.x, points.y, points.z) {
                plane
            } else {
                continue;
            };

            let a_total = 1. / area(clip.x, clip.y, clip.z);
            let z_inv = 1. / plane.n.z;

            for y in (min_y as u32..max_y as u32) {
                for x in (min_x as u32..max_x as u32) {
                    let q = Vector2::new(x as f32, y as f32);
                    let z = (-plane.d - plane.n.x * q.x - plane.n.y * q.y) * z_inv;

                    let w0 = orient2d(clip.y, clip.z, q);
                    let w1 = orient2d(clip.z, clip.x, q);
                    let w2 = orient2d(clip.x, clip.y, q);

                    let &Luma(dz) = self.depth.get_pixel(x, y);
                    if dz[0] < z && w0 >= 0. && w1 >= 0. && w2 >= 0. {
                        let a0 = area(clip.y, clip.z, q) * a_total;
                        let a1 = area(clip.z, clip.x, q) * a_total;
                        let a2 = area(clip.x, clip.y, q) * a_total;

                        self.frame.put_pixel(x, y, fragment(a0, a1, a2));
                        self.depth.put_pixel(x, y, Luma([z]));
                    }
                }
            }
        }
    }
}