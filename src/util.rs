use godot::engine::tween::{EaseType, TransitionType};
use godot::engine::{Engine, SceneTree, Tween};
use godot::prelude::*;

pub struct SquigglesUtil;

impl SquigglesUtil {
    pub fn get_scene_tree_global() -> Option<Gd<SceneTree>> {
        let Some(main_loop) = Engine::singleton().get_main_loop() else {
            return None;
        };
        let Ok(tree) = main_loop.try_cast::<SceneTree>() else {
            return None;
        };
        Some(tree)
    }

    pub fn add_child_deferred(parent: &mut Gd<Node>, child: &Gd<Node>) {
        parent.call_deferred(StringName::from("add_child"), &[child.to_variant()]);
    }

    pub fn create_tween(
        object: &mut Gd<Node>,
        ease: Option<EaseType>,
        trans: Option<TransitionType>,
    ) -> Option<Gd<Tween>> {
        object
            .create_tween()?
            .set_ease(ease.unwrap_or(EaseType::IN_OUT))?
            .set_trans(trans.unwrap_or(TransitionType::LINEAR))
    }
}
