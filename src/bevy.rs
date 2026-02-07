use bevy::prelude::Component;

use crate::goal::Goal;
use crate::plan::Plan;
use crate::planner::Planner;
#[cfg(feature = "simple")]
use crate::simple::FactMap;
use crate::world::WorldState;

/// This is a work-in-progress.
#[allow(dead_code)]
#[derive(Component)]
pub struct Agent {
	pub world_state: WorldState,
	pub planner: Planner,
	#[cfg(feature = "simple")]
	pub facts: FactMap,
	pub goals: Vec<Goal>,
	pub goal_ongoing_or_failed: Option<Goal>,
	pub plan: Option<Plan>,
	pub action_index: usize,
	pub executor_index: usize,
	pub executor_sub_index: usize,
}
