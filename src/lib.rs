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
                    hh * (v.x / v.w) + hh,
                    wh * (v.y / v.w) + wh,
                    v.z / v.w,
                    v.w / v.w
                )
            });
            let clip = clip4.map_vertex(|v| Vector2::new(v.x, v.y));

            let max_x = clip.x.x.floor().partial_max(clip.y.x.floor().partial_max(clip.z.x.floor())).partial_max(0.).partial_min(hf);
            let min_x = clip.x.x.ceil().partial_min(clip.y.x.ceil().partial_min(clip.z.x.ceil())).partial_max(0.).partial_min(hf);
            let max_y = clip.x.y.floor().partial_max(clip.y.y.floor().partial_max(clip.z.y.floor())).partial_max(0.).partial_min(wf);
            let min_y = clip.x.y.ceil().partial_min(clip.y.y.ceil().partial_min(clip.z.y.ceil())).partial_max(0.).partial_min(wf);

            let points = clip4.map_vertex(|v| Point3::new(v.x, v.y, v.z));
            let plane = if let Some(plane) = Plane::from_points(points.x, points.y, points.z) {
                plane
            } else {
                continue;
            };

            let z_inv = 1. / plane.n.z;

            for y in (min_y as u32..max_y as u32) {
                for x in (min_x as u32..max_x as u32) {
                    let p = Vector2::new(x as f32, y as f32);
                    let z = (-plane.d - plane.n.x * p.x - plane.n.y * p.y) * z_inv;
                    let &Luma(dz) = self.depth.get_pixel(x, y);

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

                    if /*z >= 0. && z <= 1. && */ dz[0] > z && u >= 0. && v >= 0. && (u + v) <= 1. {
                        self.frame.put_pixel(x, y, fragment(v, 1. - (u+v), u));
                        self.depth.put_pixel(x, y, Luma([z]));
                    }
                }
            }
        }
    }
}