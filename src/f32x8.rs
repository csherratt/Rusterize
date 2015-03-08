use std::ops::*;
use std::mem;
use cgmath::*;
use super::vmath::Dot;
use std::intrinsics::*;

#[derive(Clone, Copy, Debug)]
#[simd]
pub struct f32x8(pub f32, pub f32, pub f32, pub f32,
                 pub f32, pub f32, pub f32, pub f32);

const MASK_TABLE: [[u32; 4]; 16] = [[ 0, 0, 0, 0],
                                    [!0, 0, 0, 0],
                                    [ 0,!0, 0, 0],
                                    [!0,!0, 0, 0],
                                    [ 0, 0,!0, 0],
                                    [!0, 0,!0, 0],
                                    [ 0,!0,!0, 0],
                                    [!0,!0,!0, 0],
                                    [ 0, 0, 0,!0],
                                    [!0, 0, 0,!0],
                                    [ 0,!0, 0,!0],
                                    [!0,!0, 0,!0],
                                    [ 0, 0,!0,!0],
                                    [!0, 0,!0,!0],
                                    [ 0,!0,!0,!0],
                                    [!0,!0,!0,!0]];
impl f32x8 {
    #[inline]
    pub fn broadcast(v: f32) -> f32x8 {
        f32x8(v, v, v, v, v, v, v, v)
    }

    #[inline]
    pub fn range_x() -> f32x8 {
        f32x8(0., 1., 2., 3., 4., 5., 6., 7.)
    }

    #[inline]
    pub fn replace(&mut self, other: f32x8, mask: u8) {
        let mask = [MASK_TABLE[(mask & 0x0F) as usize],
                    MASK_TABLE[((mask & 0xF0) >> 4) as usize]];
        let mask: u32x8 = unsafe { mem::transmute(mask) };
        let nmask: u32x8 = mask ^ u32x8::broadcast(!0);
        let other: u32x8 = unsafe { mem::transmute(other) };
        let s: u32x8 = unsafe { mem::transmute(*self) };
        *self = unsafe {
            mem::transmute((mask & other) | (nmask & s))
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct f32x8x8(pub f32x8, pub f32x8, pub f32x8, pub f32x8,
                   pub f32x8, pub f32x8, pub f32x8, pub f32x8);

impl f32x8x8 {
    #[inline]
    pub fn broadcast(v: f32) -> f32x8x8 {
        let v = f32x8::broadcast(v);
        f32x8x8(v, v, v, v, v, v, v, v)
    }

    #[inline]
    pub fn range_x() -> f32x8x8 {
        f32x8x8(f32x8::range_x(), 
                f32x8::range_x(),
                f32x8::range_x(),
                f32x8::range_x(),
                f32x8::range_x(),
                f32x8::range_x(),
                f32x8::range_x(),
                f32x8::range_x())
    }

    #[inline]
    pub fn range_y() -> f32x8x8 {
        f32x8x8(f32x8::broadcast(0.), 
                f32x8::broadcast(1.),
                f32x8::broadcast(2.),
                f32x8::broadcast(3.),
                f32x8::broadcast(4.),
                f32x8::broadcast(5.),
                f32x8::broadcast(6.),
                f32x8::broadcast(7.))
    }

    #[inline]
    pub fn replace(&mut self, other: f32x8x8, mask: u64) {
        self.0.replace(other.0, (mask >> 0) as u8);
        self.1.replace(other.1, (mask >> 8) as u8);
        self.2.replace(other.2, (mask >> 16) as u8);
        self.3.replace(other.3, (mask >> 24) as u8);
        self.4.replace(other.4, (mask >> 32) as u8);
        self.5.replace(other.5, (mask >> 40) as u8);
        self.6.replace(other.6, (mask >> 48) as u8);
        self.7.replace(other.7, (mask >> 56) as u8);
    }

    /// casts a each f32 to its bit forms as u32
    /// this is numerically useless, but used for bit twiddling
    /// inside of the library
    #[inline]
    pub fn to_bit_u32x8x8(self) -> u32x8x8 {
        unsafe { mem::transmute(self) }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct f32x8x8_vec3(pub [f32x8x8; 3]);

impl f32x8x8_vec3 {
    #[inline]
    pub fn broadcast(v: Vector3<f32>) -> f32x8x8_vec3 {
        f32x8x8_vec3([f32x8x8::broadcast(v.x),
                      f32x8x8::broadcast(v.y),
                      f32x8x8::broadcast(v.z)])
    }

    #[inline]
    pub fn dot(self, rhs: f32x8x8_vec3) -> f32x8x8 {
        self.0[0] * rhs.0[0] +
        self.0[1] * rhs.0[1] +
        self.0[2] * rhs.0[2]
    }
}

impl Add<f32x8x8> for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn add(self, rhs: f32x8x8) -> f32x8x8 {
        f32x8x8(self.0 + rhs.0, self.1 + rhs.1,
                self.2 + rhs.2, self.3 + rhs.3,
                self.4 + rhs.4, self.5 + rhs.5,
                self.6 + rhs.6, self.7 + rhs.7)
    }
}

impl Add<f32x8> for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn add(self, rhs: f32x8) -> f32x8x8 {
        f32x8x8(self.0 + rhs, self.1 + rhs,
                self.2 + rhs, self.3 + rhs,
                self.4 + rhs, self.5 + rhs,
                self.6 + rhs, self.7 + rhs)
    }
}

impl Sub<f32x8x8> for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn sub(self, rhs: f32x8x8) -> f32x8x8 {
        f32x8x8(self.0 - rhs.0, self.1 - rhs.1,
                self.2 - rhs.2, self.3 - rhs.3,
                self.4 - rhs.4, self.5 - rhs.5,
                self.6 - rhs.6, self.7 - rhs.7)
    }
}

impl Sub<f32x8> for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn sub(self, rhs: f32x8) -> f32x8x8 {
        f32x8x8(self.0 - rhs, self.1 - rhs,
                self.2 - rhs, self.3 - rhs,
                self.4 - rhs, self.5 - rhs,
                self.6 - rhs, self.7 - rhs)
    }
}

impl Mul<f32x8x8> for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn mul(self, rhs: f32x8x8) -> f32x8x8 {
        f32x8x8(self.0 * rhs.0, self.1 * rhs.1,
                self.2 * rhs.2, self.3 * rhs.3,
                self.4 * rhs.4, self.5 * rhs.5,
                self.6 * rhs.6, self.7 * rhs.7)
    }
}

