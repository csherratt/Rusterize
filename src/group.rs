
use std;
use std::simd::*;

use {Frame, Interpolate, FetchPosition, Barycentric};
use image::{Rgb, Luma, ImageBuffer};
use genmesh::{Triangle, MapVertex};
use cgmath::*;

pub type TileMask = u16;

use f32x16::f32x16;

#[derive(Copy)]
pub struct Group {
    depth: f32x16,
    u: f32x16,
    v: f32x16,
    pos: Vector2<f32>
}

impl Group {
    #[inline]
    pub fn new(pos: Vector2<f32>) -> Group {
        Group {
            depth: f32x16::broadcast(0.),
            u: f32x16::broadcast(0.),
            v: f32x16::broadcast(0.),
            pos: pos
        }
    }

    #[inline]
    pub fn raster(&mut self, clip: &Triangle<Vector4<f32>>) {
        let bary = Barycentric::new(clip.map_vertex(|v| Vector2::new(v.x, v.y)));
        let [u, v] = bary.coordinate_f32x16(self.pos);
        self.u = u;
        self.v = v;
    }
}