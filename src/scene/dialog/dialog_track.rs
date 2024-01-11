use std::fmt;

use godot::{
    engine::{file_access::ModeFlags, global::Error, FileAccess, Json},
    prelude::*,
};

#[derive(Debug, Clone)]
pub struct DialogTrack {
    pub lines: Vec<Line>,
}

impl DialogTrack {
    pub fn load_from_json(file_path: GString) -> Result<Self, DialogError> {
        let Some(file) = FileAccess::open(file_path.clone(), ModeFlags::READ) else {
            return Err(DialogError::CannotOpenFile {
                file: file_path.to_string(),
                reason: FileAccess::get_open_error(),
            });
        };
        let text = file.get_as_text();
        Self::load_from_text(text, file_path)
    }
    //    pub fn load_from_json(file_path: GString) -> Result<Self, DialogError> {

    pub fn load_from_text(text: GString, file_path: GString) -> Result<Self, DialogError> {
        let mut json = Json::new_gd();
        json.parse(text.clone());
        let err_msg = json.get_error_message();
        if !err_msg.is_empty() {
            return Err(DialogError::ExternalJsonParseError {
                file: file_path.to_string(),
                loaded_text: text.to_string(),
                convert_error: format!(
                    "JSON Parse error on line {} \n {}",
                    json.get_error_line(),
                    err_msg
                ),
            });
        }

        let var = json.get_data();
        let json_dict = Dictionary::try_from_variant(&var);
        drop(var);
        if let Err(error) = json_dict {
            return Err(DialogError::ExternalJsonParseError {
                file: file_path.to_string(),
                loaded_text: text.to_string(),
                convert_error: format!("{:?}", error),
            });
        }
        Self::load_from_dict(json_dict.unwrap(), file_path)
    }

    pub fn load_from_dict(dict: Dictionary, file_path: GString) -> Result<Self, DialogError> {
        if !dict.contains_key("nodes") {
            return Err(DialogError::InternalJsonParseError {
                file: file_path.to_string(),
                error_node: "__root__/nodes".to_string(),
                reason: "root node must contain a 'nodes' node".to_string(),
            });
        }
        let node_array_var = dict.get("nodes");
        if node_array_var.is_none() {
            return Err(DialogError::InternalJsonParseError {
                file: file_path.to_string(),
                error_node: "__root__/nodes".to_string(),
                reason: "value \"nodes\" not found in dict!".to_string(),
            });
        }
        let node_array: Result<Array<Variant>, _> =
            Array::try_from_variant(&node_array_var.unwrap());
        if let Err(error) = node_array {
            return Err(DialogError::InternalJsonParseError {
                file: file_path.to_string(),
                error_node: "__root__/nodes".to_string(),
                reason: format!("Failed to parse node array: {:?}", error),
            });
        }

        let array = node_array.unwrap();
        let mut zelf = Self { lines: Vec::new() };
        for (index, node_var) in array.iter_shared().enumerate() {
            let Ok(node) = Dictionary::try_from_variant(&node_var) else {
                godot_warn!("Failed to parse node as dictionary: {:?}", node_var);
                continue;
            };
            if !node.contains_key("type".to_variant()) {
                return Err(DialogError::InternalJsonParseError {
                    file: file_path.to_string(),
                    error_node: Json::stringify(node.to_variant()).to_string(),
                    reason: "Nodes require a 'type' entry to be properly parsed!".to_string(),
                });
            }
            let line_value: Line = match node
                .get_or_nil("type".to_variant())
                .to_string()
                .to_lowercase()
                .as_str()
            {
                "text" => Self::parse_text_line(&node),
                "choice" => Self::parse_choice_line(&node),
                "signal" => Self::parse_signal_line(&node),
                "action" => Self::parse_action_line(&node),
                _ => {
                    godot_warn!(
                        "Unexpected node type: '{}'",
                        node.get("type")
                            .unwrap_or("(Failed to find type entry)".to_variant())
                    );

                    Line::None
                }
            };

            if line_value == Line::None {
                return Err(DialogError::InternalJsonParseError {
                    file: file_path.to_string(),
                    error_node: format!("{:?}", node_var),
                    reason: format!("Failed to parse single node at index {}", index).to_string(),
                });
            }
            zelf.lines.push(line_value);
        }

        Ok(zelf)
    }
    fn parse_text_line(node_data: &Dictionary) -> Line {
        if !node_data.contains_key("content".to_variant()) {
            return Line::None;
        }
        Line::Text {
            text: node_data
                .get("content")
                .unwrap_or("".to_variant())
                .to_string(),
            character: node_data
                .get("character")
                .unwrap_or("".to_variant())
                .to_string(),
            requires: node_data
                .get("requires")
                .unwrap_or("".to_variant())
                .to_string(),
        }
    }
    fn parse_signal_line(node_data: &Dictionary) -> Line {
        if !node_data
            .contains_all_keys(Array::from_iter(["name".to_variant(), "args".to_variant()]))
        {
            return Line::None;
        }
        let argsvar = node_data.get_or_nil("args");
        let params: Array<Variant> = Array::try_from_variant(&argsvar).unwrap_or(Array::new());
        let args = params
            .iter_shared()
            .map(|val| Json::stringify(val).to_string())
            .collect();

        Line::Signal {
            name: node_data
                .get("name".to_variant())
                .unwrap_or("default".to_variant())
                .to_string(),
            args,
        }
    }

