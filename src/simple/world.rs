use std::sync::LazyLock;

use crate::{State, simple::factmap::dashfactmap::DashFactMap};

/// The global facts as they pertain to the world.
pub static WORLD_FACTS: LazyLock<DashFactMap> = LazyLock::new(DashFactMap::default);

/// The global state of the world.
pub static WORLD_STATE: LazyLock<State> = LazyLock::new(State::default);
