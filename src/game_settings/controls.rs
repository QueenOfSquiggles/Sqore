use std::collections::HashMap;

use godot::{
    engine::{
        global::{JoyAxis, JoyButton, Key, MouseButton},
        ClassDb, InputEvent, InputEventJoypadButton, InputEventJoypadMotion, InputEventKey,
        InputEventMouseButton, InputMap, Json,
    },
    prelude::*,
};
use num_derive::FromPrimitive;
use num_traits::{FromPrimitive, ToPrimitive};

use crate::serialization::{SaveDataBuilder, SquigglesSerialized};

#[derive(FromPrimitive)]
enum MappingsStyle {
    All = 0,
    AllCustom = 1,
    OnlySpecified = 2,
}

#[derive(FromPrimitive)]
enum HandledInputEvents {
    JoypadButton = 0,
    JoypadMotion = 1,
    Key = 2,
    MouseButton = 3,
    Generic = 4,
}

impl ToPrimitive for HandledInputEvents {
    fn to_i64(&self) -> Option<i64> {
        match self {
            HandledInputEvents::JoypadButton => Some(0),
            HandledInputEvents::JoypadMotion => Some(1),
            HandledInputEvents::Key => Some(2),
            HandledInputEvents::MouseButton => Some(3),
            HandledInputEvents::Generic => Some(4),
        }
    }

    fn to_u64(&self) -> Option<u64> {
        let val = self.to_i64();
        Some(u64::try_from(val.unwrap_or(0)).unwrap_or(0))
    }
}

type MappingStorage = HashMap<GString, Array<Gd<InputEvent>>>;
#[derive(GodotClass)]
#[class(tool, base=Resource)]
pub struct GameControlsSettings {
    // exported
    #[export(enum=(All=0, AllCustom=1,OnlySpecified=2))]
    mappings_style: i32,
    #[export]
    allowed_mappings: PackedStringArray,

    // internal
    base_mappings: MappingStorage,
    mapping_overrides: MappingStorage,
    #[base]
    base: Base<Resource>,
}
#[godot_api]
impl IResource for GameControlsSettings {
    fn init(base: Base<Self::Base>) -> Self {
        // loads known mappings on initialization
        let mut base_mappings = MappingStorage::new();
        let mut map = InputMap::singleton();
        let actions = map.get_actions();
        for a in actions.iter_shared() {
            let events = map.action_get_events(a.clone());
            base_mappings.insert(a.into(), events);
        }

        Self {
            mappings_style: 0,
            allowed_mappings: PackedStringArray::new(),
            base_mappings,
            mapping_overrides: MappingStorage::new(),
            base,
        }
    }
}

#[godot_api]
impl GameControlsSettings {
    #[func]
    fn load_binds(&self) {
        for (action, mappings) in self.mapping_overrides.clone() {
            let sn = StringName::from(action.to_string());
            InputMap::singleton().action_erase_events(sn.clone());
            for map in mappings.iter_shared() {
                InputMap::singleton().action_add_event(sn.clone(), map)
            }
        }
    }
    #[func]
    fn can_bind(&self, action: GString) -> bool {
        let option_map_style: Option<MappingsStyle> = FromPrimitive::from_i32(self.mappings_style);
        let Some(map_style) = option_map_style else {
            godot_warn!("failed to map internal rust->MappingsStyle to i32. This will break deserialization for controls bindings");
            return false;
        };
        match map_style {
            MappingsStyle::All => true,
            MappingsStyle::AllCustom => !action.to_string().starts_with("input_"),
            MappingsStyle::OnlySpecified => self.allowed_mappings.contains(action),
        }
    }
    #[func]
    fn push_bind(&mut self, action_name: GString, event: Gd<InputEvent>) {
        if let Some(arr) = self.mapping_overrides.get_mut(&action_name) {
            arr.push(event.upcast());
        } else {
            let mut arr: Array<Gd<InputEvent>> = Array::new();
            arr.push(event);
            self.mapping_overrides.insert(action_name, arr);
        }
    }

    fn bind_generic(&mut self, data_dict: Dictionary, action_name: GString) {
        godot_print!("Trying to load generic event for action\"{}\"", action_name);
        // TODO: this is where I would usually use some kind of reflection? Not sure how I could manage that?
        let res_str_data = GString::try_from_variant(&data_dict.get_or_nil("json_data"));
        let Ok(str_data) = res_str_data else {
            return;
        };
        if !str_data.to_string().starts_with("\"InputEvent") {
            // guard against non-inputevent types (which all have InputEvent* as a prefix)
            return;
        }
        let stringval = str_data.to_string();
        let parts: Vec<&str> = stringval.splitn(3, '\"').collect();
        if parts.len() != 3 {
            return;
        }
        godot_print!(
            "Attempting to initialize an InputEvent of type \"{}\"",
            parts[1]
        );
        let sname = StringName::from(parts[1]);
        let classes = ClassDb::singleton();
        if classes.class_exists(sname.clone()) && classes.can_instantiate(sname.clone()) {
            #[allow(unused)] // just to clear this pesky warning
            let instance = classes.instantiate(sname);
            // TODO: how to we cast this variant to an object type? (Which would allow us to try to downcast)
        }
    }

