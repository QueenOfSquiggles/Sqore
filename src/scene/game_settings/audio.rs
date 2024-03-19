use crate::scene::serialization::{SaveDataBuilder, SquigglesSerialized};
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct GameAudioSettings {
    #[export]
    audio_db_limit: f32,
    #[export]
    audio_bus_volumes: PackedFloat32Array,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for GameAudioSettings {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            audio_db_limit: 0.0f32,
            audio_bus_volumes: PackedFloat32Array::new(),
        }
    }
}

#[godot_api]
impl GameAudioSettings {}

const AUDIO_SETTINGS_PATH: &str = "user://core/audio.json";
impl SquigglesSerialized for GameAudioSettings {
    fn serialize(&mut self) {
        let mut sb = SaveDataBuilder::new_alloc();
        let mut sbind = sb.bind_mut();
        sbind.set_value(
            "audio_db_limit".to_godot(),
            self.audio_db_limit.to_variant(),
        );
        sbind.set_value(
            "audio_bus_volumes".to_godot(),
            self.audio_bus_volumes.to_variant(),
        );

        sbind.save(AUDIO_SETTINGS_PATH.to_godot());
    }

    fn deserialize(&mut self) {
        let sb = SaveDataBuilder::try_load_file(AUDIO_SETTINGS_PATH.to_godot());
        let Some(mut sbgd) = sb else {
            return;
        };
        let mut sbind = sbgd.bind_mut();
        self.audio_db_limit =
            sbind.internal_get_value("audio_db_limit".to_godot(), self.audio_db_limit);
        self.audio_bus_volumes = sbind.internal_get_value(
            "audio_bus_volumes".to_godot(),
            self.audio_bus_volumes.clone(),
        );
    }
}
