use thiserror::Error;

#[derive(Debug, Error)]
pub enum PlannerError {
	#[error("no plan could be found")]
	NoPlanFound,

	#[error("planner produced an unreachable state between steps")]
	UnreachableState,
}

#[derive(Debug, Error)]
pub enum NewFactError {
	#[error("the provided fact name is an empty string, which is disallowed")]
	EmptyFactName,

	#[error("the provided fact name starts with an initial digit, which is disallowed")]
	InitialDigitFactName,

	#[error("the provided fact name contains ascii whitespace, which is disallowed")]
	ContainsAsciiWhitespaceFactName,

	#[error("the provided fact name already exists")]
	DuplicateFactName,

	#[error("no remaining values for FactId exist")]
	OutOfFactIdSpace,
}

#[derive(Debug, Error)]
pub enum ParseConditionError {
	#[error("wrong field count")]
	WrongFieldCount,

	#[error("unrecognized operator")]
	UnrecognizedOperator,

	#[error("lhs is not a fact id")]
	LhsNotAFactId,

	#[error("lhs is not a fact name or id")]
	RhsNotAFactNameOrId,
}

#[derive(Debug, Error)]
pub enum ParseEffectError {
	#[error("wrong field count")]
	WrongFieldCount,

	#[error("unrecognized operator")]
	UnrecognizedOperator,

	#[error("lhs is not a fact id")]
	LhsNotAFactId,

	#[error("lhs is not a fact name or id")]
	RhsNotAFactNameOrId,
}
