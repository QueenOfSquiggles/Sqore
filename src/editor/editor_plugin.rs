use godot::engine::{
    EditorCommandPalette, EditorPlugin, IEditorPlugin, Os, PopupMenu, ProjectSettings,
};
use godot::prelude::*;

use crate::scene::game_globals::SqoreGlobals;
use crate::scene::serialization::SqoreSerialized;

#[derive(GodotClass)]
#[class(tool, editor_plugin, init, base=EditorPlugin)]
struct SqoreEditorUtils {
    tool_items: Option<Gd<PopupMenu>>,
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for SqoreEditorUtils {
    fn enter_tree(&mut self) {
        let menu = PopupMenu::new_alloc();
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
                SqoreGlobals::singleton()
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
        )
    }

    fn exit_tree(&mut self) {}
}

impl SqoreEditorUtils {
    const DOC_ENTRY_INDEX: &'static str = "res://addons/sqore/doc/sqore/index.html";

    /// registers a callable command in both the tools dropdown pane of the editor and the command palette for quick access
    fn register_tool_item(
        &mut self,
        name: &str,
        description: &str,
        func: Callable,
        command_palette: &mut Gd<EditorCommandPalette>,
    ) {
        let fname = ("sqore_core/".to_string() + name).to_godot();
        if let Some(mut menu_items) = self.tool_items.clone() {
            menu_items.add_item(name.to_godot());
            let index = menu_items.get_item_count() - 1;
            menu_items.set_item_tooltip(index, description.to_godot())
        }
        self.base_mut()
            .add_tool_menu_item(fname.clone(), func.clone());
        command_palette.add_command(name.to_godot(), fname, func);
    }
}
