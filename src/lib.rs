#![feature(simd, unboxed_closures, core, collections)]
#![allow(non_camel_case_types)]

extern crate image;
extern crate genmesh;
extern crate cgmath;
extern crate threadpool;

use std::num::{Float, Int};
use std::sync::{Arc, Future};
use std::iter::{range_step, range_step_inclusive};
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt::Debug;

use threadpool::ScopedPool;
use image::{GenericImage, ImageBuffer, Rgba};
use cgmath::*;
use genmesh::{Triangle, MapVertex};

use tile::TileGroup;
use vmath::Dot;
use f32x8::f32x8x8;
pub use pipeline::{Fragment, Vertex};
pub use interpolate::{Flat, Interpolate};

mod interpolate;
mod pipeline;
mod f32x4;
pub mod f32x8;
mod vmath;
pub mod tile;


#[cfg(dump)]
fn dump(idx: usize, frame: &Frame) {
    use std::old_io::File;
    // Save the image output just incase the test fails
    let mut fout = File::create(&Path::new("dump").join(format!("{:05}.png", idx))).unwrap();
    let _= image::ImageRgba8(frame.frame.clone()).save(&mut fout, image::PNG);
}

#[inline]
pub fn is_backface(v: Triangle<Vector3<f32>>)-> bool {
    let e0 = v.z - v.x;
    let e1 = v.z - v.y;
    let normal = e1.cross(&e0);
    Vector3::new(0., 0., 1.).dot(&normal) >= 0.
}

#[derive(Clone, Copy, Debug)]
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

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d11 = v1.dot(v1);

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

        let d00 = self.v0.dot(self.v0);
        let d01 = self.v0.dot(self.v1);
        let d02 = self.v0.dot(v2);
        let d11 = self.v1.dot(self.v1);
        let d12 = self.v1.dot(v2);

        let u = (d11 * d02 - d01 * d12) * self.inv_denom;
        let v = (d00 * d12 - d01 * d02) * self.inv_denom;

        BarycentricCoordinate {
            u: u,
            v: v
        }
    }

    #[inline]
    pub fn coordinate_f32x4(&self, p: Vector2<f32>, s: Vector2<f32>) -> [f32x4::f32x4; 2] {
        use f32x4::{f32x4, f32x4_vec2};
        let v2 = p - self.base;

        let v0 = f32x4_vec2::broadcast(self.v0);
        let v1 = f32x4_vec2::broadcast(self.v1);
        let v2 = f32x4_vec2::range(v2.x, v2.y, s.x, s.y);

        let d00 = v0.dot(v0);
        let d01 = v0.dot(v1);
        let d02 = v0.dot(v2);
        let d11 = v1.dot(v1);
        let d12 = v1.dot(v2);

        let inv_denom = f32x4::broadcast(self.inv_denom);

        [(d11 * d02 - d01 * d12) * inv_denom,
         (d00 * d12 - d01 * d02) * inv_denom]
    }

    #[inline(always)]
    pub fn coordinate_f32x8x8(&self, p: Vector2<f32>, s: Vector2<f32>) -> [f32x8::f32x8x8; 2] {
        use f32x8::{f32x8x8, f32x8x8_vec2};
        let v2 = p - self.base;

        let v2 = f32x8x8_vec2::range(v2.x, v2.y, s.x, s.y);

        let d00 = self.v0.dot(self.v0);
        let d01 = self.v0.dot(self.v1);
        let d02 = self.v0.dot(v2);
        let d11 = self.v1.dot(self.v1);
        let d12 = self.v1.dot(v2);

        let inv_denom = f32x8x8::broadcast(self.inv_denom);

        [(d02 * d11 - d12 * d01) * inv_denom,
         (d12 * d00 - d02 * d01) * inv_denom]
    }

    /// a fast to check to tell if a tile is inside of the triangle or not
    #[inline]
    pub fn tile_fast_check(&self, p: Vector2<f32>, s: Vector2<f32>) -> bool {
        use f32x4::{f32x4};
        let [u, v] = self.coordinate_f32x4(p, s);
        let uv = f32x4::broadcast(1.) - (u + v);
        let mask = u.to_bit_u32x4().and_self() |
                   v.to_bit_u32x4().and_self() |
                   uv.to_bit_u32x4().and_self();

        mask & 0x8000_0000 != 0
    }

    #[inline]
    pub fn tile_covered(&self, p: Vector2<f32>, s: Vector2<f32>) -> bool {
        use f32x4::{f32x4};
        let [u, v] = self.coordinate_f32x4(p, s);
        let uv = f32x4::broadcast(1.) - (u + v);
        let mask = u.to_bit_u32x4().or_self() |
                   v.to_bit_u32x4().or_self() |
                   uv.to_bit_u32x4().or_self();

        mask & 0x8000_0000 != 0
    }
}

