// use godot::{
//     engine::{ArrayMesh, IMeshInstance3D, Mesh, MeshDataTool, MeshInstance3D, Path3D},
//     prelude::*,
// };

// #[derive(GodotClass)]
// #[class(init, base=MeshInstance3D)]
// pub struct SplineMesh {
//     #[export]
//     spline: Option<Gd<Path3D>>,
//     #[export]
//     target_mesh: Option<Gd<Mesh>>,
//     #[base]
//     base: Base<MeshInstance3D>,
// }

// #[godot_api]
// impl IMeshInstance3D for SplineMesh {}

// #[godot_api]
// impl SplineMesh {
//     #[func]
//     pub fn generate_mesh(&mut self) {
//         self.ensure_array_mesh();
//         let Some(generic_mesh) = self.base().get_mesh() else {
//             return;
//         };
//         let array_mesh_cast: Result<Gd<ArrayMesh>, _> = generic_mesh.try_cast();
//         let Ok(mut array_mesh) = array_mesh_cast else {
//             return;
//         };
//         let Some(spline) = self.spline.clone() else {
//             return;
//         };
//         let Some(spline) = spline.get_curve() else {
//             return;
//         };
//         let Some(target) = self.target_mesh.clone() else {
//             return;
//         };
//         array_mesh.clear_surfaces();

//         let interval = spline.get_bake_interval();
//         let length = spline.get_baked_length();
//         let points = spline.get_baked_points();
//         let tilts = spline.get_baked_tilts();
//         let ups = spline.get_baked_up_vectors();
//         let data = MeshDataTool::new_gd();
//         for (index, value) in points.as_slice().iter().enumerate() {
//             let tilt = tilts.as_slice()[index];
//             let up = ups.as_slice()[index];
//         }
//     }

//     fn ensure_array_mesh(&mut self) {
//         if self.base().get_mesh().is_none() {
//             self.base().set_mesh(ArrayMesh::new_gd().upcast());
//             return;
//         }
//         let arr_cast: Result<Gd<ArrayMesh>, _> = self.base().get_mesh().unwrap().try_cast();
//         let Ok(_) = arr_cast else {
//             self.base().set_mesh(ArrayMesh::new_gd().upcast());
//             return;
//         };
//     }
// }
