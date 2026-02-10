use pathfinding::prelude::astar;

use crate::action::Action;
use crate::condition::conditions_met;
use crate::cost::Cost;
use crate::errors::PlannerError;
use crate::goal::Goal;
use crate::plan::Plan;
use crate::world::WorldState;

#[derive(Clone, Debug)]
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

	pub fn push_action(&mut self, action: Action) {
		self.actions.push(action);
	}

	/// # Errors
	/// - `PlannerError.NoPlanFound`: If no plan is found
	/// - `PlannerError.UnreachableState`: If the planner produced an unreachable state between steps
	pub fn plan(
		&self,
		start: &WorldState,
		goal: &Goal,
		max_actions: usize,
	) -> Result<Plan, PlannerError> {
		let goal = goal.with_resolve_conditions_fully_resolved(start);

		// run A* / Dijkstra
		let result = astar(
			&(start.clone(), 0),
			|state| self.successors(state, max_actions),
			|_| Cost(0),
			|state| conditions_met(&goal.conditions, &state.0),
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
					if conditions_met(&action.preconditions, &from.0) {
						let next = from.0.with_effects(&action.effects);
						if next == to.0 { Some(i) } else { None }
					} else {
						None
					}
				})
				.ok_or(PlannerError::UnreachableState)?;

			action_indices.push(action_index);
		}

		Ok(Plan {
			total_cost: cost,
			action_indices,
		})
	}

	#[must_use]
	pub fn get_plan_action(&self, plan: &Plan, index: usize) -> Option<&Action> {
		if let Some(action_index) = plan.action_indices.get(index) {
			self.actions.get(*action_index)
		} else {
			None
		}
	}

	fn successors(
		&self,
		state: &(WorldState, usize),
		max_actions: usize,
	) -> Vec<((WorldState, usize), Cost)> {
		let mut result = Vec::new();

		if state.1 < max_actions {
			for action in &self.actions {
				if conditions_met(&action.preconditions, &state.0) {
					let next_state = (state.0.with_effects(&action.effects), state.1 + 1);
					result.push((next_state, action.cost));
				}
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
