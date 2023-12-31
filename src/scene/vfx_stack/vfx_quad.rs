use godot::{
    engine::{IMeshInstance3D, MeshInstance3D, QuadMesh, ShaderMaterial},
    prelude::*,
};

use crate::scene::game_globals::CoreGlobals;

use super::vfx_stack_resource::VFXStack;

#[derive(GodotClass)]
#[class(base=MeshInstance3D)]
pub struct VFXQuad {
    vfx: Gd<ShaderMaterial>,
    #[base]
    base: Base<MeshInstance3D>,
}
#[godot_api]
impl VFXQuad {
    pub const CALLABLE_REFRESH_VFX_STACK: &'static str = "refresh_vfx_stack";
    #[func]
    fn refresh_vfx_stack(&mut self) {
        let Some(binding) = CoreGlobals::singleton()
            .bind()
            .get_config()
            .bind()
            .get_vfx_stack()
        else {
            return;
        };
        let sbind = binding.bind();
        let mut last_ref = self.vfx.clone();
        for layer in sbind.get_layers().iter_shared() {
            let lbind = layer.bind();
            if !lbind.get_enabled() {
                continue;
            }
            if let Some(layer) = lbind.get_material_data() {
                last_ref.set_next_pass(layer.clone().upcast());
                last_ref = layer;
            }
        }
    }
}
#[godot_api]
impl IMeshInstance3D for VFXQuad {
    fn init(base: Base<MeshInstance3D>) -> Self {
        Self {
            base,
            vfx: ShaderMaterial::new(),
        }
    }

    fn ready(&mut self) {
        self.base.set_mesh(QuadMesh::new().upcast());
        let Some(mut vfx) = CoreGlobals::singleton()
            .bind()
            .get_config()
            .bind()
            .get_vfx_stack()
        else {
            return;
        };
        vfx.connect(
            StringName::from(VFXStack::SIGNAL_VFX_STACK_CHANGED),
            Callable::from_object_method(&self.base, Self::CALLABLE_REFRESH_VFX_STACK),
        );
        self.base.set_material_override(self.vfx.clone().upcast());
        self.refresh_vfx_stack();
    }
}
