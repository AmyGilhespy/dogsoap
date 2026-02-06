use crate::cost::Cost;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Plan {
	pub actions: Vec<usize>, // indices into Planner.actions
	pub total_cost: Cost,
}
