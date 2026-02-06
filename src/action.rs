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
}
