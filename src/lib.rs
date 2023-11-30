// crate-wide warnings alterations
#![allow(dead_code)] // a lot of elements are considered unused because Godot grabs it over FFI.

use godot::prelude::*;

// module specifications
pub mod camera;
pub mod editor_plugin;
pub mod error_handling;
pub mod game_globals;
pub mod game_settings;
pub mod godot_replacements;
pub mod interaction;
pub mod serialization;
pub mod state_machine;
pub mod utility_nodes;

// extension loading
struct SquigglesCore;

#[gdextension]
unsafe impl ExtensionLibrary for SquigglesCore {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            editor_plugin::register_engine_elements();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            editor_plugin::unregister_engine_elements();
        }
    }
}
