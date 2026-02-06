use crate::fact::FactId;
use crate::world::WorldState;

#[derive(Clone, Copy, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Value {
	Int(i64),
	Ref(FactId),
	Error,
}

impl Value {
	pub const FALSE: Value = Value::Int(0);
	pub const TRUE: Value = Value::Int(1);

	#[must_use]
	pub fn is_ref(&self) -> bool {
		matches!(self, Value::Ref(_))
	}

	#[must_use]
	pub fn is_error(&self) -> bool {
		matches!(self, Value::Error)
	}

	#[must_use]
	pub fn resolve(&self, state: &WorldState) -> Value {
		match self {
			Value::Ref(f) => state.get(*f),
			_ => *self,
		}
	}

	#[must_use]
	pub fn resolve_fully(&self, state: &WorldState) -> Value {
		let mut val = *self;
		while let Value::Ref(f) = val {
			val = state.get(f);
		}
		val
	}

	#[must_use]
	pub fn eq_even_error(&self, other: &Self, state: &WorldState) -> bool {
		if let Value::Int(lhs) = self.resolve_fully(state)
			&& let Value::Int(rhs) = other.resolve_fully(state)
		{
			lhs == rhs
		} else {
			false
		}
	}

	#[must_use]
	pub fn eq(&self, other: &Self, state: &WorldState) -> bool {
		if self.is_error() || other.is_error() {
			return false;
		}
		self.eq_even_error(other, state)
	}

	#[must_use]
	pub fn ne(&self, other: &Self, state: &WorldState) -> bool {
		if self.is_error() || other.is_error() {
			return false;
		}
		!self.eq(other, state)
	}

	#[must_use]
	pub fn lt(&self, other: &Self, state: &WorldState) -> bool {
		if self.is_error() || other.is_error() {
			return false;
		}
		if let Value::Int(lhs) = self.resolve_fully(state)
			&& let Value::Int(rhs) = other.resolve_fully(state)
		{
			lhs < rhs
		} else {
			false
		}
	}

	#[must_use]
	pub fn gt(&self, other: &Self, state: &WorldState) -> bool {
		if self.is_error() || other.is_error() {
			return false;
		}
		if let Value::Int(lhs) = self.resolve_fully(state)
			&& let Value::Int(rhs) = other.resolve_fully(state)
		{
			lhs > rhs
		} else {
			false
		}
	}

	#[must_use]
	pub fn le(&self, other: &Self, state: &WorldState) -> bool {
		if self.is_error() || other.is_error() {
			return false;
		}
		!self.gt(other, state)
	}

	#[must_use]
	pub fn ge(&self, other: &Self, state: &WorldState) -> bool {
		if self.is_error() || other.is_error() {
			return false;
		}
		!self.lt(other, state)
	}

	#[must_use]
	pub fn add(&self, other: &Self, state: &WorldState) -> Value {
		if let Value::Int(lhs) = self.resolve_fully(state)
			&& let Value::Int(rhs) = other.resolve_fully(state)
		{
			Value::Int(lhs + rhs)
		} else {
			Value::Error
		}
	}

	pub fn add_assign(&mut self, other: &Self, state: &WorldState) {
		*self = self.add(other, state);
	}
}

impl Default for Value {
	fn default() -> Self {
		Value::Int(0)
	}
}
