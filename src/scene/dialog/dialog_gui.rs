use godot::{
    engine::{CanvasLayer, ICanvasLayer, InputEvent},
    prelude::*,
};

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct DialogGUI {
    #[base]
    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for DialogGUI {
    fn enter_tree(&mut self) {
        //pass
    }

    fn ready(&mut self) {
        //pass
    }

    fn input(&mut self, event: Gd<InputEvent>) {}

    fn exit_tree(&mut self) {
        //pass
    }
}
