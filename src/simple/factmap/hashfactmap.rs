use hashbrown::HashMap;

use crate::errors::NewFactError;
use crate::fact::FactId;
use crate::simple::factmap::FactMap;

#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct HashFactMap {
	lut: Vec<String>,
	map: HashMap<String, FactId>,
}

impl FactMap for HashFactMap {
	fn new() -> Self {
		Self {
			lut: Vec::new(),
			map: HashMap::new(),
		}
	}

	fn get_fact_id(&self, fact_name: impl Into<String>) -> Option<FactId> {
		self.map.get(&fact_name.into()).copied()
	}

	fn get_fact_names(&self) -> Vec<String> {
		self.lut.clone()
	}
}

impl Default for HashFactMap {
	fn default() -> Self {
		Self::new()
	}
}

impl HashFactMap {
	#[allow(dead_code)]
	#[must_use]
	pub fn get_map(&self) -> &HashMap<String, FactId> {
		&self.map
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
}
