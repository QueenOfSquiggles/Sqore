use std::collections::VecDeque;

use godot::{
    engine::{Engine, Json},
    prelude::*,
};

use crate::util::SquigglesUtil;

use super::{
    dialog_blackboard::{Blackboard, Entry},
    dialog_events::DialogEvents,
    dialog_gui::{DialogGUI, DialogSettings},
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
    pub fn load_track(&mut self, file_path: GString) {
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
        match DialogTrack::load_from_json(file_path.clone()) {
            Err(error) => Self::handle_dialog_error(error),
            Ok(track) => self.current_track = Some(track),
        }
        let Some(track) = &self.current_track else {
            godot_warn!("Failed to load a dialog track from {}", file_path);
            return;
        };

        // kill old GUI
        if let Some(gui) = self.gui.as_mut() {
            gui.queue_free();
        }

        // create and add GUI
        let mut gui = DialogGUI::new_alloc();

        gui.bind_mut().track = Some(VecDeque::from_iter(track.lines.clone()));
        SquigglesUtil::add_child_deferred(&mut root.upcast(), &gui.clone().upcast());
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

    pub fn handle_line_action(&mut self, line: &Line) {
        if self.event_bus.is_none() {
            self.init_event_bus();
        }
        match line {
            Line::Action { action } => {
                self.blackboard_action(action.to_godot());
            }
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
                let index = index.floor() as usize;
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

    pub fn jump(&mut self, index: usize) {
        godot_print!("Jumping to {}", index);
    }

    pub fn end(&mut self) {
        godot_print!("Ending dialog");
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
