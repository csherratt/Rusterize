use std::ops::*;
use cgmath::*;

#[derive(Copy, Debug)]
#[simd]
pub struct f32x64(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32,
                  pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);

impl f32x64 {
    #[inline]
    pub fn broadcast(v: f32) -> f32x64 {
        f32x64(v, v, v, v, v, v, v, v,
               v, v, v, v, v, v, v, v,
               v, v, v, v, v, v, v, v,
               v, v, v, v, v, v, v, v,
               v, v, v, v, v, v, v, v,
               v, v, v, v, v, v, v, v,
               v, v, v, v, v, v, v, v,
               v, v, v, v, v, v, v, v)
    }

    #[inline]
    pub fn range_x() -> f32x64 {
        f32x64(0., 1., 2., 3., 4., 5., 6., 7.,
               0., 1., 2., 3., 4., 5., 6., 7.,
               0., 1., 2., 3., 4., 5., 6., 7.,
               0., 1., 2., 3., 4., 5., 6., 7.,
               0., 1., 2., 3., 4., 5., 6., 7.,
               0., 1., 2., 3., 4., 5., 6., 7.,
               0., 1., 2., 3., 4., 5., 6., 7.,
               0., 1., 2., 3., 4., 5., 6., 7.)
    }

    #[inline]
    pub fn range_y() -> f32x64 {
        f32x64(0., 0., 0., 0., 0., 0., 0., 0.,
               1., 1., 1., 1., 1., 1., 1., 1.,
               2., 2., 2., 2., 2., 2., 2., 2.,
               3., 3., 3., 3., 3., 3., 3., 3.,
               4., 4., 4., 4., 4., 4., 4., 4.,
               5., 5., 5., 5., 5., 5., 5., 5.,
               6., 6., 6., 6., 6., 6., 6., 6.,
               7., 7., 7., 7., 7., 7., 7., 7.)
    }

    #[inline]
    pub fn to_array(self) -> [f32; 64] {
        [self.0,   self.1,   self.2,   self.3,   self.4,   self.5,   self.6,   self.7,
         self.8,   self.9,   self.10,  self.11,  self.12,  self.13,  self.14,  self.15,
         self.16,  self.17,  self.18,  self.19,  self.20,  self.21,  self.22,  self.23,
         self.24,  self.25,  self.26,  self.27,  self.28,  self.29,  self.30,  self.31,
         self.32,  self.33,  self.34,  self.35,  self.36,  self.37,  self.38,  self.39,
         self.40,  self.41,  self.42,  self.43,  self.44,  self.45,  self.46,  self.47,
         self.48,  self.49,  self.50,  self.51,  self.52,  self.53,  self.54,  self.55,
         self.56,  self.57,  self.58,  self.59,  self.60,  self.61,  self.62,  self.63]
    }
}

#[derive(Copy, Debug)]
pub struct f32x64_vec2([f32x64; 2]);

impl f32x64_vec2 {
    #[inline]
    pub fn broadcast(v: Vector2<f32>) -> f32x64_vec2 {
        f32x64_vec2([f32x64::broadcast(v.x),
                     f32x64::broadcast(v.y)])
    }

    #[inline]
    pub fn range(x: f32, y: f32, xs: f32, ys: f32) -> f32x64_vec2 {
        f32x64_vec2([f32x64::range_x() * f32x64::broadcast(xs) + f32x64::broadcast(x),
                     f32x64::range_y() * f32x64::broadcast(ys) + f32x64::broadcast(y)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x64_vec2) -> f32x64 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1]
    }
}

#[derive(Copy, Debug)]
pub struct f32x64_vec3(pub [f32x64; 3]);

impl f32x64_vec3 {
    #[inline]
    pub fn broadcast(v: Vector3<f32>) -> f32x64_vec3 {
        f32x64_vec3([f32x64::broadcast(v.x),
                     f32x64::broadcast(v.y),
                     f32x64::broadcast(v.z)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x64_vec3) -> f32x64 {
        self.0[0] * rhs.0[0] +
        self.0[1] * rhs.0[1] +
        self.0[2] * rhs.0[2]
    }

    #[inline]
    pub fn to_array(self) -> [[f32; 64]; 3] {
        [self.0[0].to_array(),
         self.0[1].to_array(),
         self.0[2].to_array()]
    }
}