    fn bind_joypad_button(&mut self, data_dict: Dictionary, action_name: GString) {
        let mut event = InputEventJoypadButton::new();
        let Some(index) = SaveDataBuilder::get_entry_from(&data_dict, "button_index") else {
            return;
        };
        let Some(pressed) = SaveDataBuilder::get_entry_from(&data_dict, "pressed") else {
            return;
        };
        let Some(pressure) = SaveDataBuilder::get_entry_from(&data_dict, "pressure") else {
            return;
        };
        event.set_button_index(JoyButton::from_ord(index));
        event.set_pressed(pressed);
        event.set_pressure(pressure);
        self.push_bind(action_name, event.upcast());
    }
    fn bind_joypad_motion(&mut self, data_dict: Dictionary, action_name: GString) {
        let mut event = InputEventJoypadMotion::new();
        let Some(axis) = SaveDataBuilder::get_entry_from(&data_dict, "axis") else {
            return;
        };
        let Some(axis_value) = SaveDataBuilder::get_entry_from(&data_dict, "axis_value") else {
            return;
        };
        event.set_axis(JoyAxis::from_ord(axis));
        event.set_axis_value(axis_value);
        self.push_bind(action_name, event.upcast());
    }
    fn bind_key(&mut self, data_dict: Dictionary, action_name: GString) {
        let mut event = InputEventKey::new();
        let Some(echo) = SaveDataBuilder::get_entry_from(&data_dict, "key") else {
            return;
        };
        let Some(key_label) = SaveDataBuilder::get_entry_from(&data_dict, "key") else {
            return;
        };
        let Some(keycode) = SaveDataBuilder::get_entry_from(&data_dict, "key") else {
            return;
        };
        let Some(physical_keycode) = SaveDataBuilder::get_entry_from(&data_dict, "key") else {
            return;
        };
        let Some(pressed) = SaveDataBuilder::get_entry_from(&data_dict, "key") else {
            return;
        };
        let Some(unicode) = SaveDataBuilder::get_entry_from(&data_dict, "key") else {
            return;
        };
        event.set_echo(echo);
        event.set_key_label(key_label);
        event.set_keycode(Key::from_ord(keycode));
        event.set_physical_keycode(Key::from_ord(physical_keycode));
        event.set_pressed(pressed);
        event.set_unicode(unicode);
        self.push_bind(action_name, event.upcast());
    }
    fn bind_mouse_button(&mut self, data_dict: Dictionary, action_name: GString) {
        let mut event = InputEventMouseButton::new();
        let Some(button_index) = SaveDataBuilder::get_entry_from(&data_dict, "button_index") else {
            return;
        };
        let Some(cancelled) = SaveDataBuilder::get_entry_from(&data_dict, "cancelled") else {
            return;
        };
        let Some(double_click) = SaveDataBuilder::get_entry_from(&data_dict, "double_click") else {
            return;
        };
        let Some(factor) = SaveDataBuilder::get_entry_from(&data_dict, "factor") else {
            return;
        };
        let Some(pressed) = SaveDataBuilder::get_entry_from(&data_dict, "pressed") else {
            return;
        };
        event.set_button_index(MouseButton::from_ord(button_index));
        event.set_canceled(cancelled);
        event.set_double_click(double_click);
        event.set_factor(factor);
        event.set_pressed(pressed);
        self.push_bind(action_name, event.upcast());
    }

    // JoypadButton = 0,
    // JoypadMotion = 1,
    // Key = 2,
    // MouseButton = 3,

    fn ser_generic(&self, event: Gd<InputEvent>) -> Option<Dictionary> {
        let mut dict = Dictionary::new();
        dict.insert(
            "type".to_godot(),
            HandledInputEvents::Generic
                .to_i32()
                .unwrap_or(-1)
                .to_variant(),
        );

        let data = Json::stringify(event.to_variant());
        dict.insert("json_data", data);
        Some(dict)
    }

