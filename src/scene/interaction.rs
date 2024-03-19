use godot::engine::{
    Area3D, CharacterBody3D, IArea3D, IRayCast3D, RayCast3D, RigidBody3D, StaticBody3D,
};
use godot::prelude::*;

// these are accessed by calling .clone(). Normally I'd dislike this, but StringName is ref-counted so duplicating it is almost completely free
const METHOD_SELECT: &str = "on_select";
const METHOD_DESELECT: &str = "on_deselect";
const METHOD_INTERACT: &str = "interact";

const SIGNAL_ON_INTERACT: &str = "on_interacted";
const SIGNAL_CAN_INTERACT: &str = "can_interact";
const SIGNAL_ON_SELECTED: &str = "on_selected";
const SIGNAL_ON_DESELECTED: &str = "on_deselected";

#[derive(GodotClass)]
#[class(init, base=RayCast3D)]
struct InteractRaycast3D {
    #[export]
    filter_groups: PackedStringArray,
    #[var]
    target: Option<Gd<Node3D>>,

    base: Base<RayCast3D>,
}

#[derive(GodotClass)]
#[class(init, base=Area3D)]
struct InteractArea3D {
    #[export]
    filter_groups: PackedStringArray,
    #[var]
    target: Option<Gd<Node3D>>,

    base: Base<Area3D>,
}

#[derive(GodotClass)]
#[class(init, base=Area3D)]
struct InteractionObjectArea3D {
    #[export]
    is_active: bool,
    #[export]
    active_name: GString,

    base: Base<Area3D>,
}

#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
struct InteractionObjectStaticBody3D {
    #[export]
    is_active: bool,
    #[export]
    active_name: GString,

    base: Base<StaticBody3D>,
}

#[derive(GodotClass)]
#[class(init, base=CharacterBody3D)]
struct InteractionObjectCharacterBody3D {
    #[export]
    is_active: bool,
    #[export]
    active_name: GString,

    base: Base<CharacterBody3D>,
}
#[derive(GodotClass)]
#[class(init, base=RigidBody3D)]
struct InteractionObjectRigidBody3D {
    #[export]
    is_active: bool,
    #[export]
    active_name: GString,

    base: Base<RigidBody3D>,
}

fn is_active_interactable_object(node: Gd<Node>) -> bool {
    let oacast: Result<Gd<InteractionObjectArea3D>, _> = node.clone().try_cast();
    if let Ok(oa) = oacast {
        return oa.bind().get_active();
    }
    let oscast: Result<Gd<InteractionObjectStaticBody3D>, _> = node.clone().try_cast();
    if let Ok(os) = oscast {
        return os.bind().get_active();
    }
    let occast: Result<Gd<InteractionObjectCharacterBody3D>, _> = node.clone().try_cast();
    if let Ok(oc) = occast {
        return oc.bind().get_active();
    }
    let orcast: Result<Gd<InteractionObjectRigidBody3D>, _> = node.try_cast();
    if let Ok(or) = orcast {
        return or.bind().get_active();
    }

    true
}

#[godot_api]
impl InteractRaycast3D {
    #[signal]
    fn can_interact(is_able_to_interact: bool) {}

    #[func]
    fn do_interact(&mut self) {
        if let Some(target) = self.target.as_mut() {
            if target.is_instance_valid() {
                target.call_deferred(StringName::from(METHOD_INTERACT), &[]);
            }
        }
    }
}
#[godot_api]
impl IRayCast3D for InteractRaycast3D {
    fn physics_process(&mut self, _delta: f64) {
        if let Some(collider) = self.base().get_collider() {
            let mut option_typed: Result<Gd<Node3D>, Gd<Object>> = collider.try_cast();
            if let Ok(coll3d) = option_typed.as_mut() {
                let mut in_group = self.filter_groups.is_empty();
                for g in self.filter_groups.as_slice() {
                    if coll3d.is_in_group(StringName::from(g)) {
                        in_group = true;
                        break;
                    }
                }
                if in_group
                    && coll3d.has_method(StringName::from(METHOD_INTERACT))
                    && is_active_interactable_object(coll3d.clone().upcast())
                {
                    // valid object for interaction
                    let mut has_changed = false;
                    if let Some(prev) = self.target.as_mut() {
                        if !prev.is_instance_valid() {
                            has_changed = true;
                        } else if prev.instance_id_unchecked() != coll3d.instance_id_unchecked() {
                            if prev.has_method(StringName::from(METHOD_DESELECT)) {
                                prev.call(StringName::from(METHOD_DESELECT), &[]);
                            }
                            has_changed = true;
                        }
                    } else {
                        has_changed = true;
                    }
                    if has_changed {
                        if coll3d.has_method(StringName::from(METHOD_SELECT)) {
                            coll3d.call(StringName::from(METHOD_SELECT), &[]);
                        }
                        self.target = Some(coll3d.to_owned());
                        self.base_mut().emit_signal(
                            StringName::from(SIGNAL_CAN_INTERACT),
                            &[true.to_variant()],
                        );
                    }
                }
            }
        } else if let Some(prev) = self.target.as_mut() {
            if prev.is_instance_valid() && prev.has_method(StringName::from(METHOD_DESELECT)) {
                prev.call(StringName::from(METHOD_DESELECT), &[]);
            }
            self.target = None;
            self.base_mut()
                .emit_signal(StringName::from(SIGNAL_CAN_INTERACT), &[false.to_variant()]);
        }
    }
}

