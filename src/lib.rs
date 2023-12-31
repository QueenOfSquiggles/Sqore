// crate-wide warnings alterations
#![allow(dead_code)] // a lot of elements are considered unused because Godot grabs it over FFI.

use godot::prelude::*;

// module specifications
pub mod editor;
pub mod scene;

// extension loading
struct SquigglesCore;

#[gdextension]
unsafe impl ExtensionLibrary for SquigglesCore {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            scene::editor_plugin::register_engine_elements();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            scene::editor_plugin::unregister_engine_elements();
        }
    }
}
