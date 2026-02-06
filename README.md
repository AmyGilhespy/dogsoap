# Dog Soap

Dog Soap (Data Oriented, Simple, Goal-Oriented Action Planning) is a dirt-simple GOAP implementation in pure Rust.

# Code Example

```rust
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
	conditions: vec![Condition::Eq(has_wood, Value::TRUE)],
};

// Planner
let mut planner = Planner::new();
planner.add_action(get_axe);
planner.add_action(chop_wood);

// Plan!
let plan = planner.plan(&start, &goal).expect("no plan found");

// Use the plan
for index in &plan.actions {
	if let Some(action) = planner.get_plan_action(&plan, *index) {
		println!("{}", action.name);
	}
}
```

# License

Bevy Flair HTML Extension is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

    MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
    Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

# Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
