//! "Scene" refers to the intitialization step at which these are registered. Everything defined here will be accessible both in debug/editor runtimes as well as in release/standalone builds.
pub mod camera;
pub mod dialog;
pub mod error_handling;
pub mod game_globals;
pub mod game_settings;
pub mod godot_replacements;
pub mod input;
pub mod interaction;
pub mod procedural_meshes;
pub mod serialization;
pub mod signals;
pub mod state_machine;
pub mod utility_nodes;
pub mod vfx_stack;
pub mod gui;

pub fn register_singletons() {
    game_globals::register_singleton();
    dialog::register_singleton();
}

pub fn unregister_singletons() {
    game_globals::unregister_singleton();
    dialog::unregister_singleton();
}