impl Mul<f32x8> for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn mul(self, rhs: f32x8) -> f32x8x8 {
        f32x8x8(self.0 * rhs, self.1 * rhs,
                self.2 * rhs, self.3 * rhs,
                self.4 * rhs, self.5 * rhs,
                self.6 * rhs, self.7 * rhs)
    }
}

impl Mul<f32> for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn mul(self, rhs: f32) -> f32x8x8 {
        self * f32x8::broadcast(rhs)
    }
}

impl Neg for f32x8x8 {
    type Output = f32x8x8;

    #[inline]
    fn neg(self) -> f32x8x8 {
        f32x8x8(-self.0, -self.1,
                -self.2, -self.3,
                -self.4, -self.5,
                -self.6, -self.7)
    }
}

impl Neg for f32x8 {
    type Output = f32x8;

    #[inline]
    fn neg(self) -> f32x8 {
        f32x8(-self.0, -self.1,
              -self.2, -self.3,
              -self.4, -self.5,
              -self.6, -self.7)
    }
}



#[derive(Clone, Copy, Debug)]
#[simd]
pub struct u32x8(pub u32, pub u32, pub u32, pub u32, 
                 pub u32, pub u32, pub u32, pub u32);

impl u32x8 {
    #[inline]
    pub fn broadcast(v: u32) -> u32x8 { u32x8(v, v, v, v, v, v, v, v) }

    #[inline]
    pub fn or_self(self) -> u32 {
        let (a, b) = self.split();
        (a | b).or_self()
    }

    #[inline]
    fn split(self) -> (u32x4, u32x4) {
        unsafe { ::std::mem::transmute(self) }
    }
}

