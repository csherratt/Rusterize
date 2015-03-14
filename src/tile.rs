
use std::mem;
use std::num::Int;

use cgmath::*;
use image::{Rgba, ImageBuffer};
use genmesh::Triangle;

use {Barycentric, Interpolate, Fragment};
use f32x8::{f32x8x8, f32x8x8_vec3};


#[derive(Copy, Debug)]
pub struct TileMask {
    u: f32x8x8,
    v: f32x8x8,
    mask: u64
}

impl TileMask {
    #[inline(always)]
    /// Calculate the u/v coordinates for the fragment
    pub fn new(x: u32, y: u32, bary: &Barycentric) -> TileMask {
        let [u, v] =  bary.coordinate_f32x8x8(x, y);
        let uv = f32x8x8::broadcast(1.) - (u + v);

        let mask = !(uv.to_bit_u32x8x8().bitmask() |
                      u.to_bit_u32x8x8().bitmask() |
                      v.to_bit_u32x8x8().bitmask());

        TileMask {
            u: u,
            v: v,
            mask: mask
        }
    }

    #[inline(always)]
    pub fn mask_with_depth(&mut self, z: &Vector3<f32>, d: &mut f32x8x8) {
        let z = f32x8x8_vec3::broadcast(Vector3::new(z.x, z.y, z.z));
        let uv = f32x8x8::broadcast(1.) - (self.u + self.v);
        let weights = f32x8x8_vec3([uv, self.u, self.v]);
        let depth = weights.dot(z);

        self.mask &= (depth - *d).to_bit_u32x8x8().bitmask();
        self.mask &= !depth.to_bit_u32x8x8().bitmask();
        d.replace(depth, self.mask);
    }

    #[inline]
    pub fn iter(self) -> TileMaskIter {
        TileMaskIter {
            u: unsafe { mem::transmute(self.u) },
            v: unsafe { mem::transmute(self.v) },
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
    u: [f32; 64],
    v: [f32; 64],
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
            let u = self.u.get_unchecked(next as usize);
            let v = self.v.get_unchecked(next as usize);
            Some((
                TileIndex(next as u32),
                [1. - (u + v), *u, *v]

            ))
        }
    }
}

#[derive(Copy)]
pub struct Tile<P> {
    depth: f32x8x8,
    color: [P; 64],
}

impl<P: Copy> Clone for Tile<P> {
    fn clone(&self) -> Tile<P> {
        Tile {
            depth: self.depth,
            color: self.color
        }
    }
}

impl<P: Copy> Tile<P> {
    pub fn new(p: P) -> Tile<P> {
         Tile {
            depth: f32x8x8::broadcast(1.),
            color: [p; 64]
        }       
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
pub struct TileGroup<P> {
    tiles: Quad<Quad<Tile<P>>>
}

impl<P: Copy> Clone for TileGroup<P> {
    fn clone(&self) -> TileGroup<P> {
        TileGroup {
            tiles: self.tiles
        }
    }
}

impl<P: Copy> TileGroup<P> {
    pub fn new(p: P) -> TileGroup<P> {
        TileGroup {
            tiles: Quad::new(Quad::new(Tile::new(p)))
        }
    }

    pub fn write<W: Put<P>>(&self, x: u32, y: u32, v: &mut W) {
        self.tiles.write(x, y, v);
    }

    pub fn raster<F, T, O>(&mut self, x: u32, y: u32, z: &Vector3<f32>, bary: &Barycentric, t: &Triangle<T>, fragment: &F) where
              T: Interpolate<Out=O>,
              F: Fragment<O, Color=P> {

        self.tiles.raster(x, y, z, bary, t, fragment);
    }

    pub fn clear(&mut self, p: P) {
        Raster::clear(&mut self.tiles, p);
    }
}

pub trait Raster<P> {
    fn mask(&self) -> u32 { 0xFFFF_FFFF - (self.size() - 1) }
    fn size(&self) -> u32;
    fn raster<F, T, O>(&mut self, x: u32, y: u32, z: &Vector3<f32>, bary: &Barycentric, t: &Triangle<T>, fragment: &F) where
              T: Interpolate<Out=O>,
              F: Fragment<O, Color=P>;

    fn clear(&mut self, p: P);
    fn write<W: Put<P>>(&self, x: u32, y: u32, v: &mut W);
}

impl<I, P: Copy> Raster<P> for Quad<I> where I: Raster<P> {
    #[inline]
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
              F: Fragment<O, Color=P> {

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

    #[inline]
    fn clear(&mut self, p: P) {
        for i in self.0.iter_mut() {
            i.clear(p)
        }
    }

    #[inline]
    fn write<W: Put<P>>(&self, x: u32, y: u32, v: &mut W) {
        let tsize = self.0[0].size();
        self.0[0].write(x,       y,       v);
        self.0[1].write(x+tsize, y,       v);
        self.0[2].write(x,       y+tsize, v);
        self.0[3].write(x+tsize, y+tsize, v);
    }
}

impl<P: Copy> Raster<P> for Tile<P> {
    #[inline]
    fn size(&self) -> u32 { 8 }

    #[inline]
    fn raster<F, T, O>(&mut self,
                       x: u32, y: u32, z: &Vector3<f32>,
                       bary: &Barycentric,
                       t: &Triangle<T>, fragment: &F) where
              T: Interpolate<Out=O>,
              F: Fragment<O, Color=P> {

        let off = Vector2::new(x as f32, y as f32);
        if bary.tile_fast_check(off, Vector2::new(7., 7.)) {
            return;
        }

        let mut mask = TileMask::new(x, y, &bary);
        mask.mask_with_depth(z, &mut self.depth);
        for (i, w) in mask.iter() {
            let frag = Interpolate::interpolate(t, w);
            unsafe { *self.color.get_unchecked_mut(i.0 as usize) = fragment.fragment(frag); }
        }
    }

    #[inline]
    fn write<W: Put<P>>(&self, x: u32, y: u32, v: &mut W) {
        for i in (0..64).map(|x| TileIndex(x)) {
            v.put(x+i.x(), y+i.y(), self.color[i.0 as usize]);
        }
    }

    #[inline]
    fn clear(&mut self, p: P) {
        self.depth = f32x8x8::broadcast(1.);
        self.color = [p; 64];
    }
}

pub trait Put<P> {
    fn put(&mut self, x: u32, y: u32, v: P);
}

impl Put<Rgba<u8>> for ImageBuffer<Rgba<u8>, Vec<u8>> {
    fn put(&mut self, x: u32, y: u32, p: Rgba<u8>) {
        self.put_pixel(x, y, p);
    }
}