    fn ser_joy_button(&self, event: Gd<InputEvent>) -> Option<Dictionary> {
        let rcast: Result<Gd<InputEventJoypadButton>, _> = event.try_cast();
        let Ok(event) = rcast else {
            return None;
        };
        let mut dict = Dictionary::new();
        dict.insert(
            "type".to_godot(),
            HandledInputEvents::JoypadButton
                .to_i32()
                .unwrap_or(-1)
                .to_variant(),
        );
        dict.insert("button_index".to_godot(), event.get_button_index().ord());
        dict.insert("pressed".to_godot(), event.is_pressed());
        dict.insert("pressure".to_godot(), event.get_pressure());

        Some(dict)
    }
    fn ser_joy_motion(&self, event: Gd<InputEvent>) -> Option<Dictionary> {
        let rcast: Result<Gd<InputEventJoypadMotion>, _> = event.try_cast();
        let Ok(event) = rcast else {
            return None;
        };
        let mut dict = Dictionary::new();
        dict.insert(
            "type".to_godot(),
            HandledInputEvents::JoypadMotion
                .to_i32()
                .unwrap_or(-1)
                .to_variant(),
        );

        dict.insert("axis".to_godot(), event.get_axis());
        dict.insert("axis_value".to_godot(), event.get_axis_value());

        Some(dict)
    }
    fn ser_key(&self, e: Gd<InputEvent>) -> Option<Dictionary> {
        let rcast: Result<Gd<InputEventKey>, _> = e.try_cast();
        let Ok(event) = rcast else {
            return None;
        };
        let mut dict = Dictionary::new();
        dict.insert(
            "type".to_godot(),
            HandledInputEvents::Key.to_i32().unwrap_or(-1).to_variant(),
        );
        dict.insert("echo".to_godot(), event.is_echo());
        dict.insert("key_label".to_godot(), event.get_key_label());
        dict.insert("keycode".to_godot(), event.get_keycode().ord());
        dict.insert(
            "physical_keycode".to_godot(),
            event.get_physical_keycode().ord(),
        );
        dict.insert("pressed".to_godot(), event.is_pressed());
        dict.insert("unicode".to_godot(), event.get_unicode());

        Some(dict)
    }
    fn ser_mouse_button(&self, event: Gd<InputEvent>) -> Option<Dictionary> {
        let rcast: Result<Gd<InputEventMouseButton>, _> = event.try_cast();
        let Ok(event) = rcast else {
            return None;
        };
        let mut dict = Dictionary::new();
        dict.insert(
            "type".to_godot(),
            HandledInputEvents::MouseButton
                .to_i32()
                .unwrap_or(-1)
                .to_variant(),
        );
        dict.insert("button_index".to_godot(), event.get_button_index());
        dict.insert("cancelled".to_godot(), event.is_canceled());
        dict.insert("double_click".to_godot(), event.is_double_click());
        dict.insert("factor".to_godot(), event.get_factor());
        dict.insert("pressed".to_godot(), event.is_pressed());

        Some(dict)
    }
}

const CONTROLS_SETTINGS_PATH: &str = "user://core/controls.json";
impl SquigglesSerialized for GameControlsSettings {
    fn serialize(&mut self) {
        let mut sb = SaveDataBuilder::alloc_gd();
        let mut sbind = sb.bind_mut();
        for (key, values) in self.mapping_overrides.clone() {
            let mut data_arr: Array<Dictionary> = Array::new();
            for event in values.iter_shared() {
                if let Some(dict) = self.ser_joy_button(event.clone()) {
                    data_arr.push(dict);
                } else if let Some(dict) = self.ser_joy_motion(event.clone()) {
                    data_arr.push(dict);
                } else if let Some(dict) = self.ser_key(event.clone()) {
                    data_arr.push(dict);
                } else if let Some(dict) = self.ser_mouse_button(event.clone()) {
                    data_arr.push(dict);
                } else if let Some(dict) = self.ser_generic(event) {
                    data_arr.push(dict);
                }
            }
            sbind.set_value(key, data_arr.to_variant());
        }
        sbind.save(CONTROLS_SETTINGS_PATH.to_godot());
    }

    fn deserialize(&mut self) {
        let Some(mut sb) = SaveDataBuilder::try_load_file(CONTROLS_SETTINGS_PATH.to_godot()) else {
            return;
        };
        let mut sbind = sb.bind_mut();
        let custom_bindings_dictionary = sbind.get_as_dict();
        for (key, value) in custom_bindings_dictionary.iter_shared() {
            if !self.can_bind(GString::from_variant(&key)) {
                godot_warn!("Found disallowed binding on disk! This is probably fine, but check into it if you keep seeing this!");
                continue;
            }
            let action_name = GString::from_variant(&key);

            let values: Array<Dictionary> = Array::try_from_variant(&value).unwrap_or_default();
            godot_print!("Found mappings on disk for: {} ", key);
            for map in values.iter_shared() {
                let mut event_type = HandledInputEvents::Generic;
                if let Ok(type_id) = i32::try_from_variant(&map.get_or_nil("type".to_godot())) {
                    let opt_event_type: Option<HandledInputEvents> =
                        FromPrimitive::from_i32(type_id);
                    if let Some(e_type) = opt_event_type {
                        event_type = e_type;
                    }
                }
                match event_type {
                    HandledInputEvents::JoypadButton => {
                        self.bind_joypad_button(map, action_name.clone())
                    }
                    HandledInputEvents::JoypadMotion => {
                        self.bind_joypad_motion(map, action_name.clone())
                    }
                    HandledInputEvents::Key => self.bind_key(map, action_name.clone()),
                    HandledInputEvents::MouseButton => {
                        self.bind_mouse_button(map, action_name.clone())
                    }
                    HandledInputEvents::Generic => self.bind_generic(map, action_name.clone()),
                }
            }
        }
        self.load_binds();
    }
}
