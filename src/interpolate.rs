pub use cgmath::*;
use genmesh::Triangle;


#[derive(Clone, Debug, Copy)]
pub struct Flat<T>(pub T);

impl<T: Clone> Interpolate for Flat<T> {
    type Out = T;
    #[inline]
    fn interpolate(src: &Triangle<Flat<T>>, _: [f32; 3]) -> T { src.x.0.clone() }
}

pub trait Interpolate {
    type Out;

    #[inline]
    fn interpolate(src: &Triangle<Self>, w: [f32; 3]) -> Self::Out;
}

impl Interpolate for f32 {
    type Out = f32;
    #[inline]
    fn interpolate(src: &Triangle<f32>, w: [f32; 3]) -> f32 {
        src.x * w[0] + src.y * w[1] + src.z * w[2]
    }
}

impl Interpolate for [f32; 2] {
    type Out = [f32; 2];
    #[inline]
    fn interpolate(src: &Triangle<[f32; 2]>, w: [f32; 3]) -> [f32; 2] {
        [Interpolate::interpolate(&Triangle::new(src.x[0], src.y[0], src.z[0]), w),
         Interpolate::interpolate(&Triangle::new(src.x[1], src.y[1], src.z[1]), w)]
    }
}

impl Interpolate for [f32; 3] {
    type Out = [f32; 3];
    #[inline]
    fn interpolate(src: &Triangle<[f32; 3]>, w: [f32; 3]) -> [f32; 3] {
        [Interpolate::interpolate(&Triangle::new(src.x[0], src.y[0], src.z[0]), w),
         Interpolate::interpolate(&Triangle::new(src.x[1], src.y[1], src.z[1]), w),
         Interpolate::interpolate(&Triangle::new(src.x[2], src.y[2], src.z[2]), w)]
    }
}

impl Interpolate for [f32; 4] {
    type Out = [f32; 4];
    #[inline]
    fn interpolate(src: &Triangle<[f32; 4]>, w: [f32; 3]) -> [f32; 4] {
        [Interpolate::interpolate(&Triangle::new(src.x[0], src.y[0], src.z[0]), w),
         Interpolate::interpolate(&Triangle::new(src.x[1], src.y[1], src.z[1]), w),
         Interpolate::interpolate(&Triangle::new(src.x[2], src.y[2], src.z[2]), w),
         Interpolate::interpolate(&Triangle::new(src.x[3], src.y[3], src.z[3]), w)]
    }
}

impl<A, B, AO, BO> Interpolate for (A, B)
    where A: Interpolate<Out=AO> + Clone,
          B: Interpolate<Out=BO> + Clone {
    type Out = (AO, BO);
    #[inline]
    fn interpolate(src: &Triangle<(A, B)>, w: [f32; 3]) -> (AO, BO) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w))
    }
}

impl<A, B, C, AO, BO, CO> Interpolate for (A, B, C)
    where A: Interpolate<Out=AO> + Clone,
          B: Interpolate<Out=BO> + Clone,
          C: Interpolate<Out=CO> + Clone {
    type Out = (AO, BO, CO);
    #[inline]
    fn interpolate(src: &Triangle<(A, B, C)>, w: [f32; 3]) -> (AO, BO, CO) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w))
    }
}

impl<A, B, C, D, AO, BO, CO, DO> Interpolate for (A, B, C, D)
    where A: Interpolate<Out=AO> + Clone,
          B: Interpolate<Out=BO> + Clone,
          C: Interpolate<Out=CO> + Clone,
          D: Interpolate<Out=DO> + Clone {
    type Out = (AO, BO, CO, DO);
    #[inline]
    fn interpolate(src: &Triangle<(A, B, C, D)>, w: [f32; 3]) -> (AO, BO, CO, DO) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w))
    }
}

impl<A, B, C, D, E, AO, BO, CO, DO, EO> Interpolate for (A, B, C, D, E)
    where A: Interpolate<Out=AO> + Clone,
          B: Interpolate<Out=BO> + Clone,
          C: Interpolate<Out=CO> + Clone,
          D: Interpolate<Out=DO> + Clone,
          E: Interpolate<Out=EO> + Clone {
    type Out = (AO, BO, CO, DO, EO);
    #[inline]
    fn interpolate(src: &Triangle<(A, B, C, D, E)>, w: [f32; 3]) -> (AO, BO, CO, DO, EO) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w))
    }
}

impl<A, B, C, D, E, F, AO, BO, CO, DO, EO, FO> Interpolate for (A, B, C, D, E, F)
    where A: Interpolate<Out=AO> + Clone,
          B: Interpolate<Out=BO> + Clone,
          C: Interpolate<Out=CO> + Clone,
          D: Interpolate<Out=DO> + Clone,
          E: Interpolate<Out=EO> + Clone,
          F: Interpolate<Out=FO> + Clone {
    type Out = (AO, BO, CO, DO, EO, FO);
    #[inline]
    fn interpolate(src: &Triangle<(A, B, C, D, E, F)>, w: [f32; 3]) -> (AO, BO, CO, DO, EO, FO) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.5.clone(), src.y.5.clone(), src.z.5.clone()), w))
    }
}

impl<A, B, C, D, E, F, G, AO, BO, CO, DO, EO, FO, GO> Interpolate for (A, B, C, D, E, F, G)
    where A: Interpolate<Out=AO> + Clone,
          B: Interpolate<Out=BO> + Clone,
          C: Interpolate<Out=CO> + Clone,
          D: Interpolate<Out=DO> + Clone,
          E: Interpolate<Out=EO> + Clone,
          F: Interpolate<Out=FO> + Clone,
          G: Interpolate<Out=GO> + Clone {
    type Out = (AO, BO, CO, DO, EO, FO, GO);
    #[inline]
    fn interpolate(src: &Triangle<(A, B, C, D, E, F, G)>, w: [f32; 3]) -> (AO, BO, CO, DO, EO, FO, GO) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.5.clone(), src.y.5.clone(), src.z.5.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.6.clone(), src.y.6.clone(), src.z.6.clone()), w))
    }
}

impl<A, B, C, D, E, F, G, H, AO, BO, CO, DO, EO, FO, GO, HO> Interpolate for (A, B, C, D, E, F, G, H)
    where A: Interpolate<Out=AO> + Clone,
          B: Interpolate<Out=BO> + Clone,
          C: Interpolate<Out=CO> + Clone,
          D: Interpolate<Out=DO> + Clone,
          E: Interpolate<Out=EO> + Clone,
          F: Interpolate<Out=FO> + Clone,
          G: Interpolate<Out=GO> + Clone,
          H: Interpolate<Out=HO> + Clone {
    type Out = (AO, BO, CO, DO, EO, FO, GO, HO);
    #[inline]
    fn interpolate(src: &Triangle<(A, B, C, D, E, F, G, H)>, w: [f32; 3]) -> (AO, BO, CO, DO, EO, FO, GO, HO) {
        (Interpolate::interpolate(&Triangle::new(src.x.0.clone(), src.y.0.clone(), src.z.0.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.1.clone(), src.y.1.clone(), src.z.1.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.2.clone(), src.y.2.clone(), src.z.2.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.3.clone(), src.y.3.clone(), src.z.3.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.4.clone(), src.y.4.clone(), src.z.4.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.5.clone(), src.y.5.clone(), src.z.5.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.6.clone(), src.y.6.clone(), src.z.6.clone()), w),
         Interpolate::interpolate(&Triangle::new(src.x.7.clone(), src.y.7.clone(), src.z.7.clone()), w))
    }
}

