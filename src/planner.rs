use pathfinding::prelude::astar;

use crate::action::Action;
use crate::condition::conditions_met;
use crate::cost::Cost;
use crate::errors::PlannerError;
use crate::goal::Goal;
use crate::plan::Plan;
use crate::world::WorldState;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Planner {
	actions: Vec<Action>,
}

impl Planner {
	#[must_use]
	pub fn new() -> Self {
		Self {
			actions: Vec::new(),
		}
	}

	pub fn add_action(&mut self, action: Action) {
		self.actions.push(action);
	}

	/// # Errors
	/// - `PlannerError.NoPlanFound`: If no plan is found
	/// - `PlannerError.UnreachableState`: If the planner produced an unreachable state between steps
	pub fn plan(&self, start: &WorldState, goal: &Goal) -> Result<Plan, PlannerError> {
		// run A* / Dijkstra
		let result = astar(
			start,
			|state| self.successors(state),
			|_| Cost(0),
			|state| conditions_met(&goal.conditions, state),
		);

		let (path, cost) = result.ok_or(PlannerError::NoPlanFound)?;

		let mut action_indices = Vec::new();

		for window in path.windows(2) {
			let from = &window[0];
			let to = &window[1];

			let action_index = self
				.actions
				.iter()
				.enumerate()
				.find_map(|(i, action)| {
					if conditions_met(&action.preconditions, from) {
						let next = from.with_effects(&action.effects);
						if &next == to { Some(i) } else { None }
					} else {
						None
					}
				})
				.ok_or(PlannerError::UnreachableState)?;

			action_indices.push(action_index);
		}

		Ok(Plan {
			actions: action_indices,
			total_cost: cost,
		})
	}

	#[must_use]
	pub fn get_plan_action(&self, plan: &Plan, index: usize) -> Option<&Action> {
		if let Some(action_index) = plan.actions.get(index) {
			self.actions.get(*action_index)
		} else {
			None
		}
	}

	fn successors(&self, state: &WorldState) -> Vec<(WorldState, Cost)> {
		let mut result = Vec::new();

		for action in &self.actions {
			if conditions_met(&action.preconditions, state) {
				let next_state = state.with_effects(&action.effects);
				result.push((next_state, action.cost));
			}
		}

		result
	}
}

impl Default for Planner {
	fn default() -> Self {
		Self::new()
	}
}
