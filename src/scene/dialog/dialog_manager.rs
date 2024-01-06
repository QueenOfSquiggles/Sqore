use godot::{engine::Engine, prelude::*};

use super::{
    dialog_gui::DialogGUI,
    dialog_track::{DialogError, DialogTrack},
};

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct CoreDialog {
    current_track: Option<DialogTrack>,
    #[var]
    current_line_index: i32,
    gui: Option<DialogGUI>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for CoreDialog {}

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
        match DialogTrack::load_from_json(file_path) {
            Err(error) => Self::handle_dialog_error(error),
            Ok(track) => self.current_track = Some(track),
        }
        let Some(track) = &self.current_track else {
            return;
        };
        for (index, line) in track.lines.iter().enumerate() {
            println!("Dialog Track [line {}] {:#?}", index, line);
        }
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
