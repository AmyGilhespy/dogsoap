use core::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Value(pub i32);

impl Value {
	pub const FALSE: Value = Value(0);
	pub const TRUE: Value = Value(1);
}

impl Add for Value {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self(self.0 + rhs.0)
	}
}

impl AddAssign for Value {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
	}
}

impl pathfinding::num_traits::Zero for Value {
	fn zero() -> Self {
		Self(0)
	}

	fn is_zero(&self) -> bool {
		self.0 == 0
	}
}
