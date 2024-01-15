use std::path::Path;

use godot::{
    engine::{DirAccess, Engine, ProjectSettings},
    prelude::*,
};
#[repr(i32)]
#[derive(Debug, Var, Export, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModLoadingType {
    #[default]
    Disabled = 0,
    AutoLoadAll = 1,
    ManualSelection = 2,
}

#[derive(GodotClass)]
#[class(tool, init,base=Resource)]
pub struct UserModifications {
    #[export]
    mods_directory: GString,
    #[export]
    allow_mods_override_files: bool,
    #[export]
    loading_type: ModLoadingType,

    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl UserModifications {
    const MOD_FILE_EXTENSION: &'static str = ".pck";
    pub fn find_mods(&self) -> Array<GString> {
        let Some(dir) = &mut DirAccess::open(self.mods_directory.clone()) else {
            godot_error!(
                "Failed to find mods in directory '{}' error message: {:?}",
                self.mods_directory,
                DirAccess::get_open_error()
            );
            return Array::new();
        };
        let all_files = dir.get_files();
        let mod_files = all_files
            .as_slice()
            .iter()
            .filter(|path| {
                path.to_string()
                    .to_lowercase()
                    .ends_with(Self::MOD_FILE_EXTENSION)
            })
            .cloned();
        Array::from_iter(mod_files)
    }

    pub fn load_mod(&self, file_path: GString) -> bool {
        let succeeded = ProjectSettings::singleton()
            .load_resource_pack_ex(file_path.clone())
            .replace_files(self.allow_mods_override_files)
            .done();
        if succeeded {
            godot_print!("Loaded mod from file: {}", file_path);
        } else {
            godot_warn!("Failed to load mod from file: {}", file_path);
        }
        succeeded
    }

    pub fn handle_startup(&self) {
        if Engine::singleton().is_editor_hint() {
            return;
        }
        if self.loading_type == ModLoadingType::AutoLoadAll {
            let files = self.find_mods();
            for f in files.iter_shared() {
                let path =
                    Path::new(&self.mods_directory.to_string()).join(Path::new(&f.to_string()));
                self.load_mod(path.to_str().unwrap_or_default().to_godot());
            }
        }
    }
}
