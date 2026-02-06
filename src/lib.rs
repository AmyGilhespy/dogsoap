mod action;
#[cfg(feature = "bevy")]
mod bevy;
mod condition;
mod cost;
mod effect;
mod errors;
mod fact;
mod goal;
mod plan;
mod planner;
#[cfg(feature = "simple")]
mod simple;
mod value;
mod world;

pub use action::Action;
pub use condition::{Condition, conditions_met};
pub use cost::Cost;
pub use effect::Effect;
pub use errors::PlannerError;
pub use fact::FactId;
pub use goal::Goal;
pub use plan::Plan;
pub use planner::Planner;
#[cfg(feature = "simple")]
pub use simple::*;
pub use value::Value;
pub use world::WorldState;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn condition_and_effects_work() {
		let health = FactId(0);

		let start = WorldState::new(0).with_fact(health, Value::Int(10));

		let healed = start.with_effects(&[Effect::Add(health, Value::Int(5))]);

		assert!(healed.get(health).eq(&Value::Int(15), &start));

		assert!(Condition::Gt(health, Value::Int(10)).is_satisfied(&healed));
	}

	#[test]
	fn simple_goap_plan() {
		// Facts
		let has_axe = FactId(0);
		let has_wood = FactId(1);

		// Initial world state
		let start = WorldState::new(0)
			.with_fact(has_axe, Value::FALSE)
			.with_fact(has_wood, Value::FALSE);

		// Actions
		let get_axe = Action {
			name: "Get Axe".into(),
			cost: Cost(1),
			preconditions: vec![],
			effects: vec![Effect::Set(has_axe, Value::TRUE)],
		};

		let chop_wood = Action {
			name: "Chop Wood".into(),
			cost: Cost(2),
			preconditions: vec![Condition::Eq(has_axe, Value::TRUE)],
			effects: vec![Effect::Set(has_wood, Value::TRUE)],
		};

		// Goal
		let goal = Goal {
			name: "Get wood".into(),
			conditions: vec![Condition::Eq(has_wood, Value::TRUE)],
		};

		// Planner
		let mut planner = Planner::new();
		planner.add_action(get_axe);
		planner.add_action(chop_wood);

		// Plan!
		let plan = planner.plan(&start, &goal).expect("no plan found");

		// Test typical usage:
		for index in &plan.actions {
			if let Some(action) = planner.get_plan_action(&plan, *index) {
				println!("{}", action.name);
			}
		}

		// Verify plan
		assert_eq!(plan.actions.len(), 2);
		assert_eq!(
			planner
				.get_plan_action(&plan, 0)
				.expect("missing action")
				.name,
			"Get Axe"
		);
		assert_eq!(
			planner
				.get_plan_action(&plan, 1)
				.expect("missing action")
				.name,
			"Chop Wood"
		);
	}
}
