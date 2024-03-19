use godot::prelude::*;

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct GameGameplaySettings {
    #[export]
    options_bool: Array<Gd<GameplayOptionBool>>,
    #[export]
    options_number: Array<Gd<GameplayOptionNumber>>,
    #[export]
    options_string: Array<Gd<GameplayOptionString>>,

    base: Base<Resource>,
}

#[godot_api]
impl IResource for GameGameplaySettings {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            options_bool: Array::new(),
            options_number: Array::new(),
            options_string: Array::new(),
            base,
        }
    }
}

#[godot_api]
impl GameGameplaySettings {
    fn get_option_bool(&self, key: GString) -> Option<Gd<GameplayOptionBool>> {
        self.options_bool
            .iter_shared()
            .find(|s| s.bind().option_key == key)
    }
    fn get_option_number(&self, key: GString) -> Option<Gd<GameplayOptionNumber>> {
        self.options_number
            .iter_shared()
            .find(|s| s.bind().option_key == key)
    }
    fn get_option_string(&self, key: GString) -> Option<Gd<GameplayOptionString>> {
        self.options_string
            .iter_shared()
            .find(|s| s.bind().option_key == key)
    }

    fn get_value_bool(&self, key: GString) -> bool {
        if let Some(option) = self.get_option_bool(key) {
            return option.bind().value;
        }
        false
    }
    fn get_value_number(&self, key: GString) -> f32 {
        if let Some(option) = self.get_option_number(key) {
            return option.bind().value;
        }
        0f32
    }
    fn get_value_string(&self, key: GString) -> GString {
        if let Some(option) = self.get_option_string(key) {
            return option.bind().value.clone();
        }
        GString::from("")
    }
}

//
//	Available Options Entries
//

enum OptionEntry {
    Bool(GameplayOptionBool),
    Number(GameplayOptionNumber),
    String(GameplayOptionString),
    None,
}

// Booleans
#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct GameplayOptionBool {
    #[export]
    option_key: GString,
    #[export]
    value: bool,

    node: Base<Resource>,
}

#[godot_api]
impl GameplayOptionBool {}

// Number
#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct GameplayOptionNumber {
    #[export]
    option_key: GString,
    #[export]
    value: f32,
    #[export]
    min: f32,
    #[export]
    max: f32,
    #[export]
    step: f32,
    #[export]
    allow_greater: bool,
    #[export]
    allow_lesser: bool,

    node: Base<Resource>,
}

#[godot_api]
impl GameplayOptionNumber {}

// String
#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct GameplayOptionString {
    #[export]
    option_key: GString,
    #[export]
    value: GString,
    #[export]
    treat_as_enum: bool,
    #[export]
    enum_values: PackedStringArray,

    node: Base<Resource>,
}

#[godot_api]
impl GameplayOptionString {}
