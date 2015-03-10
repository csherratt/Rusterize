
use std::mem;
use std::num::Int;

use cgmath::*;
use image::{Rgba, ImageBuffer};
use genmesh::Triangle;

use {Barycentric, Interpolate, Fragment};
use f32x8::{f32x8x8, f32x8x8_vec3};


#[derive(Copy, Debug)]
pub struct TileMask {
    weights: f32x8x8_vec3,
    mask: u64
}

impl TileMask {
    #[inline]
    /// Calculate the u/v coordinates for the fragment
    pub fn new(pos: Vector2<f32>, bary: &Barycentric) -> TileMask {
        let [u, v] =  bary.coordinate_f32x8x8(pos, Vector2::new(1., 1.));
        let uv = f32x8x8::broadcast(1.) - (u + v);
        let weights = f32x8x8_vec3([uv, u, v]);

        let mask = !(weights.0[0].to_bit_u32x8x8().bitmask() |
                     weights.0[1].to_bit_u32x8x8().bitmask() |
                     weights.0[2].to_bit_u32x8x8().bitmask());

        TileMask {
            weights: weights,
            mask: mask
        }
    }

    #[inline]
    pub fn mask_with_depth(&mut self, z: &Vector3<f32>, d: &mut f32x8x8) {
        let z = f32x8x8_vec3::broadcast(Vector3::new(z.x, z.y, z.z));
        let depth = self.weights.dot(z);
        self.mask &= (depth - *d).to_bit_u32x8x8().bitmask();
        d.replace(depth, self.mask);
    }

    #[inline]
    pub fn iter(self) -> TileMaskIter {
        TileMaskIter {
            weights: unsafe { mem::transmute(self.weights) },
            mask: self.mask
        }
    }
}

#[derive(Copy, Debug)]
pub struct TileIndex(pub u32);

impl TileIndex {
    #[inline]
    pub fn from_xy(x: u32, y: u32) -> TileIndex {
        TileIndex(x*8+y)
    }
    #[inline] pub fn x(self) -> u32 { (self.0 as u32)  & 0x7 }
    #[inline] pub fn y(self) -> u32 { (self.0 as u32)  >> 3 }
    #[inline] pub fn x8(self) -> u32 { self.x() * 8 }
    #[inline] pub fn y8(self) -> u32 { self.y() * 8 }
}

pub struct TileMaskIter {
    weights: [[f32; 64]; 3],
    mask: u64
}

impl Iterator for TileMaskIter {
    type Item = (TileIndex, [f32; 3]);

    #[inline]
    fn next(&mut self) -> Option<(TileIndex, [f32; 3])> {
        if self.mask == 0 {
            return None;
        }

        let next = self.mask.trailing_zeros();
        self.mask &= !(1 << next);
        unsafe {
            Some((
                TileIndex(next as u32),
                [*self.weights[0].get_unchecked(next as usize),
                 *self.weights[1].get_unchecked(next as usize),
                 *self.weights[2].get_unchecked(next as usize)]

            ))
        }
    }
}

#[derive(Copy)]
pub struct Tile {
    depth: f32x8x8,
    color: [Rgba<u8>; 64],
}

impl Clone for Tile {
    fn clone(&self) -> Tile {
        Tile {
            depth: self.depth,
            color: self.color
        }
    }
}

impl Tile {
    pub fn new() -> Tile {
         Tile {
            depth: f32x8x8::broadcast(1.),
            color: [Rgba([0, 0, 0, 0]); 64]
        }       
    }

    #[inline]
    pub fn put(&mut self, idx: TileIndex, value: Rgba<u8>) {
        self.color[idx.0 as usize] = value;
    }
}

#[derive(Copy)]
struct Quad<T>(pub [T; 4]);

impl<T: Copy> Quad<T> {
    pub fn new(t: T) -> Quad<T> {
        Quad([t, t, t, t])
    }
}

#[derive(Copy)]
pub struct TileGroup {
    tiles: Quad<Quad<Quad<Tile>>>
}

impl Clone for TileGroup {
    fn clone(&self) -> TileGroup {
        TileGroup {
            tiles: self.tiles
        }
    }
}

impl TileGroup {
    pub fn new() -> TileGroup {
        TileGroup {
            tiles: Quad::new(Quad::new(Quad::new(Tile::new())))
        }
    }

