use std::ops::*;
use cgmath::*;

#[derive(Copy, Debug)]
#[simd]
pub struct f32x16(pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32);

impl f32x16 {
    #[inline]
    pub fn new(v0: f32,  v1: f32,  v2: f32,  v3: f32,
               v4: f32,  v5: f32,  v6: f32,  v7: f32,
               v8: f32,  v9: f32,  v10: f32, v11: f32,
               v12: f32, v13: f32, v14: f32, v15: f32) -> f32x16 {
        f32x16(v0,  v1,  v2,  v3,
               v4,  v5,  v6,  v7,
               v8,  v9,  v10, v11,
               v12, v13, v14, v15)
    }

    #[inline]
    pub fn broadcast(v: f32) -> f32x16 {
        f32x16(v, v, v, v,
               v, v, v, v,
               v, v, v, v,
               v, v, v, v)
    }

    #[inline]
    pub fn range_x() -> f32x16 {
        f32x16(0., 1., 2., 3.,
               0., 1., 2., 3.,
               0., 1., 2., 3.,
               0., 1., 2., 3.)
    }

    #[inline]
    pub fn range_y() -> f32x16 {
        f32x16(0., 0., 0., 0.,
               1., 1., 1., 1.,
               2., 2., 2., 2.,
               3., 3., 3., 3.)
    }

    #[inline]
    pub fn to_array(self) -> [f32; 16] {
        [self.0,  self.1,  self.2,  self.3,
         self.4,  self.5,  self.6,  self.7,
         self.8,  self.9,  self.10, self.11,
         self.12, self.13, self.14, self.15]
    }
}

#[derive(Copy, Debug)]
pub struct f32x16_vec2([f32x16; 2]);

impl f32x16_vec2 {
    #[inline]
    pub fn broadcast(v: Vector2<f32>) -> f32x16_vec2 {
        f32x16_vec2([f32x16::broadcast(v.x),
                     f32x16::broadcast(v.y)])
    }

    #[inline]
    pub fn range(x: f32, y: f32, xs: f32, ys: f32) -> f32x16_vec2 {
        f32x16_vec2([f32x16::range_x() * f32x16::broadcast(xs) + f32x16::broadcast(x),
                     f32x16::range_y() * f32x16::broadcast(ys) + f32x16::broadcast(y)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x16_vec2) -> f32x16 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1]
    }
}

#[derive(Copy, Debug)]
pub struct f32x16_vec3(pub [f32x16; 3]);

impl f32x16_vec3 {
    #[inline]
    pub fn broadcast(v: Vector3<f32>) -> f32x16_vec3 {
        f32x16_vec3([f32x16::broadcast(v.x),
                     f32x16::broadcast(v.y),
                     f32x16::broadcast(v.z)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x16_vec3) -> f32x16 {
        self.0[0] * rhs.0[0] +
        self.0[1] * rhs.0[1] +
        self.0[2] * rhs.0[2]
    }

    #[inline]
    pub fn to_array(self) -> [[f32; 16]; 3] {
        [self.0[0].to_array(),
         self.0[1].to_array(),
         self.0[2].to_array()]
    }
}
