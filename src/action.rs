use crate::condition::Condition;
use crate::cost::Cost;
use crate::effect::Effect;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Action {
	pub name: String,
	pub cost: Cost,
	pub preconditions: Vec<Condition>,
	pub effects: Vec<Effect>,
	pub executors: Vec<(String, String)>,
}

impl Action {
	pub fn new(name: impl Into<String>, cost: Cost) -> Self {
		Self {
			name: name.into(),
			cost,
			preconditions: Vec::new(),
			effects: Vec::new(),
			executors: Vec::new(),
		}
	}

	pub fn push_precondition(&mut self, precondition: Condition) -> &mut Self {
		self.preconditions.push(precondition);
		self
	}

	pub fn push_effect(&mut self, effect: Effect) -> &mut Self {
		self.effects.push(effect);
		self
	}

	pub fn push_executor(
		&mut self,
		executor_0: impl Into<String>,
		executor_1: impl Into<String>,
	) -> &mut Self {
		self.executors.push((executor_0.into(), executor_1.into()));
		self
	}

	pub fn push_executorv(&mut self, executor: impl Into<(String, String)>) -> &mut Self {
		self.executors.push(executor.into());
		self
	}
}

impl core::fmt::Display for Action {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.name)
	}
}
