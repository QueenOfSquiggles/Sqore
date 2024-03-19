use godot::{
    engine::{InputEvent, InputEventMouseMotion, InputEventPanGesture},
    prelude::*,
};

enum AxisInput {
    MouseMotion,
    FourActionAxis {
        left: StringName,
        right: StringName,
        down: StringName,
        up: StringName,
    },
    /// note: this is a two-finger pan
    TouchGesturePan,
}

impl AxisInput {
    fn get_process_value(&self, delta: f32) -> Vector2 {
        match self {
            AxisInput::MouseMotion => Vector2::ZERO,
            AxisInput::TouchGesturePan => Vector2::ZERO,
            // pull
            AxisInput::FourActionAxis {
                left,
                right,
                down,
                up,
            } => {
                Input::singleton().get_vector(left.clone(), right.clone(), down.clone(), up.clone())
                    * delta
            }
        }
    }
    fn get_value_for(&self, event: &Gd<InputEvent>) -> Vector2 {
        match self {
            AxisInput::MouseMotion => {
                let mcast: Result<Gd<InputEventMouseMotion>, _> = event.clone().try_cast();
                if let Ok(mouse) = mcast {
                    mouse.get_relative()
                } else {
                    Default::default()
                }
            }
            AxisInput::TouchGesturePan => {
                let tcast: Result<Gd<InputEventPanGesture>, _> = event.clone().try_cast();
                if let Ok(pad) = tcast {
                    pad.get_delta()
                } else {
                    Default::default()
                }
            }
            #[allow(unused_variables)]
            AxisInput::FourActionAxis {
                left,
                right,
                down,
                up,
            } => Vector2::ZERO,
        }
    }
}

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct InputAxisAllocatorResource {
    #[export(enum=(MouseMotion=0, FourActionAxis=1, TouchGesturePan=2))]
    input_type: u32,
    #[export]
    factor: f32,
    #[export]
    event_left: StringName,
    #[export]
    event_right: StringName,
    #[export]
    event_up: StringName,
    #[export]
    event_down: StringName,

    node: Base<Resource>,
}

#[godot_api]
impl IResource for InputAxisAllocatorResource {}
#[godot_api]
impl InputAxisAllocatorResource {}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct InputAxisAllocator {
    #[export]
    events: Array<Gd<InputAxisAllocatorResource>>,
    events_internal: Vec<(AxisInput, f32)>,

    value: Vector2,

    node: Base<Node>,
}

#[godot_api]
impl INode for InputAxisAllocator {
    fn ready(&mut self) {
        for eve in self.events.iter_shared() {
            let b = eve.bind();
            let axis = match b.input_type {
                0 => AxisInput::MouseMotion,
                1 => AxisInput::FourActionAxis {
                    left: b.event_left.clone(),
                    right: b.event_right.clone(),
                    down: b.event_down.clone(),
                    up: b.event_up.clone(),
                },
                2 => AxisInput::TouchGesturePan,
                _ => unreachable!(),
            };
            self.events_internal.push((axis, b.factor));
        }
    }

    fn process(&mut self, delta: f64) {
        for eve in self.events_internal.iter() {
            self.value += eve.0.get_process_value(delta as f32) * eve.1;
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        for eve in self.events_internal.iter() {
            self.value += eve.0.get_value_for(&event) * eve.1;
        }
    }
}

#[godot_api]
impl InputAxisAllocator {
    #[func]
    fn get_value(&mut self) -> Vector2 {
        let ret = self.value;
        self.value = Vector2::ZERO;
        ret
    }
}