struct TriangleGroup<T>(Vec<(Triangle<Vector4<f32>>, Triangle<T>)>);

impl<T> TriangleGroup<T> {
    fn from_iter<S, O>(poly: &mut S, wh: f32, hh: f32) -> TriangleGroup<T>
        where S: Iterator<Item=Triangle<T>>,
              T: Clone + Interpolate<Out=O> + FetchPosition {

        TriangleGroup(
            poly.map(|or| {
                let t = or.clone().map_vertex(|v| {
                    let v = v.position();
                    Vector4::new(v[0], v[1], v[2], v[3])
                });

                let clip4 = t.map_vertex(|v| {
                    Vector4::new(
                        wh * (v.x / v.w) + wh,
                        -hh * (v.y / v.w) + hh,
                        v.z / v.w,
                        v.w / v.w
                    )
                });

                (clip4, or)
            }).filter(|&(ref clip4, _)| {
                is_backface(clip4.map_vertex(|v| Vector3::new(v.x, v.y, v.z)))
            }).take(64).collect()
        )

    }
}


#[derive(Clone)]
pub struct Frame {
    pub width: u32,
    pub height: u32,
    pub tile: Vec<Vec<Box<TileGroup>>>,
}

impl Frame {
    pub fn new(width: u32, height: u32) -> Frame {
        Frame {
            width: width,
            height: height,
            tile: (0..(height / 64)).map(
                |_| (0..(width / 64)).map(
                    |_| Box::new(TileGroup::new())
                ).collect()
            ).collect()
        }
    }

    pub fn clear(&mut self) {
        for row in self.tile.iter_mut() {
            for tile in row.iter_mut() {
                tile.clear();
            }
        }
    }

    fn get_tile_mut(&mut self, x: usize, y: usize) -> &mut TileGroup {
        return &mut self.tile[x][y]
    }

    pub fn to_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut buffer = ImageBuffer::new(self.width, self.height);

        for (x, row) in self.tile.iter().enumerate() {
            for (y, tile) in row.iter().enumerate() {
                tile.write((x*64) as u32, (y*64) as u32, &mut buffer);
            }
        }

