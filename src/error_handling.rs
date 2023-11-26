use godot::prelude::*;

pub fn warn_unimplemented(node: Gd<Node>, func_name: &str) {
    let path = node.get_path();
    godot_warn!("Please implement function '{func_name}' for: {path}")
}
