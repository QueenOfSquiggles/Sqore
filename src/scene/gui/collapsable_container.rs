use godot::engine::Button;
use godot::engine::Control;
use godot::engine::IVBoxContainer;
use godot::engine::VBoxContainer;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(tool, init, base=VBoxContainer)]
pub struct CollapsingVBoxContainer {
    #[export]
    #[var(get,set = set_heading_text)]
    heading_text: GString,

    #[export]
    default_visibility: bool,

    heading: Option<Gd<Button>>,
    base: Base<VBoxContainer>,
}

#[godot_api]
impl IVBoxContainer for CollapsingVBoxContainer {
    fn enter_tree(&mut self) {
        let mut heading_button = Button::new_alloc();
        self.heading = Some(heading_button.clone());
        heading_button.set_text(self.heading_text.clone());
        heading_button.set_toggle_mode(true);
        heading_button.set_pressed(self.default_visibility);
        self.base_mut().add_child(heading_button.clone().upcast());
        self.base_mut()
            .move_child(heading_button.clone().upcast(), 0);
        heading_button.connect(
            StringName::from("toggled"),
            Callable::from_object_method(&self.to_gd(), "on_heading_toggle"),
        );
        self.on_heading_toggle(self.default_visibility);
    }

    fn ready(&mut self) {
        // is anything needed here??
    }
}

#[godot_api]
impl CollapsingVBoxContainer {
    #[func]
    fn on_heading_toggle(&mut self, is_toggled: bool) {
        let mut children: Vec<Gd<Node>> = self.base_mut().get_children().iter_shared().collect();
        if let Some(btn) = &self.heading {
            let btn_base = &btn.clone().upcast::<Node>();
            children = children.into_iter().filter(|p| p != btn_base).collect();
        }
        for child in children {
            if let Ok(control) = &mut child.clone().try_cast::<Control>() {
                control.set_visible(is_toggled);
            }
            if let Ok(node2d) = &mut child.clone().try_cast::<Node2D>() {
                node2d.set_visible(is_toggled);
            }
            if let Ok(node3d) = &mut child.clone().try_cast::<Node3D>() {
                node3d.set_visible(is_toggled);
            }
        }
    }

    #[func]
    fn set_heading_text(&mut self, n_text: GString) {
        self.heading_text = n_text;
        if let Some(btn) = &mut self.heading {
            btn.set_text(self.heading_text.clone());
        }
    }
}
