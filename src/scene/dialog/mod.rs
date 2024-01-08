use self::dialog_manager::CoreDialog;
use godot::{engine::Engine, prelude::*};

pub mod dialog_blackboard;
pub mod dialog_events;
pub mod dialog_gui;
pub mod dialog_manager;
pub mod dialog_track;

pub fn register_singleton() {
    Engine::singleton().register_singleton(
        StringName::from(CoreDialog::SINGLETON_NAME),
        CoreDialog::alloc_gd().upcast(),
    );
}

pub fn unregister_singleton() {
    Engine::singleton().unregister_singleton(StringName::from(CoreDialog::SINGLETON_NAME));
}
