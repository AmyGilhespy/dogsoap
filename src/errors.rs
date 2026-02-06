use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlannerError {
	#[error("no plan could be found")]
	NoPlanFound,

	#[error("planner produced an unreachable state between steps")]
	UnreachableState,
}
