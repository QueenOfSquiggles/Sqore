use godot::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Object)]
pub struct DialogBuilder {
    nodes: Array<Dictionary>,
    #[base]
    node: Base<Object>,
}
#[godot_api]
impl DialogBuilder {
    #[func]
    fn push_text(&mut self, text: GString, character: GString, requirements: GString) {
        let mut node = Dictionary::new();
        node.set("type", "text");
        node.set("content", text);
        node.set("character", character);
        node.set("requires", requirements);
        self.nodes.push(node);
    }

    #[func]
    fn push_choice(&mut self, prompt: GString, character: GString) {
        let mut node = Dictionary::new();
        node.set("type", "choice");
        node.set("prompt", prompt);
        node.set("character", character);
        node.set("options", Array::<Variant>::new());
        self.nodes.push(node);
    }

    #[func]
    fn push_choice_option(&mut self, text: GString, requires: GString, action: GString) {
        let Some(prev) = &mut self.nodes.last() else {
            godot_warn!("Cannot push a choice option into an empty node!");
            return;
        };
        if String::try_from_variant(&prev.get("type").unwrap_or_default()).unwrap_or_default()
            != "choice"
        {
            godot_warn!(
                "When pushing a choice option, the previously pushed node must be a choice node!"
            );
            return;
        }
        let Ok(choices_array) =
            &mut Array::<Variant>::try_from_variant(&prev.get_or_nil("options"))
        else {
            godot_warn!("Failed to parse array from previous node's 'options' property");
            return;
        };
        let mut node = Dictionary::new();
        node.set("text", text);
        node.set("action", action);
        node.set("requires", requires);
        choices_array.push(node.to_variant());
        prev.set("options", choices_array.to_variant());
    }

    #[func]
    fn push_signal(&mut self, text: GString, character: GString, requirements: GString) {
        let mut node = Dictionary::new();
        node.set("content", text);
        node.set("character", character);
        node.set("requires", requirements);
        self.nodes.push(node);
    }

    #[func]
    fn push_action(&mut self, text: GString, character: GString, requirements: GString) {
        let mut node = Dictionary::new();
        node.set("content", text);
        node.set("character", character);
        node.set("requires", requirements);
        self.nodes.push(node);
    }

    #[func]
    fn get_dialog_track(&self) -> Dictionary {
        let mut dict = Dictionary::new();
        dict.set("nodes".to_godot(), self.nodes.clone());
        dict
    }
}
