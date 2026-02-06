use core::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Cost(pub i32);

impl Add for Cost {
	type Output = Self;

	fn add(self, rhs: Self) -> Self {
		Self(self.0 + rhs.0)
	}
}

impl AddAssign for Cost {
	fn add_assign(&mut self, rhs: Self) {
		self.0 += rhs.0;
	}
}

impl pathfinding::num_traits::Zero for Cost {
	fn zero() -> Self {
		Self(0)
	}

	fn is_zero(&self) -> bool {
		self.0 == 0
	}
}
