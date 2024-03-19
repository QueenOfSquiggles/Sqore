use godot::{
    engine::{utilities::clampf, IMarker3D, Marker3D},
    prelude::*,
};

pub const CAMERA_BRAIN_GROUP: &str = "camera_brain";

/// A single `CameraBrain3D` is required for virtual cameras to work
#[derive(GodotClass)]
#[class(base=Camera3D)]
pub struct CameraBrain3D {
    cam_buffer: Vec<Gd<VirtualCamera3D>>,
    last_cam: bool,
    base: Base<Camera3D>,
}

#[godot_api]
impl ICamera3D for CameraBrain3D {
    fn init(base: Base<Camera3D>) -> Self {
        Self {
            cam_buffer: Vec::new(),
            last_cam: false,
            base,
        }
    }
    fn ready(&mut self) {
        self.base_mut().add_to_group(CAMERA_BRAIN_GROUP.into());
    }

    fn process(&mut self, delta: f64) {
        if let Some(vcam) = self.cam_buffer.last() {
            let mut n_trans = vcam.get_global_transform();
            if vcam.bind().use_lerp && self.last_cam {
                let factor = 1.0 / vcam.bind().lerp_speed;
                n_trans = self.base().get_global_transform().interpolate_with(
                    vcam.get_global_transform(),
                    clampf(delta * (factor as f64), 0.0, 1.0) as f32,
                );
            }
            self.base_mut().set_global_transform(n_trans);
            self.last_cam = true;
        } else if self.last_cam {
            self.last_cam = false;
        }
    }
}

#[godot_api]
impl CameraBrain3D {
    #[func]
    fn push_cam(&mut self, vcam: Gd<VirtualCamera3D>) {
        self.cam_buffer.push(vcam);
    }

    #[func]
    fn pop_cam(&mut self, vcam: Gd<VirtualCamera3D>) {
        let mut index = None;
        for i in 0..self.cam_buffer.len() {
            if self.cam_buffer[i] == vcam {
                index = Some(i);
            }
        }
        if let Some(i) = index {
            self.cam_buffer.remove(i);
        }
    }
}

/// Virtual Cameras are fairly simple. The power comes from how they are moved.
///
/// For a dolly cam, you could just move the virtual camera along a Path3D based on proximity to the target(s).
#[derive(GodotClass)]
#[class(base=Marker3D)]
pub struct VirtualCamera3D {
    #[export]
    use_lerp: bool,

    #[export]
    lerp_speed: f32,

    #[export]
    push_on_ready: bool,

    base: Base<Marker3D>,
}

#[godot_api]
impl IMarker3D for VirtualCamera3D {
    fn init(base: Base<Marker3D>) -> Self {
        Self {
            use_lerp: true,
            lerp_speed: 1.0,
            push_on_ready: true,
            base,
        }
    }

    fn ready(&mut self) {
        if self.push_on_ready {
            self.push();
        }
    }
}
#[godot_api]
impl VirtualCamera3D {
    /// Pushes this virtual camera to the active CameraBrain3D's vcam stack
    #[func]
    fn push(&mut self) {
        if let Some(mut tree) = self.base().get_tree() {
            if let Some(brain_tree) = tree.get_first_node_in_group(CAMERA_BRAIN_GROUP.into()) {
                let option_temp: Result<Gd<CameraBrain3D>, Gd<Node>> = brain_tree.try_cast();
                if let Ok(mut brain) = option_temp {
                    brain.bind_mut().push_cam(self.to_gd());
                }
            }
        }
    }

    // Pops this vcam from the active CameraBrain3D's vcam stack
    #[func]
    fn pop(&mut self) {
        if let Some(mut tree) = self.base().get_tree() {
            if let Some(brain_tree) = tree.get_first_node_in_group(CAMERA_BRAIN_GROUP.into()) {
                let option_temp: Result<Gd<CameraBrain3D>, Gd<Node>> = brain_tree.try_cast();
                if let Ok(mut brain) = option_temp {
                    brain.bind_mut().pop_cam(self.to_gd());
                }
            }
        }
    }
}
