use godot::engine::{EditorCommandPalette, EditorPlugin, IEditorPlugin, PopupMenu};
use godot::prelude::*;

use crate::scene::game_globals::CoreGlobals;
use crate::scene::serialization::SquigglesSerialized;

#[derive(GodotClass)]
#[class(tool, editor_plugin, init, base=EditorPlugin)]
struct SquigglesCoreEditorUtils {
    tool_items: Option<Gd<PopupMenu>>,
    #[base]
    base: Base<EditorPlugin>,
}

#[godot_api]
impl IEditorPlugin for SquigglesCoreEditorUtils {
    fn enter_tree(&mut self) {
        let menu = PopupMenu::new_alloc();
        self.base
            .add_tool_submenu_item("Squiggles Core".to_godot(), menu.clone());
        self.tool_items = Some(menu);
        let Some(editor) = self.base.get_editor_interface() else {
            return;
        };
        let Some(mut cmd) = editor.get_command_palette() else {
            return;
        };
        self.register_tool_item(
            "force_globals_serialize",
			"Forces the core globals to serialize their data to the user dir. Clears the missing file warnings.",
            Callable::from_fn("name", |_args: &[&Variant]| {
                CoreGlobals::singleton()
                    .bind()
                    .get_config()
                    .bind_mut()
                    .serialize();
                Ok(Variant::nil())
            }),
            &mut cmd,
        );
    }

    fn exit_tree(&mut self) {}
}

impl SquigglesCoreEditorUtils {
    /// registers a callable command in both the tools dropdown pane of the editor and the command palette for quick access
    fn register_tool_item(
        &mut self,
        name: &str,
        description: &str,
        func: Callable,
        command_palette: &mut Gd<EditorCommandPalette>,
    ) {
        let fname = ("squiggles_core/".to_string() + name).to_godot();
        if let Some(mut menu_items) = self.tool_items.clone() {
            menu_items.add_item(name.to_godot());
            let index = menu_items.get_item_count() - 1;
            menu_items.set_item_tooltip(index, description.to_godot())
        }
        self.base.add_tool_menu_item(fname.clone(), func.clone());
        command_palette.add_command(name.to_godot(), fname, func);
    }
}