    fn parse_action_line(node: &Dictionary) -> Line {
        if !node.contains_key("code") {
            return Line::None;
        }
        Line::Action {
            action: node.get("code").unwrap_or("".to_variant()).to_string(),
        }
    }

    fn parse_choice_line(node: &Dictionary) -> Line {
        if !node.contains_key("options") {
            return Line::None;
        }
        let vararr = node.get_or_nil("options");
        let arr: Array<Variant> = Array::try_from_variant(&vararr).unwrap_or_default();
        let mut choice_buffer = Vec::new();
        for (index, var) in arr.iter_shared().enumerate() {
            let Ok(dict) = Dictionary::try_from_variant(&var) else {
                godot_warn!("Failed to parse option entry: {:?}", var);
                continue;
            };
            let opt_entry = ChoiceOptionEntry::try_load_from(&dict);
            let Some(entry) = opt_entry else {
                godot_error!(
                    "Failed to load choice option at index {}. Data found: {}",
                    index,
                    Json::stringify(dict.to_variant()).to_string()
                );
                return Line::None; // no "teehee :3 I malformed my data" mfers allowed >:3
            };
            choice_buffer.push(entry);
        }
        Line::Choice {
            prompt: node
                .get("prompt".to_variant())
                .unwrap_or("".to_variant())
                .to_string(),
            character: node
                .get("character".to_variant())
                .unwrap_or("".to_variant())
                .to_string(),
            options: choice_buffer,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Line {
    Text {
        text: String,
        character: String,
        requires: String,
    },
    Choice {
        prompt: String,
        character: String,
        options: Vec<ChoiceOptionEntry>,
    },
    Action {
        action: String,
    },
    Signal {
        name: String,
        args: Vec<String>,
    },
    None,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct ChoiceOptionEntry {
    pub text: String,
    pub requires: String,
    pub action: String,
}

#[derive(Clone)]
pub enum DialogError {
    CannotOpenFile {
        file: String,
        reason: Error,
    },
    ExternalJsonParseError {
        file: String,
        loaded_text: String,
        convert_error: String,
    },
    InternalJsonParseError {
        file: String,
        error_node: String,
        reason: String,
    },
    Unexpected,
}

impl fmt::Debug for DialogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CannotOpenFile { file, reason } => f
                .debug_struct("CannotOpenFile")
                .field("file", file)
                .field("reason", reason)
                .finish(),
            Self::ExternalJsonParseError {
                file,
                loaded_text,
                convert_error,
            } => f
                .debug_struct("ExternalJsonParseError")
                .field("file", file)
                .field("loaded_text", loaded_text)
                .field("convert_rror", convert_error)
                .finish(),
            Self::InternalJsonParseError {
                file,
                error_node,
                reason,
            } => f
                .debug_struct("InternalJsonParseError")
                .field("file", file)
                .field("error_node", &Self::format_dict_string(error_node))
                .field("reason", reason)
                .finish(),
            Self::Unexpected => write!(f, "Unexpected"),
        }
    }
}

impl DialogError {
    fn format_dict_string(dict_str: &String) -> String {
        let dict = Dictionary::try_from_variant(&Json::parse_string(dict_str.to_godot()));
        if dict.is_err() {
            return dict_str.clone();
        }
        Json::stringify_ex(dict.unwrap().to_variant())
            .indent("\t".to_godot())
            .sort_keys(false)
            .full_precision(false)
            .done()
            .to_string()
    }
}

impl ChoiceOptionEntry {
    fn try_load_from(dict: &Dictionary) -> Option<Self> {
        if !dict.contains_all_keys(Array::from_iter([
            "text".to_variant(),
            "action".to_variant(),
        ])) {
            return None;
        }

        Some(Self {
            text: dict.get("text").unwrap().to_string(),
            requires: dict.get("requires").unwrap_or("".to_variant()).to_string(),
            action: dict.get("action").unwrap().to_string(),
        })
    }
}
