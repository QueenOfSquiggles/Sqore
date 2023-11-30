use godot::{
    builtin::meta::{ConvertError, GodotConvert},
    engine::*,
    prelude::*,
};

use crate::{
    editor_plugin::SINGLETON_CORE_GLOBALS, game_settings::SquigglesCoreConfig,
    serialization::SquigglesSerialized,
};

const PROJECT_SETTINGS_NAMESPACE: &str = "addons/squiggles_core/";
const S_LOADERS: &str = "loaders";
const S_GAME_SETTINGS: &str = "game_settings";

fn get_setting_name(name: &str) -> GString {
    (String::from(PROJECT_SETTINGS_NAMESPACE) + name).to_godot()
}

#[derive(GodotClass)]
#[class(tool, base=Object)]
pub struct CoreGlobals {
    #[var]
    config: Gd<SquigglesCoreConfig>,
    #[base]
    base: Base<Object>,
}

#[godot_api]
impl IObject for CoreGlobals {
    fn init(base: Base<Self::Base>) -> Self {
        // let mut zelf = Self { config: None, base };
        let mut possible_config: Option<Gd<SquigglesCoreConfig>> = None;
        match Self::get_or_init_default(S_LOADERS, PackedStringArray::new()) {
            Err(err) => godot_warn!("Conversion Error: {}", err.to_string()),
            Ok(loaders) => {
                for item in loaders.as_slice().iter() {
                    godot_print!("Found loader entry: {}", item);
                }
            }
        }
        // try load configuration file
        if let Ok(config_path) =
            Self::get_or_init_default(S_GAME_SETTINGS, "res://squiggles_config.tres".to_godot())
        {
            if let Some(config_resource) = ResourceLoader::singleton().load(config_path.clone()) {
                let opt_res: Result<Gd<SquigglesCoreConfig>, Gd<Resource>> =
                    config_resource.try_cast();
                if let Ok(valid_resource) = opt_res {
                    possible_config = Some(valid_resource);
                }
            } else {
                let msg = format!("Expected an instance of `SquigglesCoreConfig` resource to be at path: \"{}\". Either create the resource at that location, or update the `{}` setting in your project settings.", config_path, S_GAME_SETTINGS);
                godot_error!("{}", msg);
                godot_print!("{}", msg);
            }
        }
        let mut zelf = Self {
            config: possible_config.unwrap_or(SquigglesCoreConfig::new_gd()),
            base,
        };
        zelf.reload_globals();

        zelf
    }
}

#[godot_api]
impl CoreGlobals {
    #[signal]
    fn global_serialize() {}

    #[signal]
    fn global_deserialize() {}

    #[func]
    fn get_setting(&self, name: String, default_value: Variant) -> Variant {
        let result = Self::get_or_init_default(name.as_str(), default_value);
        match result {
            Ok(value) => value,
            Err(_) => Variant::nil(),
        }
    }
    #[func]
    fn save_globals(&mut self) {
        self.serialize();
    }

    #[func]
    fn reload_globals(&mut self) {
        self.deserialize();
    }

    // internal specialized functions

    pub fn get_or_init_default<T: GodotConvert + FromGodot + ToGodot>(
        name: &str,
        default: T,
    ) -> Result<T, ConvertError> {
        let mut project = ProjectSettings::singleton();
        let value_volatile = project.get_setting(get_setting_name(name));

        if value_volatile.is_nil() || value_volatile.get_type() != default.to_variant().get_type() {
            project.set_setting(get_setting_name(name), default.to_variant());
            Ok(default)
        } else {
            // no longer volatile
            T::try_from_variant(&value_volatile)
        }
    }

    pub fn singleton() -> Gd<CoreGlobals> {
        let Some(vol) = Engine::singleton().get_singleton(SINGLETON_CORE_GLOBALS.clone()) else {
            panic!("Failed to find engine singleton for CoreGlobals. You must access this after it is registered!");
        };
        let res_core: Result<Gd<CoreGlobals>, Gd<_>> = vol.try_cast();
        let Ok(core) = res_core else {
            panic!("Failed to cast engine singleton for CoreGlobals. This should never happen!");
        };
        core
    }
}

impl SquigglesSerialized for CoreGlobals {
    fn serialize(&mut self) {
        // I'm comfy using unwrap because this struct should never be constructed outside of the init function, which assigns the
        self.config.bind_mut().serialize();
    }

    fn deserialize(&mut self) {
        self.config.bind_mut().deserialize();
    }
}
