
use std;
use std::num::Int;

use interpolate::Interpolate;
use {Frame, FetchPosition, Barycentric};
use image::{Rgb, Luma, ImageBuffer};
use genmesh::{Triangle, MapVertex};
use cgmath::*;

pub type TileMask = u16;

use f32x8::{f32x8, f32x8x8, f32x8x8_vec3, u32x8, u32x8x8};

#[derive(Copy, Debug)]
pub struct Group {
    depth: f32x8x8,
    weights: f32x8x8_vec3,
    mask: u64
}

impl Group {
    #[inline]
    /// Calculate the u/v coordinates for the fragment
    pub fn new(pos: Vector2<f32>, bary: &Barycentric, z: Vector3<f32>) -> Group {
        let [u, v] =  bary.coordinate_f32x8x8(pos, Vector2::new(1., 1.));
        let uv = f32x8x8::broadcast(1.) - (u + v);
        let z = f32x8x8_vec3::broadcast(Vector3::new(z.x, z.y, z.z));
        let weights = f32x8x8_vec3([uv, u, v]);
        let depth = weights.dot(z);

        Group {
            depth: depth,
            weights: weights,
            mask: !(weights.0[0].to_bit_u32x8x8().bitmask() |
                    weights.0[1].to_bit_u32x8x8().bitmask() |
                    weights.0[2].to_bit_u32x8x8().bitmask())
        }
    }

    #[inline]
    pub fn iter(self) -> GroupIter {
        GroupIter {
            depth: self.depth.to_array(),
            weights: self.weights.to_array(),
            mask: self.mask
        }
    }
}

pub struct GroupIter {
    depth: [f32; 64],
    weights: [[f32; 64]; 3],
    mask: u64
}

impl Iterator for GroupIter {
    type Item = (usize, usize, f32, [f32; 3]);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize, f32, [f32; 3])> {
        if self.mask == 0 {
            return None;
        }

        let next = self.mask.trailing_zeros();
        self.mask &= !(1 << next);
        unsafe {
            Some((
                next & 0x7,
                next >> 3,
                *self.depth.get_unchecked(next as usize),
                [*self.weights[0].get_unchecked(next as usize),
                 *self.weights[1].get_unchecked(next as usize),
                 *self.weights[2].get_unchecked(next as usize)]

            ))
        }
    }
}

/*
    #[inline(always)]
    fn next(&mut self) -> Option<(usize, usize, f32, [f32; 3])> {
        while self.idx < 16 {
            let i = self.idx;
            self.idx += 1;
            //assert!(w[0] >= 0. && w[1] >= 0. && w[2] > 0.);
            if self.mask & (1 << i) != 0 {
                let w = [self.weights[0][i as usize],
                         self.weights[1][i as usize],
                         self.weights[2][i as usize]];
                return Some((
                    i & 0x3,
                    i >> 2,
                    self.depth[i as usize],
                    w
                ))                
            }
        }
*/