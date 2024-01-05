use self::dialog_manager::CoreDialog;
use godot::{engine::Engine, prelude::*};

mod dialog_manager;
mod dialog_track;
mod dialog_gui;

pub fn register_singleton() {
    Engine::singleton().register_singleton(
        StringName::from(CoreDialog::SINGLETON_NAME),
        CoreDialog::alloc_gd().upcast(),
    );
}

pub fn unregister_singleton() {
    Engine::singleton().unregister_singleton(StringName::from(CoreDialog::SINGLETON_NAME));
}
