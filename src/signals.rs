use godot::{
    builtin::{StringName, Variant},
    engine::{global::Error, Object},
    obj::Gd,
};

pub fn emit(node: &mut Gd<Object>, signal_name: &str, params: &[Variant]) -> Error {
    node.emit_signal(StringName::from(signal_name), params)
}
