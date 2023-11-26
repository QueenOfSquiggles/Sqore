use godot::{
    engine::{IMarker3D, Marker3D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Marker3D)]
struct DelayedInstance {
    #[export]
    scene: Option<Gd<PackedScene>>,
    #[export]
    delay_seconds: f32,

    #[base]
    base: Base<Marker3D>,
}

#[godot_api]
impl IMarker3D for DelayedInstance {
    fn init(base: Base<Marker3D>) -> Self {
        let mut s = Self {
            scene: None,
            delay_seconds: 1.0,
            base,
        };
        s.base.set_gizmo_extents(1.0);
        s
    }
    fn ready(&mut self) {
        if let Some(mut tree) = self.base.get_tree() {
            let mut timer = tree.create_timer(self.delay_seconds as f64).unwrap();
            timer.connect(
                StringName::from("timeout"),
                Callable::from_object_method(
                    self.base.clone().cast() as Gd<DelayedInstance>,
                    "instance_stored",
                ),
            );
        }
    }
}

#[godot_api]
impl DelayedInstance {
    #[func]
    fn instance_stored(&self) {
        if let Some(mut parent) = self.base.get_parent() {
            if let Some(packed) = self.scene.clone() {
                if let Some(scene) = packed.instantiate() {
                    if let Some(mut node3d) = scene.clone().try_cast() as Option<Gd<Node3D>> {
                        // if the scene node root is a Node3D, apply the transform to allow things
                        node3d.set_global_transform(self.base.get_global_transform());
                        parent.add_child(node3d.upcast());
                    } else {
                        // Unhandled node type. Just instance it and hope.
                        parent.add_child(scene);
                    }
                }
            }
        }
    }
}
