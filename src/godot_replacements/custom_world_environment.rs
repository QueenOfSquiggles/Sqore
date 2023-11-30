use godot::{
    engine::{Environment, IWorldEnvironment, WorldEnvironment},
    prelude::*,
};

use crate::game_globals::CoreGlobals;

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
        // TODO: omfg if <unwrap> else {...}; is so lovely on the eyes. I wanna convert some of the more deeply nested fuctions into this pattern if possible.
        let option_env = self.base.get_environment();
        let mut env = Environment::new();
        if option_env.is_some() && !self.force_override {
            // if there is an existing environment and we are not forcing an override, let that be the environment
            return;
        }
        if let Some(n_env) = option_env {
            env = n_env;
        }

        let gd_gfx = CoreGlobals::singleton()
            .bind()
            .get_config()
            .bind()
            .get_graphics();
        let gfx = gd_gfx.bind();
        env.set_glow_enabled(gfx.get_use_bloom());

        self.base.set_environment(env);
    }
}

#[godot_api]
impl WorldEnvironmentSettingsCompliant {}
