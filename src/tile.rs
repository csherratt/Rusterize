
use std;
use std::simd::*;
use {Frame, Interpolate, FetchPosition, Barycentric};
use image::{Rgb, Luma, ImageBuffer};
use genmesh::{Triangle, MapVertex};
use cgmath::*;

pub type TileMask = u16;

#[derive(Copy)]
pub struct Pixel {
    depth: f32,
    weights: [f32; 3],
    z: f32,
    color: Rgb<u8>
}

impl Pixel {
    fn new() -> Pixel {
        Pixel {
            depth: 0.,
            weights: [0.; 3],
            color: Rgb([0; 3]),
            z: 0.
        }
    }
}

#[derive(Copy)]
pub struct Tile {
    pixels: [Pixel; 16],
    x: i32,
    y: i32
}

impl Tile {
    #[inline]
    pub fn new() -> Tile {
        Tile {
            pixels: [Pixel::new(); 16],
            x: 0,
            y: 0
        }
    }

    #[inline(always)]
    pub fn index(idx: usize) -> (u32, u32) {((idx as u32 >> 3) & 0x7, idx as u32 & 0x7)}

    #[inline]
    pub fn from_frame(frame: &Frame, x: u32, y: u32) -> Tile {
        let mut tile = Tile::new();

        /*for (ix, depth) 

        for (ix, x) in (x..x+8).enumerate() {
            if x > frame.frame.height() { break; }
            for (iy, y) in (y..y+8).enumerate() {
               if y > frame.frame.width() { break; }
               let &Luma([d]) = frame.depth.get_pixel(x, y);
               let &color = frame.frame.get_pixel(x, y);
               tile.depth[ix][iy] = d;
               tile.color[ix][iy] = color;
            }
        }*/

        tile
    }

    #[inline]
    pub fn write_tile(&self, frame: &mut Frame, x: u32, y: u32) {
        /*for (ix, x) in (x..x+8).enumerate() {
            if x > frame.frame.height() { break; }
            for (iy, y) in (y..y+8).enumerate() {
               if y > frame.frame.width() { break; }
               frame.depth.put_pixel(x, y, Luma([self.depth[ix][iy]]));
               frame.frame.put_pixel(x, y, self.color[ix][iy]);
            }
        }*/
    }

    #[inline]
    pub fn raster(&mut self, clip: &Triangle<Vector4<f32>>) {
        let bary = Barycentric::new(clip.map_vertex(|v| Vector2::new(v.x, v.y)));

        for (i, pixel) in self.pixels.iter_mut().enumerate() {
            let x = (i & 0x3) as i32;
            let y = (i >> 2) as i32;
            let p = Vector2::new((self.x+x) as f32, (self.y+y) as f32);
            let cood = bary.coordinate(p);
            pixel.weights = cood.weights();
            let z = pixel.weights[0] * clip.x.z + pixel.weights[1] * clip.y.z + pixel.weights[2] * clip.z.z;
            pixel.z = z;
        }
        /*for (i, ((u, v), w)) in self.u.iter_mut()
                           .zip(self.v.iter_mut())
                           .zip(self.w.iter_mut()).enumerate() {
            let p = [Vector2::new((self.x+0) as f32, (self.y+i as u32) as f32),
                     Vector2::new((self.x+1) as f32, (self.y+i as u32) as f32),
                     Vector2::new((self.x+2) as f32, (self.y+i as u32) as f32),
                     Vector2::new((self.x+3) as f32, (self.y+i as u32) as f32)];
            let cood = [bary.coordinate(p[0]),
                        bary.coordinate(p[1]),
                        bary.coordinate(p[2]),
                        bary.coordinate(p[3])];
            let weights = [cood[0].weights(),
                           cood[1].weights(),
                           cood[2].weights(),
                           cood[3].weights()];
            *u = f32x4(weights[0][0], weights[1][0], weights[2][0], weights[3][0]);
            *v = f32x4(weights[0][1], weights[1][1], weights[2][1], weights[3][1]);
            *w = f32x4(weights[0][2], weights[1][2], weights[2][2], weights[3][2]);
        }*/
    }
}