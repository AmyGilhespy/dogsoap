use crate::fact::FactId;
use crate::value::Value;
use crate::world::WorldState;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Condition {
	Eq(FactId, Value),
	Ne(FactId, Value),
	Gt(FactId, Value),
	Lt(FactId, Value),
	Ge(FactId, Value),
	Le(FactId, Value),
}

impl Condition {
	#[must_use]
	pub fn is_satisfied(&self, state: &WorldState) -> bool {
		match *self {
			Condition::Eq(fact, value) => state.get(fact) == value,
			Condition::Ne(fact, value) => state.get(fact) != value,
			Condition::Gt(fact, value) => state.get(fact) > value,
			Condition::Lt(fact, value) => state.get(fact) < value,
			Condition::Ge(fact, value) => state.get(fact) >= value,
			Condition::Le(fact, value) => state.get(fact) <= value,
		}
	}
}

#[must_use]
pub fn conditions_met(conditions: &[Condition], state: &WorldState) -> bool {
	conditions.iter().all(|cond| cond.is_satisfied(state))
}
