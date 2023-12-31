use godot::{
    engine::{viewport::Scaling3DMode, Environment, IWorldEnvironment, WorldEnvironment},
    prelude::*,
};

use crate::scene::game_globals::CoreGlobals;

#[derive(GodotClass)]
#[class(init, base=WorldEnvironment)]
struct WorldEnvironmentSettingsCompliant {
    #[export]
    force_override: bool,
    #[base]
    base: Base<WorldEnvironment>,
}

#[godot_api]
impl IWorldEnvironment for WorldEnvironmentSettingsCompliant {
    fn ready(&mut self) {
        self.on_graphics_settings_changed();
        if let Some(mut gfx) = CoreGlobals::singleton()
            .bind()
            .get_config()
            .bind()
            .get_graphics()
        {
            gfx.connect(
                StringName::from("graphics_changed"),
                Callable::from_object_method(&self.to_gd(), "on_graphics_settings_changed"),
            );
        }
    }
}

#[godot_api]
impl WorldEnvironmentSettingsCompliant {
    fn on_graphics_settings_changed(&mut self) {
        let option_env = self.base.get_environment();
        let mut env = Environment::new();
        if let Some(n_env) = option_env {
            env = n_env;
        }

        let Some(gd_gfx) = CoreGlobals::singleton()
            .bind()
            .get_config()
            .bind()
            .get_graphics()
        else {
            return;
        };
        let gfx = gd_gfx.bind();
        env.set_glow_enabled(gfx.get_use_bloom());
        env.set_ssao_enabled(gfx.get_use_ssao());
        env.set_sdfgi_enabled(gfx.get_use_sdfgi());
        env.set_ssil_enabled(gfx.get_use_ssil());
        env.set_ssr_enabled(gfx.get_use_ssr());
        env.set_adjustment_brightness(gfx.get_value_brightness());
        env.set_adjustment_contrast(gfx.get_value_contrast());
        env.set_adjustment_saturation(gfx.get_value_saturation());
        env.set_tonemap_exposure(gfx.get_value_exposure());
        if let Some(mut viewport) = self.base.get_viewport() {
            viewport.set_scaling_3d_mode(match gfx.get_scaling_algorithm() {
                0 => Scaling3DMode::SCALING_3D_MODE_BILINEAR,
                1 => Scaling3DMode::SCALING_3D_MODE_FSR,
                2 => Scaling3DMode::SCALING_3D_MODE_FSR2,
                _ => unreachable!(),
            })
        }
        self.base.set_environment(env);
    }
}
