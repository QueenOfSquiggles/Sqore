use std::collections::VecDeque;

use godot::{
    engine::{Engine, Json},
    prelude::*,
};

use crate::util::SquigglesUtil;

use super::{
    dialog_blackboard::Blackboard,
    dialog_events::DialogEvents,
    dialog_gui::{DialogGUI, DialogSettings},
    dialog_track::{DialogError, DialogTrack, Line},
};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct CoreDialog {
    current_track: Option<DialogTrack>,
    #[var]
    current_line_index: i32,
    #[var]
    override_settings: Option<Gd<DialogSettings>>,
    #[var]
    event_bus: Option<Gd<DialogEvents>>,
    gui: Option<Gd<DialogGUI>>,
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

        gui.bind_mut().track = Some(VecDeque::from_iter(track.lines.clone().into_iter()));
        SquigglesUtil::add_child_deferred(&mut root.upcast(), &gui.clone().upcast());
    }

    fn handle_dialog_error(err: DialogError) {
        godot_error!("DialogError: {:#?}", err);
    }

    pub fn handle_line_action(&mut self, line: &Line) {
        if self.event_bus.is_none() {
            self.init_event_bus();
        }
        match line {
            Line::Action { action } => self.blackboard.parse_action(action.clone()),
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
