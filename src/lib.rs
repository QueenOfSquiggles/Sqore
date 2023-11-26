use godot::prelude::*;

pub mod camera;
pub mod error_handling;
pub mod interaction;
pub mod state_machine;
struct SquigglesCore;

#[gdextension]
unsafe impl ExtensionLibrary for SquigglesCore {}
