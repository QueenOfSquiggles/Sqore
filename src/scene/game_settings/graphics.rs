use godot::{
    engine::{display_server::WindowMode, viewport::Scaling3DMode, DisplayServer, ProjectSettings},
    prelude::*,
};

use crate::scene::serialization::{SaveDataBuilder, SquigglesSerialized};

#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct GameGraphicsSettings {
    #[export]
    use_ssao: bool,
    #[export]
    use_bloom: bool,
    #[export]
    use_sdfgi: bool,
    #[export]
    use_ssil: bool,
    #[export]
    use_ssr: bool,
    #[export]
    value_brightness: f32,
    #[export]
    value_contrast: f32,
    #[export]
    value_saturation: f32,
    #[export]
    value_exposure: f32,
    #[export(enum=(Windowed=0, Minimized=1, Maximized=2, Fullscreen=3, ExclusiveFullscreen=4))]
    window_fullscreen_mode: i32,
    #[export(enum=(Standard=0, FSR10=1, FSR22=2))]
    scaling_algorithm: i32,
    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl IResource for GameGraphicsSettings {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            use_ssao: true,
            base,
            use_bloom: true,
            use_sdfgi: true,
            use_ssil: false,
            use_ssr: false,
            value_brightness: 1.0,
            value_contrast: 1.0,
            value_saturation: 1.0,
            value_exposure: 1.0,
            scaling_algorithm: Scaling3DMode::SCALING_3D_MODE_BILINEAR.ord(),
            window_fullscreen_mode: WindowMode::WINDOW_MODE_MAXIMIZED.ord(),
        }
    }
}

#[godot_api]
impl GameGraphicsSettings {
    #[signal]
    fn graphics_changed() {}

    #[func]
    fn mark_dirty(&mut self) {
        // == 3D scaling (FSR)
        ProjectSettings::singleton().set_setting(
            "rendering/scaling_3d/mode".to_godot(),
            self.scaling_algorithm.to_variant(),
        );

        // == window mode
        DisplayServer::singleton().window_set_mode(match self.window_fullscreen_mode {
            0 => WindowMode::WINDOW_MODE_WINDOWED,
            1 => WindowMode::WINDOW_MODE_MINIMIZED,
            2 => WindowMode::WINDOW_MODE_MAXIMIZED,
            3 => WindowMode::WINDOW_MODE_FULLSCREEN,
            4 => WindowMode::WINDOW_MODE_EXCLUSIVE_FULLSCREEN,
            _ => {
				godot_warn!("CoreGlobals/config/graphics:window_fullscreen_mode = {}. This is outside of the allowed bounds. Don't frickin do that!?", self.window_fullscreen_mode);
				WindowMode::WINDOW_MODE_WINDOWED},
    	    }
		);

        // emit signal out
        self.base_mut()
            .emit_signal(StringName::from("graphics_changed"), &[]);
    }
}

const GRAPHICS_SAVE_PATH: &str = "user://core/graphics.json";

impl SquigglesSerialized for GameGraphicsSettings {
    fn serialize(&mut self) {
        let mut save = SaveDataBuilder::new_alloc();
        let mut bind = save.bind_mut();
        bind.set_value("use_ssao".to_godot(), self.use_ssao.to_variant());
        bind.set_value("use_bloom".to_godot(), self.use_bloom.to_variant());
        bind.set_value("use_sdfgi".to_godot(), self.use_sdfgi.to_variant());
        bind.set_value("use_ssil".to_godot(), self.use_ssil.to_variant());
        bind.set_value("use_ssr".to_godot(), self.use_ssr.to_variant());
        bind.set_value(
            "value_brightness".to_godot(),
            self.value_brightness.to_variant(),
        );
        bind.set_value(
            "value_contrast".to_godot(),
            self.value_contrast.to_variant(),
        );
        bind.set_value(
            "value_saturation".to_godot(),
            self.value_saturation.to_variant(),
        );
        bind.set_value(
            "value_exposure".to_godot(),
            self.value_exposure.to_variant(),
        );
        bind.set_value(
            "window_fullscreen_mode".to_godot(),
            self.window_fullscreen_mode.to_variant(),
        );
        bind.set_value(
            "scaling_algorithm".to_godot(),
            self.scaling_algorithm.to_variant(),
        );
        bind.save(GRAPHICS_SAVE_PATH.into_godot());
    }

    fn deserialize(&mut self) {
        let Some(mut load) = SaveDataBuilder::try_load_file(GRAPHICS_SAVE_PATH.into_godot()) else {
            return;
        };
        // use_ssao: bool,
        // use_bloom: bool,
        // use_sdfgi: bool,
        // use_ssil: bool,
        // use_ssr: bool,
        // value_brightness: f32,
        // value_contrast: f32,
        // value_saturation: f32,
        // value_exposure: f32,
        // window_fullscreen_mode: i32,
        let mut bind = load.bind_mut();
        self.use_ssao = bind.internal_get_value("use_ssao".to_godot(), self.use_ssao.to_godot());
        self.use_bloom = bind.internal_get_value("use_bloom".to_godot(), self.use_bloom.to_godot());
        self.use_sdfgi = bind.internal_get_value("use_sdfgi".to_godot(), self.use_sdfgi.to_godot());
        self.use_ssil = bind.internal_get_value("use_ssil".to_godot(), self.use_ssil.to_godot());
        self.use_ssr = bind.internal_get_value("use_ssr".to_godot(), self.use_ssr.to_godot());
        self.value_brightness =
            bind.internal_get_value("value_brightness".to_godot(), self.value_brightness);
        self.value_contrast =
            bind.internal_get_value("value_contrast".to_godot(), self.value_contrast);
        self.value_saturation =
            bind.internal_get_value("value_saturation".to_godot(), self.value_saturation);
        self.value_exposure =
            bind.internal_get_value("value_exposure".to_godot(), self.value_exposure);
        self.window_fullscreen_mode = bind.internal_get_value(
            "window_fullscreen_mode".to_godot(),
            self.window_fullscreen_mode,
        );
    }
}
