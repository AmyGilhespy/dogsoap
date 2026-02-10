use crate::cost::Cost;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Plan {
	pub total_cost: Cost,
	pub(crate) action_indices: Vec<usize>, // indices into Planner.actions
}

impl Plan {
	#[must_use]
	pub fn is_empty(&self) -> bool {
		self.action_indices.is_empty()
	}

	#[must_use]
	pub fn len(&self) -> usize {
		self.action_indices.len()
	}
}
