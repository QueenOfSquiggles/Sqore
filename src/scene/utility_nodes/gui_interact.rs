use godot::{
    engine::{AudioStream, Control},
    prelude::*,
};

type Sfx = Option<Gd<AudioStream>>;
#[derive(GodotClass)]
#[class(init, base=Node)]
struct GuiInteract {
    #[export]
    auto_focus: bool,
    #[export]
    hover_sfx: Sfx,
    #[export]
    interact_sfx: Sfx,
    #[export]
    appear_sfx: Sfx,
    #[export]
    disappear_sfx: Sfx,

    audio_player: Option<Gd<AudioStreamPlayer>>,
    #[base]
    base: Base<Node>,
}

#[godot_api]
impl INode for GuiInteract {
    fn ready(&mut self) {
        // load audio stream player
        let player = AudioStreamPlayer::new_alloc();
        self.audio_player = Some(player.clone());
        self.base_mut().add_child(player.upcast());
        // TODO: use a global setting to assign the audio bus and volume

        // get parent
        let Some(node_parent) = self.base().get_parent() else {
            return;
        };
        let rcast: Result<Gd<Control>, _> = node_parent.try_cast();
        let Ok(mut control_parent) = rcast else {
            return;
        };

        // grab focus
        if self.auto_focus {
            control_parent.grab_focus();
        }

        // connect signals
        self.try_connect(&mut control_parent, "pressed", "on_interact");
        self.try_connect(
            &mut control_parent,
            "visibility_changed",
            "on_visiblity_changed",
        );
        self.try_connect(&mut control_parent, "focus_entered", "on_hover");
        self.try_connect(&mut control_parent, "mouse_entered", "on_mouse_enter");
    }
}

#[godot_api]
impl GuiInteract {
    #[func]
    fn on_visiblity_changed(&mut self) {
        if let Some(node_parent) = self.base().get_parent() {
            let rcast: Result<Gd<Control>, _> = node_parent.try_cast();
            if let Ok(parent) = rcast {
                if parent.is_visible() {
                    if let Some(sfx) = &self.appear_sfx {
                        self.try_play_sfx(sfx.clone());
                    }
                } else if let Some(sfx) = &self.disappear_sfx {
                    self.try_play_sfx(sfx.clone());
                }
            }
        }
    }

    #[func]
    fn on_mouse_enter(&mut self) {
        // get parent
        let Some(node_parent) = self.base().get_parent() else {
            return;
        };
        let rcast: Result<Gd<Control>, _> = node_parent.try_cast();
        let Ok(mut control_parent) = rcast else {
            return;
        };

        // grab focus
        if self.auto_focus {
            control_parent.grab_focus();
        }
    }

    #[func]
    fn on_hover(&mut self) {
        if let Some(sfx) = &self.hover_sfx {
            self.try_play_sfx(sfx.clone());
        }
    }
    #[func]
    fn on_interact(&mut self) {
        if let Some(sfx) = &self.interact_sfx {
            self.try_play_sfx(sfx.clone());
        }
    }

    fn try_play_sfx(&mut self, stream: Gd<AudioStream>) {
        if let Some(player) = &mut self.audio_player {
            player.stop();
            player.set_stream(stream);
            player.play();
        }
    }
    fn try_connect(&self, node: &mut Gd<Control>, signal_name: &str, callable: &str) -> bool {
        let signal = StringName::from(signal_name);
        if !node.has_signal(signal.clone()) {
            return false;
        }
        node.connect(
            signal,
            Callable::from_object_method(&self.to_gd(), callable),
        );

        true
    }
}
