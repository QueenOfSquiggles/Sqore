use godot::{engine::*, prelude::*};
use once_cell::sync::Lazy;

use crate::game_globals::CoreGlobals;

pub static SINGLETON_CORE_GLOBALS: Lazy<StringName> = Lazy::new(|| StringName::from("CoreGlobals"));

pub fn register_engine_elements() {
    Engine::singleton().register_singleton(
        SINGLETON_CORE_GLOBALS.clone(),
        CoreGlobals::alloc_gd().upcast(),
    );
}

pub fn unregister_engine_elements() {
    Engine::singleton().unregister_singleton(SINGLETON_CORE_GLOBALS.clone());
}
