use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct DialogEvents {
    #[base]
    node: Base<Node>,
}

#[godot_api]
impl INode for DialogEvents {}

#[godot_api]
impl DialogEvents {
    pub const SIGNAL_TRACK_STARTED: &'static str = "track_started";
    pub const SIGNAL_TRACK_ENDED: &'static str = "track_ended";
    pub const SIGNAL_TRACK_SIGNAL: &'static str = "track_signal";

    #[signal]
    fn track_ended(track: GString) {}
    #[signal]
    fn track_signal(name: GString, args: Array<Variant>) {}
    #[signal]
    fn track_started(track: GString) {}
}
