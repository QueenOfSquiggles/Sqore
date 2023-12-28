// crate-wide warnings alterations
#![allow(dead_code)] // a lot of elements are considered unused because Godot grabs it over FFI.

use godot::prelude::*;

// module specifications
pub mod camera;
pub mod editor_plugin;
pub mod editor_utils;
pub mod error_handling;
pub mod game_globals;
pub mod game_settings;
pub mod godot_replacements;
pub mod input;
pub mod interaction;
pub mod serialization;
pub mod signals;
pub mod state_machine;
pub mod utility_nodes;
pub mod vfx_stack;

// extension loading
struct SquigglesCore;

#[gdextension]
unsafe impl ExtensionLibrary for SquigglesCore {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => editor_plugin::register_engine_elements(),
            InitLevel::Editor => editor_utils::register_editor_elements(),
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => editor_plugin::unregister_engine_elements(),
            InitLevel::Editor => editor_utils::unregister_editor_elements(),
            _ => (),
        }
    }
}
