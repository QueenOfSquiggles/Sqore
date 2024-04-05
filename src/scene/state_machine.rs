//! # State Machines
//! For Godot 4.2 earlier, you have to manually remember the functions "tick(delta:float)->void", "on_enter()->void", and "on_exit()->void" as those are the string name values that are checked for in the states.
//! > From Godot 4.3+, support for virtual functions will have been reached which will allow a refactor that makes extending the base class functions significantly easier.
//!
//! For now just remember to keep your function names in order and everything should be fine
use godot::prelude::*;

const METHOD_TICK: &str = "tick";
const METHOD_ON_ENTER: &str = "on_enter";
const METHOD_ON_EXIT: &str = "on_exit";

#[derive(Var, PartialEq, Eq, Debug, Default, Export, Clone, GodotConvert)]
#[godot(via = i64)]
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
        let tick_mode = self.tick_mode.clone();
        self.base_mut().set_process(tick_mode == TickMode::Process);
        self.base_mut()
            .set_physics_process(tick_mode == TickMode::PhysicsProcess);
        if let Some(mut curr) = self.current.clone() {
            curr.call(StringName::from(METHOD_ON_ENTER), &[]);
        }
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
            state.call_deferred(StringName::from(METHOD_TICK), &[delta.to_variant()]);
        }
    }

    #[func]
    fn push_state(&mut self, n_state: Option<Gd<Node>>) {
        if let Some(mut prev) = self.current.clone() {
            prev.call(StringName::from(METHOD_ON_EXIT), &[]);
        }
        self.current = n_state;
        if let Some(mut now) = self.current.clone() {
            now.call(StringName::from(METHOD_ON_ENTER), &[]);
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
    fn on_enter(&mut self) {
        if let Some(mut curr) = self.current.clone() {
            curr.call(StringName::from(METHOD_ON_ENTER), &[]);
        }
    }

    #[func]
    fn tick(&mut self, delta: f64) {
        if let Some(mut state) = self.current.clone() {
            state.call(StringName::from(METHOD_TICK), &[delta.to_variant()]);
        }
    }

    #[func]
    fn push_state(&mut self, n_state: Option<Gd<Node>>) {
        if let Some(mut prev) = self.current.clone() {
            prev.call(StringName::from(METHOD_ON_EXIT), &[]);
        }
        self.current = n_state;
        if let Some(mut now) = self.current.clone() {
            now.call(StringName::from(METHOD_ON_ENTER), &[]);
        }
    }
}
