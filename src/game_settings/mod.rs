use godot::prelude::*;

use crate::serialization::SquigglesSerialized;

use self::graphics::GameGraphicsSettings;

pub mod graphics;

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct SquigglesCoreConfig {
    #[export]
    graphics: Gd<graphics::GameGraphicsSettings>,

    #[base]
    base: Base<Resource>,
}
#[godot_api]
impl IResource for SquigglesCoreConfig {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            graphics: GameGraphicsSettings::new_gd(),
        }
    }
}

#[godot_api]
impl SquigglesCoreConfig {}

impl SquigglesSerialized for SquigglesCoreConfig {
    fn serialize(&mut self) {
        self.graphics.bind_mut().serialize();
    }

    fn deserialize(&mut self) {
        self.graphics.bind_mut().deserialize();
    }
}
