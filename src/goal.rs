use crate::{WorldState, condition::Condition};

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Goal {
	pub name: String,
	pub conditions: Vec<Condition>,
}

impl Goal {
	pub fn new(name: impl Into<String>) -> Self {
		Self {
			name: name.into(),
			conditions: Vec::new(),
		}
	}

	pub fn push_condition(&mut self, condition: Condition) -> &mut Self {
		self.conditions.push(condition);
		self
	}

	#[must_use]
	pub(crate) fn with_resolve_conditions_fully_resolved(&self, state: &WorldState) -> Self {
		let mut resolved = self.clone();
		for condition in &mut resolved.conditions {
			*condition = condition.as_fully_resolved(state);
		}
		resolved
	}
}

impl PartialEq for Goal {
	fn eq(&self, other: &Self) -> bool {
		self.name == other.name
	}
}

impl Eq for Goal {}

impl core::fmt::Display for Goal {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.name)
	}
}