#[derive(Clone, Copy, Debug)]
#[simd]
pub struct u32x4(pub u32, pub u32, pub u32, pub u32);

impl u32x4 {
    #[inline]
    fn split(self) -> (u32x2, u32x2) {
        unsafe { ::std::mem::transmute(self) }
    }


    #[inline]
    pub fn or_self(self) -> u32 {
        let (a, b) = self.split();
        (a | b).or_self()
    }
}

#[derive(Clone, Copy, Debug)]
#[simd]
pub struct u32x2(pub u32, pub u32);

impl u32x2 {
    #[inline]
    fn split(self) -> (u32, u32) {
        unsafe { ::std::mem::transmute(self) }
    }

    #[inline]
    pub fn or_self(self) -> u32 {
        let (a, b) = self.split();
        a | b
    }
}

#[derive(Clone, Copy, Debug)]
pub struct f32x8x8_vec2(pub [f32x8x8; 2]);

impl f32x8x8_vec2 {
    #[inline]
    pub fn range(x: f32, y: f32, xs: f32, ys: f32) -> f32x8x8_vec2 {
        f32x8x8_vec2([f32x8x8::range_x() * f32x8x8::broadcast(xs) + f32x8x8::broadcast(x),
                      f32x8x8::range_y() * f32x8x8::broadcast(ys) + f32x8x8::broadcast(y)])
    }
}

#[derive(Clone, Copy, Debug)]
pub struct u32x8x8(pub u32x8, pub u32x8, pub u32x8, pub u32x8, 
                   pub u32x8, pub u32x8, pub u32x8, pub u32x8);

impl u32x8x8 {
    /// convert component 0-3 into a bitmask. If the value is negative
    /// a bit in the bitmask will be set for it.
    #[inline]
    fn bitmask_low(&self) -> u32 {
        let mask = u32x8::broadcast(0x8000_0000);
        let scale = u32x8(0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80);
        unsafe {
         (overflowing_mul(((self.0 & mask) >> u32x8::broadcast(31)), scale) |
          overflowing_mul(((self.1 & mask) >> u32x8::broadcast(23)), scale) |
          overflowing_mul(((self.2 & mask) >> u32x8::broadcast(15)), scale) |
          overflowing_mul(((self.3 & mask) >> u32x8::broadcast(7)), scale)).or_self()
        }
    }

    /// convert component 4-7 into a bitmask. If the value is negative
    /// a bit in the bitmask will be set for it.
    #[inline]
    fn bitmask_high(&self) -> u32 {
        let mask = u32x8::broadcast(0x8000_0000);
        let scale = u32x8(0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80);
        unsafe {
        (overflowing_mul(((self.4 & mask) >> u32x8::broadcast(31)), scale) |
         overflowing_mul(((self.5 & mask) >> u32x8::broadcast(23)), scale) |
         overflowing_mul(((self.6 & mask) >> u32x8::broadcast(15)), scale) |
         overflowing_mul(((self.7 & mask) >> u32x8::broadcast(7)), scale)).or_self()
        }
    }

    /// convert component 0-7 into a bitmask. If the value is negative
    /// a bit in the bitmask will be set for it.
    #[inline]
    pub fn bitmask(&self) -> u64 {
        self.bitmask_low() as u64 | ((self.bitmask_high() as u64) << 32)
    }
}

impl Dot<f32x8x8_vec2> for f32x8x8_vec2 {
    type Output = f32x8x8;

    #[inline]
    fn dot(self, rhs: f32x8x8_vec2) -> f32x8x8 {
        self.0[0] * rhs.0[0] + self.0[1] * rhs.0[1]
    }
}

impl Dot<f32x8x8_vec2> for Vector2<f32> {
    type Output = f32x8x8;

    #[inline]
    fn dot(self, rhs: f32x8x8_vec2) -> f32x8x8 {
        rhs.0[0] * self.x + rhs.0[1] * self.y
    }
}

impl Dot<Vector2<f32>> for Vector2<f32> {
    type Output = f32;

    #[inline]
    fn dot(self, rhs: Vector2<f32>) -> f32 {
        rhs.x * self.x + rhs.y * self.y
    }
}

