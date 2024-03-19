use godot::prelude::*;

use self::{
    audio::GameAudioSettings, controls::GameControlsSettings, gameplay::GameGameplaySettings,
    graphics::GameGraphicsSettings, user_mods::UserModifications,
};
use super::dialog::dialog_settings::DialogSettings;
use crate::scene::{serialization::SquigglesSerialized, vfx_stack::vfx_stack_resource::VFXStack};

pub mod accessibility;
pub mod audio;
pub mod controls;
pub mod effects;
pub mod gameplay;
pub mod graphics;
pub mod user_mods;

#[derive(GodotClass)]
#[class(tool, init, base=Resource)]
pub struct SquigglesCoreConfig {
    #[export]
    pub graphics: Option<Gd<GameGraphicsSettings>>,
    #[export]
    pub controls: Option<Gd<GameControlsSettings>>,
    #[export]
    pub gameplay: Option<Gd<GameGameplaySettings>>,
    #[export]
    pub audio: Option<Gd<GameAudioSettings>>,
    #[export]
    pub vfx_stack: Option<Gd<VFXStack>>,
    #[export]
    pub dialog: Option<Gd<DialogSettings>>,
    #[export]
    pub user_mods: Option<Gd<UserModifications>>,

    //
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
