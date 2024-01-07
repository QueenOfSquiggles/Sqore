use godot::{engine::Engine, prelude::*};

use crate::util::SquigglesUtil;

use super::{
    dialog_gui::{DialogGUI, DialogSettings},
    dialog_track::{DialogError, DialogTrack},
};

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct CoreDialog {
    current_track: Option<DialogTrack>,
    #[var]
    current_line_index: i32,
    #[var]
    override_settings: Option<Gd<DialogSettings>>,
    gui: Option<Gd<DialogGUI>>,
    #[base]
    base: Base<Object>,
}

#[godot_api]
impl IObject for CoreDialog {}

#[godot_api]
impl CoreDialog {
    pub const SIGNAL_TRACK_STARTED: &'static str = "track_started";
    pub const SIGNAL_TRACK_ENDED: &'static str = "track_ended";
    pub const SIGNAL_TRACK_SIGNAL: &'static str = "track_signal";
    pub const SINGLETON_NAME: &'static str = "CoreDialog";

    #[signal]
    fn track_ended(track: GString) {}
    #[signal]
    fn track_signal(name: GString, args: Array<Variant>) {}
    #[signal]
    fn track_started(track: GString) {}

    #[func]
    pub fn load_track(&mut self, file_path: GString) {
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
        let mut gui = DialogGUI::alloc_gd();
        gui.bind_mut().track = Some(track.clone());
        SquigglesUtil::add_child_deferred(&mut root.upcast(), &gui.clone().upcast());
    }

    fn handle_dialog_error(err: DialogError) {
        let reason = format!("{:#?}", err);
        godot_error!("DialogError: {}", reason);
        // godot_print!("DialogError : {}", reason);
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
