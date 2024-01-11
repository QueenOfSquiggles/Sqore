use std::collections::VecDeque;

use godot::{
    engine::{Engine, Json},
    prelude::*,
};

use crate::util::SquigglesUtil;

use super::{
    dialog_blackboard::{Blackboard, Entry},
    dialog_events::DialogEvents,
    dialog_gui::DialogGUI,
    dialog_settings::DialogSettings,
    dialog_track::{DialogError, DialogTrack, Line},
};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct CoreDialog {
    current_track: Option<DialogTrack>,

    #[var]
    override_settings: Option<Gd<DialogSettings>>,
    #[var]
    pub event_bus: Option<Gd<DialogEvents>>,
    pub gui: Option<Gd<DialogGUI>>,
    pub blackboard: Blackboard,
    #[base]
    base: Base<Object>,
}

#[godot_api]
impl IObject for CoreDialog {}

#[godot_api]
impl CoreDialog {
    pub const SINGLETON_NAME: &'static str = "CoreDialog";

    #[func]
    pub fn init_event_bus(&mut self) {
        if self.event_bus.is_some() {
            godot_warn!("No need to call init_event_bus! Already initialized!");
            return;
        }
        let Some(tree) = SquigglesUtil::get_scene_tree_global() else {
            godot_warn!("Failed to find SceneTree when initializing event bus");
            return;
        };
        let Some(root) = &mut tree.get_root() else {
            return;
        };
        let event_bus = DialogEvents::new_alloc();
        self.event_bus = Some(event_bus.clone());
        root.call_deferred(StringName::from("add_child"), &[event_bus.to_variant()]);
    }

    #[func]
    pub fn load_track_file(&mut self, file_path: GString) {
        let result = DialogTrack::load_from_json(file_path.clone());
        if result.is_ok() {
            self.current_track = Some(result.unwrap());
            self.load_track();
        } else {
            Self::handle_dialog_error(result.unwrap_err());
        }
    }

    #[func]
    pub fn load_track_text(&mut self, track_text: GString) {
        let result = DialogTrack::load_from_text(track_text, "<internal text>".to_godot());
        if result.is_ok() {
            self.current_track = Some(result.unwrap());
            self.load_track();
        } else {
            Self::handle_dialog_error(result.unwrap_err());
        }
    }

    #[func]
    pub fn load_track_dict(&mut self, track_dict: Dictionary) {
        let result = DialogTrack::load_from_dict(track_dict, "<internal dict>".to_godot());
        if result.is_ok() {
            self.current_track = Some(result.unwrap());
            self.load_track();
        } else {
            Self::handle_dialog_error(result.unwrap_err());
        }
    }

    pub fn load_track(&mut self) {
        if self.event_bus.is_none() {
            self.init_event_bus();
        }
        // ensure is in tree
        let Some(tree) = SquigglesUtil::get_scene_tree_global() else {
            godot_warn!("failed to load godot scene tree for CoreDialog");
            return;
        };
        let Some(root) = tree.get_root() else {
            return;
        };

        // load track data
        let Some(track) = &self.current_track else {
            godot_warn!("Failed to load a dialog track");
            return;
        };

        // kill old GUI
        self.gui.as_mut().map(|g| {
            if g.is_instance_valid() {
                g.queue_free()
            }
        });

        // Creates Really Bad Panic
        // if let Some(gui) = &mut self.gui.clone() {
        //     if gui.is_instance_valid() {
        //         gui.queue_free();
        //     }
        // }

        // create and add GUI
        let mut gui = DialogGUI::new_alloc();

        SquigglesUtil::add_child_deferred(&mut root.upcast(), &gui.clone().upcast());
        gui.bind_mut().track = Some(VecDeque::from_iter(track.lines.clone()));
        self.gui = Some(gui);
    }

    fn handle_dialog_error(err: DialogError) {
        godot_error!("DialogError: {:#?}", err);
    }
    #[func]
    pub fn make_choice_selection(&mut self, selection: i32) -> bool {
        let Some(gui) = &mut self.gui else {
            return false;
        };
        gui.bind_mut().make_dialog_choice(selection)
    }

    pub fn handle_dialog_signal(&mut self, line: &Line) {
        if self.event_bus.is_none() {
            self.init_event_bus();
        }
        match line {
            Line::Signal { name, args } => {
                let Some(bus) = &mut self.event_bus else {
                    return;
                };
                bus.emit_signal(
                    StringName::from(DialogEvents::SIGNAL_TRACK_SIGNAL),
                    &[
                        name.to_variant(),
                        Array::from_iter(args.iter().map(|s| Json::parse_string(s.to_godot())))
                            .to_variant(),
                    ],
                );
            }
            _ => (),
        }
    }

    pub fn handle_line_query(&mut self, query: String) -> bool {
        self.blackboard.parse_query(query)
    }

    #[func]
    pub fn blackboard_action(&mut self, action: GString) {
        self.blackboard.parse_action(action.to_string());
        let Some((event_name, event_arg)) = self.blackboard.get_event() else {
            return;
        };
        match event_name.as_str() {
            // TODO handle events with pub const value
            "end" => {
                let Some(gui) = &mut self.gui else {
                    return;
                };
                gui.bind_mut().update_track(VecDeque::new());
            }
            "jump" => {
                let Entry::Number(index) = event_arg else {
                    return;
                };
                let index = (index.floor() - 1f32) as usize;
                let Some(gui) = &mut self.gui else {
                    return;
                };
                let n_track = self.current_track.clone().unwrap();
                let mut lines = VecDeque::from_iter(n_track.lines.iter().cloned());
                for _ in 0..index {
                    let _ = lines.pop_front();
                }
                gui.bind_mut().update_track(lines);
            }
            _ => godot_error!("Unhandled internal event! event: \"{}\"", event_name),
        }
        self.blackboard.mark_event_handled();
    }

    #[func]
    pub fn blackboard_query(&mut self, query: GString) -> bool {
        self.blackboard.parse_query(query.to_string())
    }

    #[func]
    pub fn blackboard_debug_dump(&self) {
        godot_print!("{:#?}", self.blackboard);
        // self.blackboard.debug_print();
    }

    pub fn blackboard_parse(&self, text: String) -> String {
        self.blackboard.format_text(text)
    }

    pub fn singleton() -> Gd<CoreDialog> {
        let Some(vol) = Engine::singleton().get_singleton(StringName::from(Self::SINGLETON_NAME))
        else {
            panic!("Failed to find engine singleton for CoreGlobals. You must access this after it is registered!");
        };
        let res_core: Result<Gd<CoreDialog>, Gd<_>> = vol.try_cast();
        let Ok(core) = res_core else {
            panic!("Failed to cast engine singleton for CoreGlobals. This should never happen!");
        };
        core
    }
}