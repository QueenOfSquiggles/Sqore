use godot::prelude::*;

use crate::scene::{serialization::SquigglesSerialized, vfx_stack::vfx_stack_resource::VFXStack};

use self::{
    audio::GameAudioSettings, controls::GameControlsSettings, gameplay::GameGameplaySettings,
    graphics::GameGraphicsSettings,
};

pub mod accessibility;
pub mod audio;
pub mod controls;
pub mod effects;
pub mod gameplay;
pub mod graphics;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct SquigglesCoreConfig {
    #[export]
    graphics: Option<Gd<GameGraphicsSettings>>,
    #[export]
    controls: Option<Gd<GameControlsSettings>>,
    #[export]
    gameplay: Option<Gd<GameGameplaySettings>>,
    #[export]
    audio: Option<Gd<GameAudioSettings>>,
    #[export]
    vfx_stack: Option<Gd<VFXStack>>,

    //
    #[base]
    base: Base<Resource>,
}
#[godot_api]
impl IResource for SquigglesCoreConfig {}

#[godot_api]
impl SquigglesCoreConfig {}

impl SquigglesSerialized for SquigglesCoreConfig {
    fn serialize(&mut self) {
        if let Some(mut gfx) = self.graphics.clone() {
            gfx.bind_mut().serialize();
        }
        if let Some(mut controls) = self.controls.clone() {
            controls.bind_mut().serialize();
        }
        if let Some(mut audio) = self.audio.clone() {
            audio.bind_mut().serialize();
        }
        // if let Some(mut gameplay) = self.gameplay {
        // 	gameplay.bind_mut().serialize();
        // }
        // if let Some(mut vfx) = self.vfx_stack {
        // 	vfx.bind_mut().serialize();
        // }
    }

    fn deserialize(&mut self) {
        if let Some(mut gfx) = self.graphics.clone() {
            gfx.bind_mut().deserialize()
        }
        if let Some(mut controls) = self.controls.clone() {
            controls.bind_mut().deserialize()
        }
        if let Some(mut audio) = self.audio.clone() {
            audio.bind_mut().deserialize()
        }
        // if let Some(mut gameplay) = self.gameplay {
        // 	gameplay.bind_mut().deserialize();
        // }
        // if let Some(mut vfx) = self.vfx_stack {
        // 	vfx.bind_mut().deserialize();
        // }
    }
}
