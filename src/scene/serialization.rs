use std::collections::HashMap;

use godot::{
    engine::{file_access::ModeFlags, DirAccess, FileAccess, Json, ProjectSettings},
    prelude::*,
};

const INTERNAL_PREFIX: &str = "__internal__";

#[derive(GodotClass)]
#[class(base=Object)]
pub struct SaveDataBuilder {
    data: Dictionary,
    child_builders: HashMap<GString, Gd<SaveDataBuilder>>,
    base: Base<Object>,
}

#[godot_api]
impl IObject for SaveDataBuilder {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            data: Dictionary::new(),
            child_builders: HashMap::new(),
            base,
        }
    }
}
#[godot_api]
pub impl SaveDataBuilder {
    #[func]
    pub fn set_value(&mut self, key: GString, value: Variant) {
        self.data.set(key, value);
    }

    #[func]
    pub fn get_value(&mut self, key: GString) -> Variant {
        self.data.get_or_nil(key)
    }

    #[func]
    pub fn get_value_or_default(&mut self, key: GString, default: Variant) -> Variant {
        if let Some(value) = self.data.get(key) {
            return value;
        }
        default
    }

    #[func]
    pub fn get_child_builder(&mut self, key: GString) -> Gd<SaveDataBuilder> {
        let mut n_builder = SaveDataBuilder::new_alloc();
        if let Some(child) = self.child_builders.get_mut(&key) {
            n_builder = child.clone();
        } else {
            self.child_builders.insert(key, n_builder.clone());
        }
        n_builder
    }

    #[func]
    pub fn save(&mut self, file_path: GString) -> bool {
        let abs_path = ProjectSettings::singleton().globalize_path(file_path.clone());
        if let Some(base_dir) = std::path::Path::new(abs_path.to_string().as_str()).parent() {
            if let Some(valid_base_dir) = base_dir.to_str() {
                if !DirAccess::dir_exists_absolute(GString::from(valid_base_dir)) {
                    DirAccess::make_dir_recursive_absolute(GString::from(valid_base_dir));
                }
            }
        }
        let Some(mut file) = FileAccess::open(file_path.clone(), ModeFlags::WRITE) else {
            godot_warn!("Failed to access file {}", file_path);
            return false;
        };
        let text = Json::stringify(self.get_as_dict().to_variant());
        file.store_string(text);
        true
    }

    #[func]
    pub fn load(&mut self, file_path: GString) -> bool {
        let Some(file) = FileAccess::open(file_path.clone(), ModeFlags::READ) else {
            godot_warn!("Failed to access file {}", file_path);
            return false;
        };
        let opt_cast = Json::parse_string(file.get_as_text());
        if let Ok(data) = Dictionary::try_from_variant(&opt_cast) {
            for entry in data.iter_shared() {
                let skey = GString::from_variant(&entry.0);
                if skey.to_string().starts_with(INTERNAL_PREFIX) {
                } else {
                    self.data.set(skey, entry.1);
                }
            }
        };

        true
    }

    #[func]
    pub fn load_from(dict: Dictionary) -> Gd<SaveDataBuilder> {
        let mut data = SaveDataBuilder::new_alloc();
        for entry in dict.iter_shared() {
            let skey = GString::from_variant(&entry.0);
            if skey.to_string().starts_with(INTERNAL_PREFIX) {
                // attempts to construct an internal save data builder (which is effecitively a sub-layer in the JSON)
                if let Ok(dict) = Dictionary::try_from_variant(&entry.1) {
                    let child = SaveDataBuilder::load_from(dict);
                    let i_key = skey.to_string().replace(INTERNAL_PREFIX, "");
                    data.bind_mut()
                        .child_builders
                        .insert(i_key.to_godot(), child);
                } else {
                    godot_warn!("Found SaveDataBuilder entry that is corrupted. Please ensure this JSON data is correct. Key=\"{}\"; expected dictionary value. Found: {}", skey, entry.1);
                }
            } else {
                // loads a simple data value
                data.bind_mut().data.set(skey, entry.1);
            }
        }
        data
    }

    pub fn try_load_file(file_path: GString) -> Option<Gd<SaveDataBuilder>> {
        let mut result = SaveDataBuilder::new_alloc();
        if !result.bind_mut().load(file_path) {
            None
        } else {
            Some(result)
        }
    }

    #[func]
    pub fn get_as_dict(&mut self) -> Dictionary {
        let mut dict = Dictionary::new();
        for entry in self.data.iter_shared() {
            dict.insert(entry.0, entry.1);
        }
        for entry in self.child_builders.iter_mut() {
            let n_key: GString =
                (String::from(INTERNAL_PREFIX) + &entry.0.to_string().to_owned()).to_godot();
            dict.insert(n_key, entry.1.bind_mut().get_as_dict());
        }
        dict
    }

    pub fn internal_get_value<T: FromGodot + ToGodot>(
        &mut self,
        key: GString,
        default_value: T,
    ) -> T {
        let value = self.get_value(key);
        if value.is_nil() {
            return default_value;
        }
        let Ok(valid) = T::try_from_variant(&value) else {
            return default_value;
        };
        valid
    }

    pub fn get_entry_from<T: FromGodot>(dict: &Dictionary, key: &str) -> Option<T> {
        let Some(value) = dict.get(key.to_godot()) else {
            return None;
        };
        let Ok(valid_value) = T::try_from_variant(&value) else {
            return None;
        };
        Some(valid_value)
    }
}

/// Unfortunately this only can be used internally, but it grants access to serialization functions for all serializable functions
pub trait SquigglesSerialized {
    fn serialize(&mut self);
    fn deserialize(&mut self);
}
