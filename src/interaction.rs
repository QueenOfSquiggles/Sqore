use crate::error_handling::*;
use godot::engine::{Area3D, CharacterBody3D, IArea3D, IRayCast3D, RayCast3D, StaticBody3D};
use godot::prelude::*;
use once_cell::sync::Lazy;

// TODO: add signals to the interactobjects for on_select and on_deselect so users can just hook into signals without needing to implement other stuff if they so choose

// these are accessed by calling .clone(). Normally I'd dislike this, but StringName is ref-counted so duplicating it is almost completely free
static METHOD_SELECT: Lazy<StringName> = Lazy::new(|| StringName::from("on_select"));
static METHOD_DESELECT: Lazy<StringName> = Lazy::new(|| StringName::from("on_deselect"));
static METHOD_INTERACT: Lazy<StringName> = Lazy::new(|| StringName::from("interact"));

static SIGNAL_ON_INTERACT: Lazy<StringName> = Lazy::new(|| StringName::from("on_interacted"));
static SIGNAL_CAN_INTERACT: Lazy<StringName> = Lazy::new(|| StringName::from("can_interact"));
static SIGNAL_ON_SELECTED: Lazy<StringName> = Lazy::new(|| StringName::from("on_selected"));
static SIGNAL_ON_DESELECTED: Lazy<StringName> = Lazy::new(|| StringName::from("on_deselected"));

#[derive(GodotClass)]
#[class(init, base=RayCast3D)]
struct InteractRaycast3D {
    #[export]
    filter_groups: PackedStringArray,
    target: Option<Gd<Node3D>>,
    #[base]
    base: Base<RayCast3D>,
}

#[derive(GodotClass)]
#[class(init, base=Area3D)]
struct InteractArea3D {
    #[export]
    filter_groups: PackedStringArray,
    target: Option<Gd<Node3D>>,
    #[base]
    base: Base<Area3D>,
}

#[derive(GodotClass)]
#[class(init, base=Area3D)]
struct InteractionObjectArea3D {
    #[base]
    base: Base<Area3D>,
}

#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
struct InteractionObjectStaticBody3D {
    #[base]
    base: Base<StaticBody3D>,
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct InteractionObjectCharacterBody3D {
    #[base]
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl InteractRaycast3D {
    #[signal]
    fn can_interact(is_able_to_interact: bool) {}

    #[func]
    fn do_interact(&mut self) {
        if let Some(mut target) = self.target.clone() {
            target.call(METHOD_INTERACT.clone(), &[]);
        }
    }
}
#[godot_api]
impl IRayCast3D for InteractRaycast3D {
    fn physics_process(&mut self, _delta: f64) {
        if let Some(collider) = self.base.get_collider() {
            let option_typed: Option<Gd<Node3D>> = collider.try_cast();
            if let Some(mut coll3d) = option_typed {
                let mut in_group = self.filter_groups.is_empty();
                for g in self.filter_groups.as_slice() {
                    if coll3d.is_in_group(StringName::from(g)) {
                        in_group = true;
                        break;
                    }
                }
                if in_group && coll3d.has_method(METHOD_INTERACT.clone()) {
                    // valid object for interaction
                    let mut has_changed = false;
                    if let Some(mut prev) = self.target.clone() {
                        if !prev.eq(&coll3d) {
                            if prev.has_method(METHOD_DESELECT.clone()) {
                                prev.call(METHOD_DESELECT.clone(), &[]);
                            }
                            has_changed = true;
                        }
                    } else {
                        has_changed = true;
                    }
                    if has_changed {
                        if coll3d.has_method(METHOD_SELECT.clone()) {
                            coll3d.call(METHOD_SELECT.clone(), &[]);
                        }
                        self.target = Some(coll3d);
                        self.base
                            .emit_signal(SIGNAL_CAN_INTERACT.clone(), &[true.to_variant()]);
                    }
                }
            }
        } else if let Some(mut prev) = self.target.clone() {
            if prev.has_method(METHOD_DESELECT.clone()) {
                prev.call(METHOD_DESELECT.clone(), &[]);
            }
            self.target = None;
            self.base
                .emit_signal(SIGNAL_CAN_INTERACT.clone(), &[false.to_variant()]);
        }
    }
}

#[godot_api]
impl InteractArea3D {
    #[signal]
    fn can_interact(is_able_to_interact: bool) {}
    #[func]
    fn do_interact(&mut self) {
        if let Some(mut target) = self.target.clone() {
            target.call(METHOD_INTERACT.clone(), &[]);
        }
    }
}

#[godot_api]
impl IArea3D for InteractArea3D {
    fn physics_process(&mut self, _delta: f64) {
        let mut target_buffer: Array<Gd<Node3D>> = Array::new();
        target_buffer.extend_array(self.base.get_overlapping_bodies());
        let temp = self.base.get_overlapping_areas();
        for t in temp.iter_shared() {
            target_buffer.push(t.upcast());
        }

        if target_buffer.is_empty() {
            return;
        }

        let mut closest: Option<Gd<Node3D>> = None;
        let mut dist = f32::MAX;
        for target in target_buffer.iter_shared() {
            let mut in_group = self.filter_groups.is_empty();
            for g in self.filter_groups.as_slice() {
                if target.is_in_group(StringName::from(g)) {
                    in_group = true;
                    break;
                }
            }
            if !in_group || !target.has_method(METHOD_INTERACT.clone()) {
                continue;
            }
            let d = self
                .base
                .get_global_position()
                .distance_squared_to(target.get_global_position());
            if d < dist {
                dist = d;
                closest = Some(target);
            }
        }

        if let Some(mut coll3d) = closest {
            if let Some(mut prev) = self.target.clone() {
                if !prev.eq(&coll3d) {
                    if prev.has_method(METHOD_DESELECT.clone()) {
                        prev.call(METHOD_DESELECT.clone(), &[]);
                    }
                    if coll3d.has_method(METHOD_SELECT.clone()) {
                        coll3d.call(METHOD_SELECT.clone(), &[]);
                    }
                    self.target = Some(coll3d);
                    self.base
                        .emit_signal(SIGNAL_CAN_INTERACT.clone(), &[true.to_variant()]);
                }
            }
        } else if let Some(mut prev) = self.target.clone() {
            if prev.has_method(METHOD_DESELECT.clone()) {
                prev.call(METHOD_DESELECT.clone(), &[]);
            }
            self.target = None;
            self.base
                .emit_signal(SIGNAL_CAN_INTERACT.clone(), &[false.to_variant()]);
        }
    }
}

#[godot_api]
impl InteractionObjectArea3D {
    #[signal]
    fn on_interacted() {}
    #[signal]
    fn on_selected() {}
    #[signal]
    fn on_deselected() {}

    #[func]
    fn on_select(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_SELECTED.clone(), &[]);
    }
    #[func]
    fn on_deselect(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_DESELECTED.clone(), &[]);
    }

