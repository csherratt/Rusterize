

pub trait Dot<RHS = Self> where <Self as Dot<RHS>>::Output: Sized {
	type Output;

	fn dot(self, rhs: RHS) -> <Self as Dot<RHS>>::Output;
}
