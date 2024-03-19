use godot::{engine::ShaderMaterial, prelude::*};

use crate::scene::game_globals::SqoreGlobals;

#[derive(GodotClass)]
#[class(init, base=Resource)]
pub struct VFXStack {
    #[export]
    layers: Array<Gd<VFXStackLayer>>,

    node: Base<Resource>,
}

#[godot_api]
impl VFXStack {
    #[func]
    fn set_layer_enabled_label(&mut self, label: GString, is_enabled: bool) {
        if let Some(mut layer) = self.layers.iter_shared().find(|p| p.bind().label == label) {
            layer.bind_mut().enabled = is_enabled;
        }
        SqoreGlobals::singleton().emit_signal(
            StringName::from(SqoreGlobals::SIGNAL_VFX_STACK_CHANGED),
            &[],
        );
    }
    #[func]
    fn set_layer_enabled(&mut self, index: i32, is_enabled: bool) {
        if let Some(mut layer) = self.layers.try_get(usize::try_from(index).unwrap_or(0)) {
            layer.bind_mut().enabled = is_enabled;
        }
        SqoreGlobals::singleton().emit_signal(
            StringName::from(SqoreGlobals::SIGNAL_VFX_STACK_CHANGED),
            &[],
        );
    }

    #[func]
    fn get_layer_enabled(&self, index: i32) -> bool {
        let Some(layer) = self.layers.try_get(usize::try_from(index).unwrap_or(0)) else {
            return false;
        };
        let val = layer.bind().enabled;
        val
    }
    #[func]
    fn get_layer_enabled_label(&self, label: GString) -> bool {
        let Some(layer) = self.layers.iter_shared().find(|p| p.bind().label == label) else {
            return false;
        };
        let val = layer.bind().enabled;
        val
    }

    #[func]
    fn set_all(&mut self, is_enabled: bool) {
        for mut entry in self.layers.iter_shared() {
            entry.bind_mut().enabled = is_enabled;
        }
        SqoreGlobals::singleton().emit_signal(
            StringName::from(SqoreGlobals::SIGNAL_VFX_STACK_CHANGED),
            &[],
        );
    }
}

#[derive(GodotClass)]
#[class(base=Resource)]
pub struct VFXStackLayer {
    #[export]
    material_data: Option<Gd<ShaderMaterial>>,
    #[export]
    label: GString,
    #[export]
    enabled: bool,

    node: Base<Resource>,
}
#[godot_api]
impl IResource for VFXStackLayer {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            material_data: None,
            label: "Unnamed VFX Layer".to_godot(),
            enabled: true,
            node: base,
        }
    }
}

#[godot_api]
impl VFXStackLayer {}
