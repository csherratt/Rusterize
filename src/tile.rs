
use std::mem;
use std::num::Int;

use Barycentric;
use cgmath::*;

use f32x8::{f32x8, f32x8x8, f32x8x8_vec3};

#[derive(Copy, Debug)]
pub struct Tile {
    weights: f32x8x8_vec3,
    mask: u64
}

impl Tile {
    #[inline]
    /// Calculate the u/v coordinates for the fragment
    pub fn new(pos: Vector2<f32>, bary: &Barycentric) -> Tile {
        let [u, v] =  bary.coordinate_f32x8x8(pos, Vector2::new(1., 1.));
        let uv = f32x8x8::broadcast(1.) - (u + v);
        let weights = f32x8x8_vec3([uv, u, v]);

        let mask = !(weights.0[0].to_bit_u32x8x8().bitmask() |
                     weights.0[1].to_bit_u32x8x8().bitmask() |
                     weights.0[2].to_bit_u32x8x8().bitmask());

        Tile {
            weights: weights,
            mask: mask
        }
    }

    #[inline(always)]
    pub fn mask_with_depth(mut self, z: Vector3<f32>, d: &mut f32x8x8) -> Tile {
        let z = f32x8x8_vec3::broadcast(Vector3::new(z.x, z.y, z.z));
        let depth = self.weights.dot(z);
        self.mask &= (depth - *d).to_bit_u32x8x8().bitmask();
        d.replace(depth, self.mask);
        self
    }

    #[inline]
    pub fn iter(self) -> TileIter {
        TileIter {
            weights: unsafe { mem::transmute(self.weights) },
            mask: self.mask
        }
    }
}

pub struct TileIter {
    weights: [[f32; 64]; 3],
    mask: u64
}

impl Iterator for TileIter {
    type Item = (usize, usize, [f32; 3]);

    #[inline]
    fn next(&mut self) -> Option<(usize, usize, [f32; 3])> {
        if self.mask == 0 {
            return None;
        }

        let next = self.mask.trailing_zeros();
        self.mask &= !(1 << next);
        unsafe {
            Some((
                next & 0x7,
                next >> 3,
                [*self.weights[0].get_unchecked(next as usize),
                 *self.weights[1].get_unchecked(next as usize),
                 *self.weights[2].get_unchecked(next as usize)]

            ))
        }
    }
}
