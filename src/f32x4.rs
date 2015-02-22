use std::ops::*;
use cgmath::*;

#[derive(Copy, Debug)]
#[simd]
pub struct f32x4(pub f32, pub f32, pub f32, pub f32);

impl f32x4 {
    #[inline]
    pub fn broadcast(v: f32) -> f32x4 {
        f32x4(v, v, v, v)
    }

    #[inline]
    pub fn range_x() -> f32x4 {
        f32x4(0., 1., 0., 1.)
    }

    #[inline]
    pub fn range_y() -> f32x4 {
        f32x4(0., 0., 1., 1.)
    }

    #[inline]
    pub fn to_array(self) -> [f32; 4] {
        [self.0,  self.1,  self.2,  self.3]
    }
}

#[derive(Copy, Debug)]
pub struct f32x4_vec2([f32x4; 2]);

impl f32x4_vec2 {
    #[inline]
    pub fn broadcast(v: Vector2<f32>) -> f32x4_vec2 {
        f32x4_vec2([f32x4::broadcast(v.x),
                    f32x4::broadcast(v.y)])
    }

    #[inline]
    pub fn range(x: f32, y: f32, xs: f32, ys: f32) -> f32x4_vec2 {
        f32x4_vec2([f32x4::range_x() * f32x4::broadcast(xs) + f32x4::broadcast(x),
                    f32x4::range_y() * f32x4::broadcast(ys) + f32x4::broadcast(y)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x4_vec2) -> f32x4 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1]
    }
}

#[derive(Copy, Debug)]
pub struct f32x4_vec3(pub [f32x4; 3]);

impl f32x4_vec3 {
    #[inline]
    pub fn broadcast(v: Vector3<f32>) -> f32x4_vec3 {
        f32x4_vec3([f32x4::broadcast(v.x),
                    f32x4::broadcast(v.y),
                    f32x4::broadcast(v.z)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x4_vec3) -> f32x4 {
        self.0[0] * rhs.0[0] +
        self.0[1] * rhs.0[1] +
        self.0[2] * rhs.0[2]
    }

    #[inline]
    pub fn to_array(self) -> [[f32; 4]; 3] {
        [self.0[0].to_array(),
         self.0[1].to_array(),
         self.0[2].to_array()]
    }
}
