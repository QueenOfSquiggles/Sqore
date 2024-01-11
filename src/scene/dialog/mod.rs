use self::core_dialog::CoreDialog;
use godot::{engine::Engine, prelude::*};

pub mod core_dialog;
pub mod dialog_blackboard;
pub mod dialog_builder;
pub mod dialog_events;
pub mod dialog_gui;
pub mod dialog_settings;
pub mod dialog_track;

pub fn register_singleton() {
    Engine::singleton().register_singleton(
        StringName::from(CoreDialog::SINGLETON_NAME),
        CoreDialog::new_alloc().upcast(),
    );
}

pub fn unregister_singleton() {
    Engine::singleton().unregister_singleton(StringName::from(CoreDialog::SINGLETON_NAME));
}
