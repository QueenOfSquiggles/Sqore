use godot::engine::{
    EditorCommandPalette, EditorPlugin, Engine, IEditorPlugin, Os, PopupMenu, ProjectSettings,
};
use godot::prelude::*;

use crate::scene::game_globals::Sqore;
use crate::scene::serialization::SqoreSerialized;

use super::templates_manager;

#[derive(GodotClass)]
#[class(tool, editor_plugin, init, base=EditorPlugin)]
struct SqoreEditorUtils {
    tool_items: Option<Gd<PopupMenu>>,
    base: Base<EditorPlugin>,
    menu_callbacks: Vec<Callable>,
}

#[godot_api]
impl IEditorPlugin for SqoreEditorUtils {
    fn enter_tree(&mut self) {
        if Engine::singleton().is_editor_hint() {
            templates_manager::load_templates();
        }

        let mut menu = PopupMenu::new_alloc();
        menu.connect(
            StringName::from("id_pressed"),
            Callable::from_object_method(&self.to_gd(), "on_menu_item"),
        );
        self.base_mut()
            .add_tool_submenu_item("Sqore".to_godot(), menu.clone());
        self.tool_items = Some(menu);
        let Some(editor) = self.base_mut().get_editor_interface() else {
            return;
        };
        let Some(mut cmd) = editor.get_command_palette() else {
            return;
        };
        self.register_tool_item(
            "force_globals_serialize",
			"Forces the core globals to serialize their data to the user dir. Clears the missing file warnings.",
            Callable::from_fn("force_globals_serialize", |_args: &[&Variant]| {
                Sqore::singleton()
                    .bind()
                    .get_config()
                    .bind_mut()
                    .serialize();
                Ok(Variant::nil())
            }),
            &mut cmd,
        );
        self.register_tool_item(
            "open_sqore_docs",
            "Opens the documentation for Sqore in your default browser",
            Callable::from_fn("open_sqore_docs", |_| {
                let global_path =
                    ProjectSettings::singleton().globalize_path(Self::DOC_ENTRY_INDEX.to_godot());
                Os::singleton().shell_open(global_path);
                Ok(Variant::nil())
            }),
            &mut cmd,
        );
        self.register_tool_item(
            "rebuild_sqore_templates",
            "Writes out template files for GD scripts.",
            Callable::from_fn("open_sqore_docs", |_| {
                templates_manager::load_templates();

                Ok(Variant::nil())
            }),
            &mut cmd,
        );
    }

    fn exit_tree(&mut self) {}
}
#[godot_api]
impl SqoreEditorUtils {
    const DOC_ENTRY_INDEX: &'static str = "res://addons/sqore/doc/sqore/index.html";

    fn humanize_text(input: &str) -> String {
        let (f, b) = input.split_at(1);
        let front: String = f.to_uppercase();
        let back: String = b.replace('_', " ");
        front + back.as_str()
    }

    /// registers a callable command in both the tools dropdown pane of the editor and the command palette for quick access
    fn register_tool_item(
        &mut self,
        name: &str,
        description: &str,
        func: Callable,
        command_palette: &mut Gd<EditorCommandPalette>,
    ) {
        let fname = ("sqore_core/".to_string() + name).to_godot();
        let hname = Self::humanize_text(name);
        if let Some(mut menu_items) = self.tool_items.clone() {
            menu_items.add_item(name.to_godot());
            let index = menu_items.get_item_count() - 1;
            menu_items.set_item_text(index, hname.clone().into());
            menu_items.set_item_tooltip(index, description.to_godot());
            self.menu_callbacks.push(func.clone());
        } else {
            self.base_mut()
                .add_tool_menu_item(fname.clone(), func.clone());
        }
        command_palette.add_command(hname.to_godot(), fname, func);
    }

    #[func]
    fn on_menu_item(&mut self, index: u32) {
        if let Some(callable) = self.menu_callbacks.get(index as usize) {
            callable.callv(Array::new());
        }
    }
}
