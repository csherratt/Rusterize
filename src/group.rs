
use std;

use interpolate::Interpolate;
use {Frame, FetchPosition, Barycentric};
use image::{Rgb, Luma, ImageBuffer};
use genmesh::{Triangle, MapVertex};
use cgmath::*;

pub type TileMask = u16;

use f32x4::{f32x4, f32x4_vec3};

#[derive(Copy, Debug)]
pub struct Group {
    depth: f32x4,
    weights: f32x4_vec3
}

impl Group {
    #[inline(always)]
    /// Calculate the u/v coordinates for the fragment
    pub fn new(pos: Vector2<f32>, bary: &Barycentric, z: Vector3<f32>) -> Group {
        let [u, v] =  bary.coordinate_f32x4(pos, Vector2::new(1., 1.));
        let uv = f32x4::broadcast(1.) - (u + v);
        let z = f32x4_vec3::broadcast(Vector3::new(z.x, z.y, z.z));
        let weights = f32x4_vec3([uv, u, v]);
        let depth = weights.dot(z);

        Group {
            depth: depth,
            weights: weights
        }
    }

    #[inline]
    pub fn iter(self) -> GroupIter {
        GroupIter {
            depth: self.depth.to_array(),
            weights: self.weights.to_array(),
            idx: 0
        }
    }
}

pub struct GroupIter {
    depth: [f32; 4],
    weights: [[f32; 4]; 3],
    idx: usize
}

impl Iterator for GroupIter {
    type Item = (usize, usize, f32, [f32; 3]);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize, f32, [f32; 3])> {
        while self.idx < 4 {
            let i = self.idx;
            self.idx += 1;
            let w = [self.weights[0][i as usize],
                     self.weights[1][i as usize],
                     self.weights[2][i as usize]];
            if w[0] >= 0. && w[1] >= 0. && w[2] >= 0. {
                return Some((i & 0x1, i >> 1, self.depth[i as usize], w))
            }
        }
        None
    }
}