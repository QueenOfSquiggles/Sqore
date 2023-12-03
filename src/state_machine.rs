use godot::prelude::*;
use once_cell::sync::Lazy;

static METHOD_TICK: Lazy<StringName> = Lazy::new(|| StringName::from("tick"));
static METHOD_ON_ENTER: Lazy<StringName> = Lazy::new(|| StringName::from("on_enter"));
static METHOD_ON_EXIT: Lazy<StringName> = Lazy::new(|| StringName::from("on_exit"));

#[repr(i32)]
#[derive(Property, PartialEq, Eq, Debug, Default, Export)]
enum TickMode {
    #[default]
    Process = 0,
    PhysicsProcess = 1,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct FiniteStateMachine {
    #[export]
    tick_mode: TickMode,
    #[export]
    current: Option<Gd<Node>>,
    #[base]
    base: Base<Node>,
}
#[derive(GodotClass)]
#[class(init, base=Node)]
struct FiniteState;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct FiniteSubStateMachine {
    // aggressively fighting the urge to call it "subspace"
    #[export]
    current: Option<Gd<Node>>,
}

#[godot_api]
impl INode for FiniteStateMachine {
    fn ready(&mut self) {
        self.base.set_process(self.tick_mode == TickMode::Process);
        self.base
            .set_physics_process(self.tick_mode == TickMode::PhysicsProcess);
    }
    fn process(&mut self, delta: f64) {
        self.do_tick(delta);
    }

    fn physics_process(&mut self, delta: f64) {
        self.do_tick(delta);
    }
}
#[godot_api]
impl FiniteStateMachine {
    #[func]
    fn do_tick(&mut self, delta: f64) {
        if let Some(mut state) = self.current.clone() {
            state.call_deferred(METHOD_TICK.clone(), &[delta.to_variant()]);
        }
    }

    #[func]
    fn push_state(&mut self, n_state: Option<Gd<Node>>) {
        if let Some(mut prev) = self.current.clone() {
            prev.call(METHOD_ON_EXIT.clone(), &[]);
        }
        self.current = n_state;
        if let Some(mut now) = self.current.clone() {
            now.call(METHOD_ON_ENTER.clone(), &[]);
        }
    }
}

#[godot_api]
impl FiniteState {
    #[signal]
    fn exit_state() {}

    // #[func]
    // fn on_exit(&mut self) {}
    // #[func]
    // fn on_enter(&mut self) {}
    // #[func]
    // fn tick(&mut self, _delta: f64) {}
}
#[godot_api]
impl FiniteSubStateMachine {
    #[func]
    fn on_exit(&mut self) {}

    #[func]
    fn on_enter(&mut self) {}

    #[func]
    fn tick(&mut self, delta: f64) {
        if let Some(mut state) = self.current.clone() {
            state.call(METHOD_TICK.clone(), &[delta.to_variant()]);
        }
    }

    #[func]
    fn push_state(&mut self, n_state: Option<Gd<Node>>) {
        if let Some(mut prev) = self.current.clone() {
            prev.call(METHOD_ON_EXIT.clone(), &[]);
        }
        self.current = n_state;
        if let Some(mut now) = self.current.clone() {
            now.call(METHOD_ON_ENTER.clone(), &[]);
        }
    }
}
