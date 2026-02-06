use crate::fact::FactId;
use crate::value::Value;
use crate::world::WorldState;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Effect {
	Set(FactId, Value),
	Add(FactId, Value),
	Sub(FactId, Value),
}

impl Effect {
	pub fn apply(&self, state: &mut WorldState) {
		match *self {
			Effect::Set(fact, value) => {
				state.values[usize::from(fact.0)] = value;
			}
			Effect::Add(fact, delta) => {
				let index = usize::from(fact.0);
				state.values[index] = state.values[index].add(&delta, state);
			}
			Effect::Sub(fact, delta) => {
				let index = usize::from(fact.0);
				state.values[index] = state.values[index].sub(&delta, state);
			}
		}
	}
}
