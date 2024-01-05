use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct DialogTrack {
    #[export]
    lines: Array<Gd<DialogTrackLine>>,
    #[base]
    node: Base<Resource>,
}

#[godot_api]
impl IResource for DialogTrack {}

#[godot_api]
impl DialogTrack {}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct DialogTrackLine {
    #[export]
    label: GString,
    #[export]
    text: GString,
    #[export]
    options: Array<GString>,
    #[base]
    node: Base<Resource>,
}

#[godot_api]
impl IResource for DialogTrackLine {}

#[godot_api]
impl DialogTrackLine {}
