use crate::fact::FactId;
use crate::value::Value;
use crate::world::WorldState;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Effect {
	Add(FactId, Value),
	Set(FactId, Value),
}

impl Effect {
	pub fn apply(&self, state: &mut WorldState) {
		match *self {
			Effect::Add(fact, delta) => {
				state.values[usize::from(fact.0)] += delta;
			}
			Effect::Set(fact, value) => {
				state.values[usize::from(fact.0)] = value;
			}
		}
	}
}
