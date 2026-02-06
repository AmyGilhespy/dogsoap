use crate::condition::Condition;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Goal {
	pub conditions: Vec<Condition>,
}
