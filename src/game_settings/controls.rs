use std::collections::HashMap;

use godot::{
    engine::{
        global::{JoyAxis, JoyButton, Key, MouseButton},
        InputEvent, InputEventJoypadButton, InputEventJoypadMotion, InputEventKey,
        InputEventMouseButton, InputMap,
    },
    prelude::*,
};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

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
    fn load_binds(&self) {
        for (action, mappings) in self.mapping_overrides.clone() {
            let sn = StringName::from(action.to_string());
            InputMap::singleton().action_erase_events(sn.clone());
            for map in mappings.iter_shared() {
                InputMap::singleton().action_add_event(sn.clone(), map)
            }
        }
    }
    fn can_bind(&self, action: GString) -> bool {
        let option_map_style: Option<MappingsStyle> = FromPrimitive::from_i32(self.mappings_style);
        let Some(map_style) = option_map_style else {
            return false;
        };
        match map_style {
            MappingsStyle::All => true,
            MappingsStyle::AllCustom => !action.to_string().starts_with("input_"),
            MappingsStyle::OnlySpecified => self.allowed_mappings.contains(action),
        }
    }
    fn push_bind(&mut self, action_name: GString, event: Gd<InputEvent>) {
        if let Some(arr) = self.mapping_overrides.get_mut(&action_name) {
            arr.push(event.upcast());
        } else {
            let mut arr: Array<Gd<InputEvent>> = Array::new();
            arr.push(event);
            self.mapping_overrides.insert(action_name, arr);
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

    fn ser_joy_button(&self, event: Gd<InputEvent>) -> Option<Dictionary> {
        let rcast: Result<Gd<InputEventJoypadButton>, _> = event.try_cast();
        let Ok(event) = rcast else {
            return None;
        };
        let mut dict = Dictionary::new();
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
        dict.insert("button_index".to_godot(), event.get_button_index());
        dict.insert("cancelled".to_godot(), event.is_canceled());
        dict.insert("double_click".to_godot(), event.is_double_click());
        dict.insert("factor".to_godot(), event.get_factor());
        dict.insert("pressed".to_godot(), event.is_pressed());

        Some(dict)
    }
}

const CONTROLS_SETTINGS_PATH: &str = "user://core/audio.json";
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
                } else if let Some(dict) = self.ser_mouse_button(event) {
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
        let custom_bindings_dictionary =
            sbind.internal_get_value("custom_bindings".to_godot(), Dictionary::new());
        for (key, value) in custom_bindings_dictionary.iter_shared() {
            if !self.can_bind(GString::from_variant(&key)) {
                continue;
            }
            if value.get_type() != VariantType::Array {
                continue;
            }
            let opt_event = Dictionary::try_from_variant(&value);
            let Ok(event_dict) = opt_event else {
                continue;
            };
            let Ok(type_id) = i32::try_from_variant(&event_dict.get_or_nil("type".to_godot()))
            else {
                continue;
            };
            let opt_event_type: Option<HandledInputEvents> = FromPrimitive::from_i32(type_id);
            let Some(event_type) = opt_event_type else {
                continue;
            };
            let action_name = GString::from_variant(&key);
            let values: Array<Dictionary> = Array::try_from_variant(&value).unwrap_or_default();
            for map in values.iter_shared() {
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
                }
            }
        }
        self.load_binds();
    }
}
