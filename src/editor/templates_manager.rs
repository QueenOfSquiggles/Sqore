use godot::{
    engine::{file_access::ModeFlags, FileAccess, ProjectSettings},
    prelude::*,
};

struct Template {
    name: String,
    node_type: String,
    text: String,
}

pub fn load_templates() {
    let mut templates: Vec<Template> = Vec::new();
    templates.push(Template {
        name: "test_template".into(),
        node_type: "Node".into(),
        text: r#"extends Node
			func _ready() -> void:
				print("Hey what's up fuckers!? Custom templates in the house!")
				
		"#
        .into(),
    });

    let mut dir = ProjectSettings::singleton()
        .get_setting("editor/script_templates_search_path".into())
        .stringify();
    if dir.is_empty() {
        dir = "res://script_templates/".into();
    }
    for t in templates.iter() {
        let mut file = FileAccess::open(
            (dir.to_string() + t.name.as_str() + ".gd").into(),
            ModeFlags::WRITE,
        );
        if let Some(file) = &mut file {
            file.store_string(t.text.clone().into());
        }
    }
}
