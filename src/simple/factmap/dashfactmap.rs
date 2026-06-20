use std::sync::{Mutex, RwLock};

use dashmap::DashMap;

use crate::errors::NewFactError;
use crate::fact::FactId;
use crate::simple::factmap::FactMap;

#[derive(Debug)]
pub struct DashFactMap {
	lock: Mutex<()>,
	lut: RwLock<Vec<String>>,
	map: DashMap<String, FactId>,
}

impl FactMap for DashFactMap {
	fn new() -> Self {
		Self {
			lock: Mutex::new(()),
			lut: RwLock::new(Vec::new()),
			map: DashMap::new(),
		}
	}

	fn get_fact_id(&self, fact_name: impl Into<String>) -> Option<FactId> {
		self.map.get(&fact_name.into()).as_deref().copied()
	}

	fn get_fact_names(&self) -> Vec<String> {
		self.lut.read().unwrap().clone()
	}
}

impl Default for DashFactMap {
	fn default() -> Self {
		Self::new()
	}
}

impl DashFactMap {
	/// # Errors
	/// - `NewFactError::EmptyFactName`: if the provided fact name was empty
	/// - `NewFactError::InitialDigitFactName`: if the provided fact name begins with a digit
	/// - `NewFactError::DuplicateFactName`: if the provided fact name was already used
	/// - `NewFactError::ContainsAsciiWhitespaceFactName`: if the provided fact names contains ascii whitespace
	/// - `NewFactError::OutOfFactIdSpace`: somehow you used 65k fact ids and want to keep going
	#[allow(dead_code)]
	fn new_fact(&self, fact_name: impl Into<String>) -> Result<FactId, NewFactError> {
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

		let _guard = self.lock.lock().unwrap();

		// Re-check after acquiring the lock:
		if self.map.contains_key(&fact_name) {
			return Err(NewFactError::DuplicateFactName);
		}

		let Ok(next_id) = u16::try_from(self.lut.read().unwrap().len()) else {
			return Err(NewFactError::OutOfFactIdSpace);
		};
		let fact = FactId(next_id);
		self.lut.write().unwrap().push(fact_name.clone());
		self.map.insert(fact_name, fact);

		Ok(fact)
	}
}
