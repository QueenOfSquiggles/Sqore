#![allow(dead_code)] // a lot of elements are considered unused because Godot grabs it over FFI.
#![allow(clippy::tabs_in_doc_comments)] // I literally can't be arsed to care about the tabs in my doc comments.
//! # Squiggles Core
//! *an opinionated code library for making 3D games with Godot 4*
//!
//!	A majority of the resources you will need are in the module [scene]
//!
use godot::prelude::*;

// module specifications
pub mod editor;
pub mod scene;
pub mod util;
// extension loading
struct SquigglesCore;

#[gdextension]
unsafe impl ExtensionLibrary for SquigglesCore {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            scene::register_singletons();
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            scene::unregister_singletons();
        }
    }
}
