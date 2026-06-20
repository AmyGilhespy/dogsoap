use crate::fact::FactId;
use crate::state::State;
use crate::value::Value;

#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "nanoserde", derive(nanoserde::DeRon))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Effect {
	Set(FactId, Value),
	Add(FactId, Value),
	Sub(FactId, Value),
}

impl Effect {
	pub fn apply(&self, state: &mut State) {
		match *self {
			Effect::Set(fact, value) => {
				state.set(fact, value);
			}
			Effect::Add(fact, delta) => {
				state.add(fact, delta);
			}
			Effect::Sub(fact, delta) => {
				state.sub(fact, delta);
			}
		}
	}
}