#[godot_api]
impl InteractArea3D {
    #[signal]
    fn can_interact(is_able_to_interact: bool) {}
    #[func]
    fn do_interact(&mut self) {
        if let Some(target) = self.target.as_mut() {
            target.call_deferred(METHOD_INTERACT.into(), &[]);
        }
    }
}

#[godot_api]
impl IArea3D for InteractArea3D {
    fn physics_process(&mut self, _delta: f64) {
        let mut target_buffer: Array<Gd<Node3D>> = Array::new();
        target_buffer.extend_array(self.base().get_overlapping_bodies());
        let temp = self.base().get_overlapping_areas();
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
            if !in_group || !target.has_method(METHOD_INTERACT.into()) {
                continue;
            }
            if !is_active_interactable_object(target.clone().upcast()) {
                continue;
            }

            let d = self
                .base()
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
                    if prev.has_method(StringName::from(METHOD_DESELECT)) {
                        prev.call(StringName::from(METHOD_DESELECT), &[]);
                    }
                    if coll3d.has_method(StringName::from(METHOD_SELECT)) {
                        coll3d.call(StringName::from(METHOD_SELECT), &[]);
                    }
                    self.target = Some(coll3d);
                    self.base_mut()
                        .emit_signal(StringName::from(SIGNAL_CAN_INTERACT), &[true.to_variant()]);
                }
            }
        } else if let Some(mut prev) = self.target.clone() {
            if prev.has_method(StringName::from(METHOD_DESELECT)) {
                prev.call(StringName::from(METHOD_DESELECT), &[]);
            }
            self.target = None;
            self.base_mut()
                .emit_signal(StringName::from(SIGNAL_CAN_INTERACT), &[false.to_variant()]);
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
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_SELECTED), &[]);
    }
    #[func]
    fn on_deselect(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_DESELECTED), &[]);
    }

    #[func]
    fn interact(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_INTERACT), &[]);
    }

    #[func]
    fn get_active(&self) -> bool {
        self.is_active
    }

    #[func]
    fn get_interact_name(&self) -> GString {
        self.active_name.clone()
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
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_SELECTED), &[]);
    }
    #[func]
    fn on_deselect(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_DESELECTED), &[]);
    }

    #[func]
    fn interact(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_INTERACT), &[]);
    }
    #[func]
    fn get_active(&self) -> bool {
        self.is_active
    }

    #[func]
    fn get_interact_name(&self) -> GString {
        self.active_name.clone()
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
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_SELECTED), &[]);
    }
    #[func]
    fn on_deselect(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_DESELECTED), &[]);
    }

    #[func]
    fn interact(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_INTERACT), &[]);
    }
    #[func]
    fn get_active(&self) -> bool {
        self.is_active
    }

    #[func]
    fn get_interact_name(&self) -> GString {
        self.active_name.clone()
    }
}

#[godot_api]
impl InteractionObjectRigidBody3D {
    #[signal]
    fn on_interacted() {}
    #[signal]
    fn on_selected() {}
    #[signal]
    fn on_deselected() {}

    #[func]
    fn on_select(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_SELECTED), &[]);
    }
    #[func]
    fn on_deselect(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_DESELECTED), &[]);
    }

    #[func]
    fn interact(&mut self) {
        if !self.base().is_inside_tree() {
            return;
        }
        self.base_mut()
            .emit_signal(StringName::from(SIGNAL_ON_INTERACT), &[]);
    }

    #[func]
    fn get_active(&self) -> bool {
        self.is_active
    }

    #[func]
    fn get_interact_name(&self) -> GString {
        self.active_name.clone()
    }
}
