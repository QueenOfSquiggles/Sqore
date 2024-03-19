use godot::prelude::*;

use godot::engine::{ISubViewportContainer, SubViewportContainer};

#[derive(GodotClass)]
#[class(base=SubViewportContainer)]
pub struct PostProcessingContainer {
    base: Base<SubViewportContainer>,
}

#[godot_api]
impl ISubViewportContainer for PostProcessingContainer {
    fn init(base: Base<SubViewportContainer>) -> Self {
        Self { base }
    }
}
