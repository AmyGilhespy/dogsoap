use hashbrown::HashMap;

use crate::Condition;
use crate::errors::{NewFactError, ParseConditionError};
use crate::fact::FactId;
use crate::value::Value;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FactMap {
	lut: Vec<String>,
	map: HashMap<String, FactId>,
}

impl FactMap {
	#[must_use]
	pub fn new() -> Self {
		FactMap {
			lut: Vec::new(),
			map: HashMap::new(),
		}
	}

	/// # Errors
	/// - `NewFactError::EmptyFactName`: if the provided fact name was empty
	/// - `NewFactError::InitialDigitFactName`: if the provided fact name begins with a digit
	/// - `NewFactError::DuplicateFactName`: if the provided fact name was already used
	/// - `NewFactError::ContainsAsciiWhitespaceFactName`: if the provided fact names contains ascii whitespace
	/// - `NewFactError::OutOfFactIdSpace`: somehow you used 65k fact ids and want to keep going
	pub fn new_fact(&mut self, fact_name: impl Into<String>) -> Result<FactId, NewFactError> {
		let fact_name = fact_name.into();
		let Some(ch0) = fact_name.as_bytes().first() else {
			return Err(NewFactError::EmptyFactName);
		};
		let ch0 = char::from(*ch0);
		if ch0.is_ascii_digit() {
			return Err(NewFactError::InitialDigitFactName);
		}
		if fact_name.contains(' ') {
			return Err(NewFactError::ContainsAsciiWhitespaceFactName);
		}
		if self.map.contains_key(&fact_name) {
			return Err(NewFactError::DuplicateFactName);
		}
		let Ok(next_id) = u16::try_from(self.lut.len()) else {
			return Err(NewFactError::OutOfFactIdSpace);
		};
		let fact = FactId(next_id);
		self.lut.push(fact_name.clone());
		self.map.insert(fact_name, fact);
		Ok(fact)
	}

	#[must_use]
	pub fn get_fact_id(&self, fact_name: impl Into<String>) -> Option<FactId> {
		self.map.get(&fact_name.into()).copied()
	}

	#[must_use]
	pub fn get_fact_names(&self) -> &Vec<String> {
		&self.lut
	}

	#[must_use]
	pub fn get_map(&self) -> &HashMap<String, FactId> {
		&self.map
	}

	/// Parse a value string such as:
	/// `"3"` => `Value::Int(3)`
	/// `"my_fact"` => `Value::Ref(my_fact)`
	/// `"123 invalid"` => `Value::Error`
	#[must_use]
	pub fn parse_value(&self, string: impl Into<String>) -> Value {
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
	pub fn parse_condition(
		&self,
		string: impl Into<String>,
	) -> Result<Condition, ParseConditionError> {
		let string = string.into();
		let fields = string
			.as_str()
			.split_ascii_whitespace()
			.collect::<Vec<&str>>();
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

impl Default for FactMap {
	fn default() -> Self {
		Self::new()
	}
}