        buffer
    }

    pub fn raster<S, F, T, O>(&mut self, mut poly: S, fragment: F)
        where S: Iterator<Item=Triangle<T>>,
              T: Clone + Interpolate<Out=O> + FetchPosition + Send + Sync,
              F: Fragment<O, Color=Rgba<u8>> + Send + Sync {

        use std::cmp::{min, max};
        let h = self.height;
        let w = self.width;
        let (hf, wf) = (h as f32, w as f32);
        let (hh, wh) = (hf/2., wf/2.);

        let mut commands: Vec<Vec<Vec<(u64, Arc<TriangleGroup<T>>)>>> =
            (0..(h / 64)).map( 
                |_| (0..(w / 64)).map(
                    |_| Vec::new()
                ).collect()
            ).collect();

        loop {
            let group = Arc::new(TriangleGroup::from_iter(&mut poly, hh, wh));
            if group.0.len() == 0 {
                break;
            }

            let mut apply: BTreeMap<(u32, u32), u64> = BTreeMap::new();

            for (idx, &(ref clip4, _)) in group.0.iter().enumerate() {
                let clip = clip4.map_vertex(|v| Vector2::new(v.x, v.y));

                let max_x = clip.x.x.ceil().partial_max(clip.y.x.ceil().partial_max(clip.z.x.ceil()));
                let min_x = clip.x.x.floor().partial_min(clip.y.x.floor().partial_min(clip.z.x.floor()));
                let max_y = clip.x.y.ceil().partial_max(clip.y.y.ceil().partial_max(clip.z.y.ceil()));
                let min_y = clip.x.y.floor().partial_min(clip.y.y.floor().partial_min(clip.z.y.floor()));

                let min_x = (max(min_x as i32, 0) as u32) & (0xFFFFFFFF & !0x3F);
                let min_y = (max(min_y as i32, 0) as u32) & (0xFFFFFFFF & !0x3F);
                let max_x = min(max_x as u32, w);
                let max_y = min(max_y as u32, h);
                let max_x = if max_x & (64-1) != 0 { max_x + (64 - (max_x & (64-1))) } else { max_x };
                let max_y = if max_y & (64-1) != 0 { max_y + (64 - (max_y & (64-1))) } else { max_y };

                for y in range_step(min_y, max_y, 64) {
                    for x in range_step(min_x, max_x, 64) {
                        match apply.entry((y/64, x/64)) {
                            Entry::Occupied(mut entry) => *entry.get_mut() |= 1 << idx as u64,
                            Entry::Vacant(entry) => { entry.insert(1 << idx as u64); }
                        }
                    }
                }
            }

            for ((y, x), mask) in apply.into_iter() {
                commands[x as usize][y as usize].push((mask, group.clone()));
            }

            if group.0.len() < 64 {
                break;
            }
        }


        let fragment = Arc::new(fragment);
        {
            let pool = ScopedPool::new(8);
            for (x, (row, row_commands)) in self.tile.iter_mut().zip(commands.into_iter()).enumerate() {
                for (y, (tile, tile_command)) in row.iter_mut().zip(row_commands.into_iter()).enumerate() {
                    if tile_command.len() == 0 {
                        continue;
                    }

                    let x = x as u32;
                    let y = y as u32;
                    let fragment = fragment.clone();
                    pool.execute(move || {
                        for (mut mask, group) in tile_command.into_iter() {
                            while mask != 0 {
                                let next = mask.trailing_zeros();
                                mask &= !(1 << next);

                                let (clip4, ref or) = group.0[next as usize];
                                let clip3 = Vector3::new(clip4.x.z, clip4.y.z, clip4.z.z);
                                let clip = clip4.map_vertex(|v| Vector2::new(v.x, v.y));
                                let bary = Barycentric::new(clip);

                                tile.raster(x*64, y*64, &clip3, &bary, or, &*fragment);
                            }
                        }
                    });
                }
            }
        }

    }

    /// draw grid line over the frame buffer. This is mostly a debug feature
    pub fn draw_grid(&mut self, spacing: u32, color: Rgba<u8>) {
        let h = self.height;
        let w = self.width;

        let mut put = |x, y| {
            if x < w && y < h {
                let tx = (x / 64) as usize;
                let ty = (y / 64) as usize;
                self.tile[tx][ty].put(x % 64, y % 64, color);
            }
        };

        for x in range_step_inclusive(0, w, spacing) {
            for y in range_step_inclusive(0, h, spacing) {
                put(x, y-1);
                put(x, y);
                put(x, y+1);
            }
        }
        for y in range_step_inclusive(0, h, spacing) {
            for x in range_step_inclusive(0, w, spacing) {
                put(x-1, y);
                put(x, y);
                put(x+1, y);
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
