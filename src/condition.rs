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
	EqResolve(FactId, Value),
	NeResolve(FactId, Value),
	GtResolve(FactId, Value),
	LtResolve(FactId, Value),
	GeResolve(FactId, Value),
	LeResolve(FactId, Value),
}

impl Condition {
	#[must_use]
	pub fn is_satisfied(&self, state: &WorldState) -> bool {
		match *self {
			Condition::Eq(fact, value) | Condition::EqResolve(fact, value) => {
				state.get(fact).eq(&value, state)
			}
			Condition::Ne(fact, value) | Condition::NeResolve(fact, value) => {
				state.get(fact).ne(&value, state)
			}
			Condition::Gt(fact, value) | Condition::GtResolve(fact, value) => {
				state.get(fact).gt(&value, state)
			}
			Condition::Lt(fact, value) | Condition::LtResolve(fact, value) => {
				state.get(fact).lt(&value, state)
			}
			Condition::Ge(fact, value) | Condition::GeResolve(fact, value) => {
				state.get(fact).ge(&value, state)
			}
			Condition::Le(fact, value) | Condition::LeResolve(fact, value) => {
				state.get(fact).le(&value, state)
			}
		}
	}

	#[must_use]
	pub(crate) fn as_fully_resolved(&self, state: &WorldState) -> Condition {
		match *self {
			Condition::EqResolve(fact, value) => Condition::Eq(fact, value.resolve_fully(state)),
			Condition::NeResolve(fact, value) => Condition::Ne(fact, value.resolve_fully(state)),
			Condition::GtResolve(fact, value) => Condition::Gt(fact, value.resolve_fully(state)),
			Condition::LtResolve(fact, value) => Condition::Lt(fact, value.resolve_fully(state)),
			Condition::GeResolve(fact, value) => Condition::Ge(fact, value.resolve_fully(state)),
			Condition::LeResolve(fact, value) => Condition::Le(fact, value.resolve_fully(state)),
			_ => *self,
		}
	}
}

#[must_use]
pub fn conditions_met(conditions: &[Condition], state: &WorldState) -> bool {
	conditions.iter().all(|cond| cond.is_satisfied(state))
}
