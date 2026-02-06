use crate::effect::Effect;
use crate::fact::FactId;
use crate::value::Value;

/// Regarding `Eq`, `PartialEq`, and `Hash`:
/// `Value::Error` is considered equal to `Value::Error` here so that comparing hashes produces the same result as comparing values.
#[derive(Clone, Debug)] // Eq, Hash are required by astar
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct WorldState {
	pub(crate) values: Vec<Value>, // indexed by FactId
}

impl PartialEq for WorldState {
	fn eq(&self, other: &Self) -> bool {
		if self.values.len() != other.values.len() {
			return false;
		}
		self.values
			.iter()
			.zip(&other.values)
			.all(|p| p.0.eq_even_error(p.1, self))
	}
}

impl Eq for WorldState {}

impl core::hash::Hash for WorldState {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.values
			.iter()
			.map(|v| v.resolve_fully(self))
			.collect::<Vec<Value>>()
			.hash(state);
	}
}

impl WorldState {
	#[must_use]
	pub fn new(num_facts: usize) -> Self {
		Self {
			values: vec![Value::default(); num_facts],
		}
	}

	#[must_use]
	pub fn with_fact(&self, fact: FactId, value: Value) -> Self {
		let mut next = self.clone();
		let fact = usize::from(fact.0);

		if fact >= next.values.len() {
			next.values.resize(fact + 1, Value::default());
		}

		next.values[fact] = value;

		next
	}

	#[must_use]
	pub fn with_effects(&self, effects: &[Effect]) -> Self {
		let mut next = self.clone();

		for effect in effects {
			effect.apply(&mut next);
		}

		next
	}

	#[inline]
	#[must_use]
	pub fn get(&self, fact: FactId) -> Value {
		self.values[usize::from(fact.0)]
	}

	#[inline]
	pub fn set(&mut self, fact: FactId, value: Value) {
		self.values[usize::from(fact.0)] = value;
	}
}

impl Default for WorldState {
	fn default() -> Self {
		Self::new(0)
	}
}
