# Dog Soap

Dog Soap (Data Oriented, Simple, Goal-Oriented Action Planning) is a dirt-simple GOAP implementation in pure Rust.

# Code Example

```rust
// Facts (...but consider using a FactMap!)
let has_axe = FactId(0);
let has_wood = FactId(1);

// Initial world state
let mut start = WorldState::new(0);
start.push_fact(has_axe, Value::FALSE);
start.push_fact(has_wood, Value::FALSE);

// Actions
let get_axe = Action {
	name: "Get Axe".into(),
	cost: Cost(1),
	preconditions: vec![],
	effects: vec![Effect::Set(has_axe, Value::TRUE)],
	executors: vec![],
};

let chop_wood = Action {
	name: "Chop Wood".into(),
	cost: Cost(2),
	preconditions: vec![Condition::Eq(has_axe, Value::TRUE)],
	effects: vec![Effect::Set(has_wood, Value::TRUE)],
	executors: vec![],
};

// Goal
let mut goal = Goal::new("Get wood");
goal.push_condition(Condition::Eq(has_wood, Value::TRUE));

// Planner
let mut planner = Planner::new();
planner.push_action(get_axe);
planner.push_action(chop_wood);

// Plan!
let plan = planner.plan(&start, &goal).expect("no plan found");

// Use the plan
for index in &plan.actions {
	if let Some(action) = planner.get_plan_action(&plan, *index) {
		println!("{}", action.name);
	}
}
```

You can use a FactMap to simplify the parsing of a data file (JSON, ron, etc):

```rust
let json = r#"
{
	"name_en": "Deer",
	"desc_en": "A common prey animal.",
	"facts": [
		"satiety", // satiety := fullness; opposite of hunger
		"can_see_threat",
		"is_threatened",
		"has_escape_route",
		"energy_level",
		"wakefulness"
	],
	"initial_state": { // Anything not listed defaults to 0
		"satiety": 50,
		"energy_level": 25,
		"wakefulness": 75
	},
	"goals": [
		["Avoid Danger", ["is_threatened == 0"]],
		["Eat",          ["satiety >= 75"]],
		["Sleep",        ["wakefulness >= 75"]]
	],
	"actions": [
		{
				"name": "Wander Aimlessly",
				"cost": 1,
				"preconditions": ["is_threatened == 0"],
				"effects": []
		},
		{
				"name": "Search for Escape Route",
				"cost": 2,
				"preconditions": [],
				"effects": ["has_escape_route = 1"]
		},
		{
				"name": "Graze",
				"cost": 3,
				"preconditions": ["is_threatened == 0", "can_see_threat == 0"],
				"effects": ["energy_level += 1", "satiety += 3"]
		},
		{
				"name": "Sleep",
				"cost": 10,
				"preconditions": ["is_threatened == 0", "can_see_threat == 0"],
				"effects": ["energy_level += 1", "wakefulness += 3"]
		},
		{
				"name": "Flee",
				"cost": 15,
				"preconditions": ["has_escape_route == 1"],
				"effects": ["is_threatened = 0", "can_see_threat = 0"]
		}
	]
}
"#;

// Read the data
let template: NpcTemplate = serde_json::from_str(json)?;

// Facts
let mut facts = FactMap::new();
for fact_name in template.facts {
	if let Err(err) = facts.new_fact(&fact_name) {
		error!("Failed to create new fact \"{fact_name}\": {err}");
	}
}

// Initial World State
let mut world_state = WorldState::default();
for kv in template.initial_state {
	let Some(fact_id) = facts.get_fact_id(&kv.0) else {
		error!("Reference to undefined fact id \"{}\".", kv.0);
		continue;
	};
	world_state = world_state.with_fact(fact_id, Value::Int(kv.1));
}

// Goals
let mut goals = Vec::new();
for g in &template.goals {
	let mut goal = Goal::new(&g.0);
	for condition in &g.1 {
		match facts.parse_condition(condition) {
			Ok(cond) => {
				goal.push_condition(cond);
			}
			Err(err) => {
				error!("Failed to parse NPC goal condition: \"{condition}\": {err}");
				continue;
			}
		}
	}
	goals.push(goal);
}

// Planner (Actions & Events)
let mut planner = Planner::new();
for a in &template.actions {
	let mut action = Action::new(&a.name, Cost(a.cost));
	for precondition in &a.preconditions {
		match facts.parse_condition(precondition) {
			Ok(cond) => {
				action.push_precondition(cond);
			}
			Err(err) => {
				error!(
					"Failed to parse NPC action precondition: \"{precondition}\": {err}"
				);
				continue;
			}
		}
	}
	for effect in &a.effects {
		match facts.parse_effect(effect) {
			Ok(ef) => {
				action.push_effect(ef);
			}
			Err(err) => {
				error!("Failed to parse NPC action effect: \"{effect}\": {err}");
				continue;
			}
		}
	}
	for executor in &a.executors {
		action.push_executor(&executor.0, &executor.1);
	}
	planner.push_action(action);
}

// Construct your NPC
let deer = Npc {
	world_state,
	planner: ,
	facts,
	goals,
};

// Then, later:
let plan = deer.planner.plan(&deer.world_state, &deer.goals[0]).expect("no plan found");
```

# License

Dog Soap is free, open source and permissively licensed! Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

    	MIT License (LICENSE-MIT or http://opensource.org/licenses/MIT)
    	Apache License, Version 2.0 (LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0)

at your option. This means you can select the license you prefer! This dual-licensing approach is the de-facto standard in the Rust ecosystem and there are very good reasons to include both.

# Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