    #[func]
    fn interact(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_INTERACT.clone(), &[]);
    }

    #[func]
    fn get_active(&self) -> bool {
        warn_unimplemented(self.base.clone().upcast(), "get_active");
        true
    }

    #[func]
    fn get_interact_name(&self) -> GString {
        warn_unimplemented(self.base.clone().upcast(), "get_interact_name");
        GString::from("No name given")
    }
}

#[godot_api]
impl InteractionObjectStaticBody3D {
    #[signal]
    fn on_interacted() {}
    #[signal]
    fn on_selected() {}
    #[signal]
    fn on_deselected() {}

    #[func]
    fn on_select(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_SELECTED.clone(), &[]);
    }
    #[func]
    fn on_deselect(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_DESELECTED.clone(), &[]);
    }

    #[func]
    fn interact(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_INTERACT.clone(), &[]);
    }

    #[func]
    fn get_active(&self) -> bool {
        warn_unimplemented(self.base.clone().upcast(), "get_active");
        true
    }

    #[func]
    fn get_interact_name(&self) -> GString {
        warn_unimplemented(self.base.clone().upcast(), "get_interact_name");
        GString::from("No name given")
    }
}

#[godot_api]
impl InteractionObjectCharacterBody3D {
    #[signal]
    fn on_interacted() {}
    #[signal]
    fn on_selected() {}
    #[signal]
    fn on_deselected() {}

    #[func]
    fn on_select(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_SELECTED.clone(), &[]);
    }
    #[func]
    fn on_deselect(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_DESELECTED.clone(), &[]);
    }

    #[func]
    fn interact(&mut self) {
        if !self.base.is_inside_tree() {
            return;
        }
        self.base.emit_signal(SIGNAL_ON_INTERACT.clone(), &[]);
    }

    #[func]
    fn get_active(&self) -> bool {
        warn_unimplemented(self.base.clone().upcast(), "get_active");
        true
    }

    #[func]
    fn get_interact_name(&self) -> GString {
        warn_unimplemented(self.base.clone().upcast(), "get_interact_name");
        GString::from("No name given")
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_raycast_detect() {
        // Error thrown because this requires the godot engine for testing
        // basically, testing requires a scene tree, which requires godot. So as a library we can't really test directly. Unless we want to test something that doesn't rely on Godot's engine stuff.
        // - - - - - - -
        // use godot::{
        //     engine::{CollisionShape3D, SphereShape3D},
        //     obj::UserClass,
        // };
        // let mut ray: Gd<InteractRaycast3D> = InteractRaycast3D::alloc_gd();
        // let mut object = InteractionObjectArea3D::alloc_gd();
        // ray.set_collide_with_bodies(true);
        // ray.set_global_position(Vector3::new(0.0, 0.0, -1.0));
        // ray.set_target_position(Vector3::new(0.0, 0.0, 5.0));
        // object.set_global_position(Vector3::new(0.0, 0.0, 1.0));
        // let mut obj_coll = CollisionShape3D::new_alloc();
        // obj_coll.set_shape(SphereShape3D::new().upcast());
        // object.add_child(obj_coll.upcast());

        // let scene = SceneTree::new_alloc();
        // if let Some(mut root) = scene.get_root() {
        //     root.add_child(ray.clone().upcast());
        //     root.add_child(object.clone().upcast());
        //     ray.force_raycast_update();
        //     assert_eq!(ray.is_colliding(), true);
        // }
    }
}
