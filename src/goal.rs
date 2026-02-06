use crate::condition::Condition;

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
