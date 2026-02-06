use crate::plan::Plan;
use crate::world::WorldState;

#[derive(Component)]
pub struct GoapAgent {
	pub world: WorldState,
	pub current_plan: Option<Plan>,
}