    pub fn write(&self, x: u32, y: u32, v: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        self.tiles.write(x, y, v);
    }

    pub fn raster<F, T, O>(&mut self, x: u32, y: u32, z: &Vector3<f32>, bary: &Barycentric, t: &Triangle<T>, fragment: &F) where
              T: Interpolate<Out=O>,
              F: Fragment<O, Color=Rgba<u8>> {

        self.tiles.raster(x, y, z, bary, t, fragment);
    }

    pub fn clear(&mut self) {
        Raster::clear(&mut self.tiles);
    }

    /*#[inline]
    pub fn put(&mut self, x: u32, y: u32, value: Rgba<u8>) {
        let gidx = TileIndex::from_xy(x / 8, y / 8);
        let idx = TileIndex::from_xy(x & 0x7, y & 0x7);
        self.tiles[gidx.0 as usize].put(idx, value);
    }*/
}

trait Raster {
    fn size(&self) -> u32;
    fn raster<F, T, O>(&mut self, x: u32, y: u32, z: &Vector3<f32>, bary: &Barycentric, t: &Triangle<T>, fragment: &F) where
              T: Interpolate<Out=O>,
              F: Fragment<O, Color=Rgba<u8>>;

    fn clear(&mut self);
    fn write(&self, x: u32, y: u32, v: &mut ImageBuffer<Rgba<u8>, Vec<u8>>);
}

impl<I> Raster for Quad<I> where I: Raster {
    #[inline(always)]
    fn size(&self) -> u32 { 2 * self.0[0].size() }

    #[inline]
    fn raster<F, T, O>(&mut self,
                       x: u32,
                       y: u32,
                       z: &Vector3<f32>,
                       bary: &Barycentric,
                       t: &Triangle<T>,
                       fragment: &F) where
              T: Interpolate<Out=O>,
              F: Fragment<O, Color=Rgba<u8>> {

        let off = Vector2::new(x as f32, y as f32);
        let size = (self.size() - 1) as f32;
        if bary.tile_fast_check(off, Vector2::new(size, size)) {
            return;
        }

        let tsize = self.0[0].size();
        self.0[0].raster(x,       y,       z, bary, t, fragment);
        self.0[1].raster(x+tsize, y,       z, bary, t, fragment);
        self.0[2].raster(x,       y+tsize, z, bary, t, fragment);
        self.0[3].raster(x+tsize, y+tsize, z, bary, t, fragment);
    }

    fn clear(&mut self) {
        for i in self.0.iter_mut() {
            i.clear()
        }
    }

    fn write(&self, x: u32, y: u32, v: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let tsize = self.0[0].size();
        self.0[0].write(x,       y,       v);
        self.0[1].write(x+tsize, y,       v);
        self.0[2].write(x,       y+tsize, v);
        self.0[3].write(x+tsize, y+tsize, v);
    }
}

impl Raster for Tile {
    #[inline(always)]
    fn size(&self) -> u32 { 8 }

    #[inline]
    fn raster<F, T, O>(&mut self, x: u32, y: u32, z: &Vector3<f32>, bary: &Barycentric, t: &Triangle<T>, fragment: &F) where
              T: Interpolate<Out=O>,
              F: Fragment<O, Color=Rgba<u8>> {

        let off = Vector2::new(x as f32, y as f32);
        if bary.tile_fast_check(off, Vector2::new(7., 7.)) {
            return;
        }

        let off = Vector2::new(x as f32, y as f32);
        let mut mask = TileMask::new(off, &bary);
        mask.mask_with_depth(z, &mut self.depth);
        for (i, w) in mask.iter() {
            let frag = Interpolate::interpolate(t, w);
            unsafe { *self.color.get_unchecked_mut(i.0 as usize) = fragment.fragment(frag); }
        }
    }

    #[inline]
    fn write(&self, x: u32, y: u32, v: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        for i in (0..64).map(|x| TileIndex(x)) {
            v.put_pixel(x+i.x(), y+i.y(), self.color[i.0 as usize]);
        }
    }

    #[inline]
    fn clear(&mut self) {
        self.depth = f32x8x8::broadcast(1.);
        self.color = [Rgba([0, 0, 0, 0]); 64];
    }
}