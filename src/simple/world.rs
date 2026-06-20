use std::sync::{LazyLock, RwLock};

use crate::{State, simple::factmap::dashfactmap::DashFactMap};

/// The global facts as they pertain to the world.
pub static WORLD_FACTS: LazyLock<DashFactMap> = LazyLock::new(DashFactMap::default);

/// The global state of the world.
pub static WORLD_STATE: LazyLock<RwLock<State>> = LazyLock::new(|| RwLock::new(State::default()));
