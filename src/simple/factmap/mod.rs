use crate::{
	Condition, Effect, FactId, Value,
	errors::{NewFactError, ParseConditionError, ParseEffectError},
};

#[cfg(feature = "dashmap")]
pub mod dashfactmap;

pub mod hashfactmap;

pub trait FactMap {
	#[must_use]
	fn new() -> Self;

	/// # Errors
	/// - `NewFactError::EmptyFactName`: if the provided fact name was empty
	/// - `NewFactError::InitialDigitFactName`: if the provided fact name begins with a digit
	/// - `NewFactError::DuplicateFactName`: if the provided fact name was already used
	/// - `NewFactError::ContainsAsciiWhitespaceFactName`: if the provided fact names contains ascii whitespace
	/// - `NewFactError::OutOfFactIdSpace`: somehow you used 65k fact ids and want to keep going
	fn new_fact(&mut self, fact_name: impl Into<String>) -> Result<FactId, NewFactError>;

	#[must_use]
	fn get_fact_id(&self, fact_name: impl Into<String>) -> Option<FactId>;

	#[must_use]
	fn get_fact_names(&self) -> &Vec<String>;

	/// Parse a value string such as:
	/// `"3"` => `Value::Int(3)`
	/// `"my_fact"` => `Value::Ref(my_fact)`
	/// `"123 invalid"` => `Value::Error`
	fn parse_value(&self, string: impl Into<String>) -> Value {
		let string = string.into();
		match string.parse::<i64>() {
			Ok(int) => Value::Int(int),
			Err(_) => {
				if let Some(fact_id) = self.get_fact_id(string) {
					Value::Ref(fact_id)
				} else {
					Value::Error
				}
			}
		}
	}

	/// Parse a condition string such as:
	/// `"my_fact == 3"` => `Condition::Eq(my_fact, Value::Int(3))`
	/// `"my_fact > my_other_fact"` => `Condition::Eq(my_fact, Value::Ref(my_other_fact))`
	///
	/// # Errors
	/// - `ParseConditionError::WrongFieldCount` if the string does not split into exactly `3` fields (ascii spacing as separators)
	/// - `ParseConditionError::LhsNotAFactId` if the lhs is not a fact id
	/// - `ParseConditionError::RhsNotAFactNameOrId` if parsing the rhs with `parse_value` returns `Value::Error` (it wasn't an `i64` or fact id)
	/// - `ParseConditionError::UnrecognizedOperator` if the middle field is not one of `"=="`, `"!="`, `"<"`, `">"`, `"<="`, or `">="`
	fn parse_condition(&self, string: impl Into<String>) -> Result<Condition, ParseConditionError> {
		let string = string.into();
		let mut fields = string
			.as_str()
			.split_ascii_whitespace()
			.collect::<Vec<&str>>();
		let resolve = fields.len() == 4 && fields[2] == "resolve";
		if resolve {
			fields.remove(2);
		}
		let fields = fields;
		if fields.len() != 3 {
			return Err(ParseConditionError::WrongFieldCount);
		}
		let Some(lhs) = self.get_fact_id(fields[0]) else {
			return Err(ParseConditionError::LhsNotAFactId);
		};
		let rhs = self.parse_value(fields[2]);
		if rhs.is_error() {
			return Err(ParseConditionError::RhsNotAFactNameOrId);
		}
		if resolve {
			match fields[1] {
				"==" => Ok(Condition::EqResolve(lhs, rhs)),
				"!=" => Ok(Condition::NeResolve(lhs, rhs)),
				">" => Ok(Condition::GtResolve(lhs, rhs)),
				"<" => Ok(Condition::LtResolve(lhs, rhs)),
				">=" => Ok(Condition::GeResolve(lhs, rhs)),
				"<=" => Ok(Condition::LeResolve(lhs, rhs)),
				_ => Err(ParseConditionError::UnrecognizedOperator),
			}
		} else {
			match fields[1] {
				"==" => Ok(Condition::Eq(lhs, rhs)),
				"!=" => Ok(Condition::Ne(lhs, rhs)),
				">" => Ok(Condition::Gt(lhs, rhs)),
				"<" => Ok(Condition::Lt(lhs, rhs)),
				">=" => Ok(Condition::Ge(lhs, rhs)),
				"<=" => Ok(Condition::Le(lhs, rhs)),
				_ => Err(ParseConditionError::UnrecognizedOperator),
			}
		}
	}

	/// Parse an effect string such as:
	/// `"my_fact = 3"` => `Effect::Set(my_fact, Value::Int(3))`
	/// `"my_fact += my_other_fact"` => `Effect::Add(my_fact, Value::Ref(my_other_fact))`
	///
	/// # Errors
	/// - `ParseEffectError::WrongFieldCount` if the string does not split into exactly `3` fields (ascii spacing as separators)
	/// - `ParseEffectError::LhsNotAFactId` if the lhs is not a fact id
	/// - `ParseEffectError::RhsNotAFactNameOrId` if parsing the rhs with `parse_value` returns `Value::Error` (it wasn't an `i64` or fact id)
	/// - `ParseEffectError::UnrecognizedOperator` if the middle field is not one of `"="`, `"+="`, or `"-="`
	fn parse_effect(&self, string: impl Into<String>) -> Result<Effect, ParseEffectError> {
		let string = string.into();
		let fields = string
			.as_str()
			.split_ascii_whitespace()
			.collect::<Vec<&str>>();
		if fields.len() != 3 {
			return Err(ParseEffectError::WrongFieldCount);
		}
		let Some(lhs) = self.get_fact_id(fields[0]) else {
			return Err(ParseEffectError::LhsNotAFactId);
		};
		let rhs = self.parse_value(fields[2]);
		if rhs.is_error() {
			return Err(ParseEffectError::RhsNotAFactNameOrId);
		}
		match fields[1] {
			"=" => Ok(Effect::Set(lhs, rhs)),
			"+=" => Ok(Effect::Add(lhs, rhs)),
			"-=" => Ok(Effect::Sub(lhs, rhs)),
			_ => Err(ParseEffectError::UnrecognizedOperator),
		}
	}
}
