use pathfinding::prelude::astar;

use crate::action::Action;
use crate::condition::conditions_met;
use crate::cost::Cost;
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

	/// # Panics
	/// If the Planner produces an unreachable state.
	#[must_use]
	pub fn plan(&self, start: &WorldState, goal: &Goal) -> Option<Plan> {
		// run A* / Dijkstra
		let result = astar(
			start,
			|state| self.successors(state),
			|_| Cost(0),
			|state| conditions_met(&goal.conditions, state),
		);

		result.map(|(path, cost)| {
			let mut action_indices = Vec::new();

			// path is a Vec<WorldState>
			// we need to reconstruct which action led to each state
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
					.expect("planner produced an unreachable state");

				action_indices.push(action_index);
			}

			Plan {
				actions: action_indices,
				total_cost: cost,
			}
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
