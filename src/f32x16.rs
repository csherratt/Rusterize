use std::simd::*;
use std::ops::*;
use cgmath::*;

#[derive(Copy, Clone)]
pub struct f32x16([f32x4; 4]);

impl f32x16 {
    #[inline]
    pub fn new(v0: f32, v1: f32, v2: f32, v3: f32,
           v4: f32, v5: f32, v6: f32, v7: f32,
           v8: f32, v9: f32, v10: f32, v11: f32,
           v12: f32, v13: f32, v14: f32, v15: f32) -> f32x16 {
        f32x16([f32x4(v0, v1, v2, v3),
                f32x4(v4, v5, v6, v7),
                f32x4(v8, v9, v10, v11),
                f32x4(v12, v13, v14, v15)])
    }

    #[inline]
    pub fn broadcast(v: f32) -> f32x16 {
        f32x16::new(v, v, v, v,
                    v, v, v, v,
                    v, v, v, v,
                    v, v, v, v)
    }

    #[inline]
    pub fn range_x_4x4(x: f32) -> f32x16 {
        f32x16::new(x, x+1., x+2., x+3.,
                    x, x+1., x+2., x+3.,
                    x, x+1., x+2., x+3.,
                    x, x+1., x+2., x+3.)
    }

    #[inline]
    pub fn range_y_4x4(x: f32) -> f32x16 {
        f32x16::new(x, x, x, x,
                    x+1., x+1., x+1., x+1.,
                    x+2., x+2., x+2., x+2.,
                    x+3., x+3., x+3., x+3.)
    }
}

impl Add for f32x16 {
    type Output = f32x16;
    #[inline]
    fn add(self, rhs: f32x16) -> f32x16 {
        f32x16([self.0[0] + rhs.0[0],
                self.0[1] + rhs.0[1],
                self.0[2] + rhs.0[2],
                self.0[3] + rhs.0[3]])
    }
}

impl Sub for f32x16 {
    type Output = f32x16;
    #[inline]
    fn sub(self, rhs: f32x16) -> f32x16 {
        f32x16([self.0[0] - rhs.0[0],
                self.0[1] - rhs.0[1],
                self.0[2] - rhs.0[2],
                self.0[3] - rhs.0[3]])
    }
}

impl Mul for f32x16 {
    type Output = f32x16;
    #[inline]
    fn mul(self, rhs: f32x16) -> f32x16 {
        f32x16([self.0[0] * rhs.0[0],
                self.0[1] * rhs.0[1],
                self.0[2] * rhs.0[2],
                self.0[3] * rhs.0[3]])
    }
}

impl Div for f32x16 {
    type Output = f32x16;
    #[inline]
    fn div(self, rhs: f32x16) -> f32x16 {
        f32x16([self.0[0] / rhs.0[0],
                self.0[1] / rhs.0[1],
                self.0[2] / rhs.0[2],
                self.0[3] / rhs.0[3]])
    }
}

#[derive(Copy, Clone)]
pub struct f32x16_vec2([f32x16; 2]);

impl f32x16_vec2 {
    #[inline]
    pub fn broadcast(v: Vector2<f32>) -> f32x16_vec2 {
        f32x16_vec2([f32x16::broadcast(v.x),
                     f32x16::broadcast(v.y)])
    }

    #[inline]
    pub fn range(x: f32, y: f32) -> f32x16_vec2 {
        f32x16_vec2([f32x16::range_x_4x4(x),
                     f32x16::range_y_4x4(y)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x16_vec2) -> f32x16 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1]
    }
}