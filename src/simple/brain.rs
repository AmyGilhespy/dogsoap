use std::collections::HashMap;

use crate::{
	State, Value,
	simple::factmap::{FactMap, hashfactmap::HashFactMap},
};

#[cfg(feature = "dashmap")]
use crate::simple::world::{WORLD_FACTS, WORLD_STATE};

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Brain {
	/// Map of stats that pertain to the agent
	pub stats: HashFactMap,

	/// State of the agent's stats, i.e. things that are factual about the agent
	pub stat_state: State,

	/// Map of thoughts that pertain to the agent
	pub thoughts: HashFactMap,

	/// State of the agent's thoughts, i.e. things that the agent believes to be true
	pub thought_state: State,

	/// Map from thought names to expressions that update them each ai tick
	#[cfg(feature = "fasteval")]
	pub thought_updater: HashMap<String, String>,
}

impl Brain {
	#[must_use]
	pub fn new(num_stats: usize, num_thoughts: usize) -> Self {
		Self {
			stats: HashFactMap::new(),
			stat_state: State::new(num_stats),

			thoughts: HashFactMap::new(),
			thought_state: State::new(num_thoughts),

			#[cfg(feature = "fasteval")]
			thought_updater: HashMap::new(),
		}
	}

	#[cfg(feature = "fasteval")]
	pub fn update_thoughts<'a, F>(&mut self, fallback: Option<F>)
	where
		F: FnMut(&Self, &str, Vec<f64>) -> Option<f64> + 'a,
	{
		let mut fallback = fallback;
		for (thought_name, expr) in &self.thought_updater {
			let Some(thought_id) = self.thoughts.get_fact_id(thought_name) else {
				continue;
			};
			let mut ns =
				fasteval::CachedCallbackNamespace::new(|variable_name: &str, args: Vec<f64>| {
					self.lookup_expr_variable(variable_name).or_else(|| {
						if let Some(ref mut flbk) = fallback {
							flbk(self, variable_name, args)
						} else {
							None
						}
					})
				});
			#[allow(clippy::cast_possible_truncation)]
			let value_option = fasteval::ez_eval(expr, &mut ns)
				.ok()
				.map(|f| Value::Int(f.round() as i64));
			drop(ns);
			if let Some(value) = value_option {
				self.thought_state.set(thought_id, value);
			}
		}
	}

	fn lookup_expr_variable(&self, variable_name: &str) -> Option<f64> {
		if let Some((domain, fact_name)) = variable_name.split_once('.') {
			match domain {
				"thought" => self.thoughts.get_fact_id(fact_name).and_then(|thought_id| {
					#[allow(clippy::cast_precision_loss)]
					self.thought_state
						.get(thought_id)
						.resolve_fully(&self.thought_state)
						.int()
						.map(|i| i as f64)
				}),

				"stat" => self.stats.get_fact_id(fact_name).and_then(|stat_id| {
					#[allow(clippy::cast_precision_loss)]
					self.stat_state
						.get(stat_id)
						.resolve_fully(&self.stat_state)
						.int()
						.map(|i| i as f64)
				}),

				#[cfg(feature = "dashmap")]
				"world" => WORLD_FACTS
					.get_fact_id(fact_name)
					.and_then(|world_fact_id| {
						let state = WORLD_STATE.read().unwrap();
						#[allow(clippy::cast_precision_loss)]
						state
							.get(world_fact_id)
							.resolve_fully(&state)
							.int()
							.map(|i| i as f64)
					}),

				_ => None,
			}
		} else {
			None
		}
	}
}

impl Default for Brain {
	fn default() -> Self {
		Self::new(0, 0)
	}
}
