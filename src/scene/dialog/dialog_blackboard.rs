use core::fmt;
use std::{
    collections::{hash_map::DefaultHasher, HashMap, VecDeque},
    fmt::Display,
    hash::{Hash, Hasher},
    rc::Rc,
};

use godot::{
    engine::{global::Error, Json},
    prelude::*,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Entry {
    Number(f32),
    String(String),
    Bool(bool),
    None,
}
impl Default for Entry {
    fn default() -> Self {
        Self::None
    }
}
impl Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Entry::Number(val) => f.write_fmt(format_args!("{:.2}", val)),
            Entry::String(val) => f.write_fmt(format_args!("{}", val)),
            Entry::Bool(val) => f.write_fmt(format_args!("{}", val)),
            Entry::None => f.write_str("nil"),
        }
    }
}

pub struct Blackboard {
    entries: HashMap<String, Entry>,
    commands: Vec<Command>,
}

impl Default for Blackboard {
    fn default() -> Self {
        let mut zelf = Self {
            entries: HashMap::new(),
            commands: Vec::new(),
        };
        zelf.commands.push(Command {
            name: "set".into(),
            args: 2,
            callback: Rc::new(|bb, args| bb.set(args[0].as_str(), args[1].as_str())),
        });
        zelf.commands.push(Command {
            name: "add".into(),
            args: 2,
            callback: Rc::new(|bb, args| bb.add(args[0].as_str(), args[1].as_str())),
        });
        zelf.commands.push(Command {
            name: "sub".into(),
            args: 2,
            callback: Rc::new(|bb, args| bb.sub(args[0].as_str(), args[1].as_str())),
        });
        zelf.commands.push(Command {
            name: "jump".into(),
            args: 1,
            callback: Rc::new(|bb, args| bb.jump(args[0].as_str())),
        });
        zelf.commands.push(Command {
            name: "end".into(),
            args: 0,
            callback: Rc::new(|bb, _| bb.end()),
        });
        zelf
    }
}

impl Blackboard {
    /// Parses the action string
    pub fn parse_action(&mut self, code: String) {
        // godot_print!("Running action(s): {}", code);
        for action in code.split(';') {
            // godot_print!("Running sub-action: {}", action);
            let mut parts = VecDeque::from_iter(action.trim().split(' ').map(|dirty| dirty.trim()));
            let command = parts.pop_front().unwrap_or("");
            let mut callback: Option<_> = None;
            for cmd in self.commands.clone().iter() {
                if cmd.name == command {
                    if parts.len() == cmd.args {
                        callback = Some(cmd.callback.clone());
                        break;
                    } else {
                        godot_warn!(
                            "Command \"{}\" requires \"{}\" arguments. Found {}. Code: {}",
                            command,
                            cmd.args,
                            parts.len(),
                            action
                        );
                    }
                }
            }

            if let Some(callable) = callback {
                let parts = parts.iter().map(|v| v.to_string()).collect();
                (callable)(self, parts);
            } else {
                godot_warn!(
                    "Unrecognized command! \"{}\" in line \"{}\"",
                    command,
                    action
                );
            }
        }
    }

    pub fn parse_query(&mut self, code: String) -> bool {
        // godot_print!("Running quer(y/ies): {}", code);
        for query in code.split("and") {
            let mut chunk_val = false;
            for options in query.split("or") {
                // godot_print!("Running sub-query: {}", options);

                let parts = Vec::from_iter(options.split_whitespace());
                if parts.len() != 3 {
                    if query.contains('\"') {
                        // TODO if someone want's to make this support space strings, go right ahead, I'll accept the PR. But I'm not writing it myself lol
                        godot_error!("Strings with spaces are not supported for queries! Only used for storage!")
                    }
                    godot_warn!("Malformed query {}, in code {}", options, code);
                    continue;
                }
                chunk_val = chunk_val || self.parse_query_value((parts[0], parts[1], parts[2]));
            }
            if !chunk_val {
                return false;
            }
        }
        true
    }

    fn parse_query_value(&mut self, query: (&str, &str, &str)) -> bool {
        let arg1 = self.get_numeric_value(query.0);
        let arg2 = self.get_numeric_value(query.2);
        // godot_print!("Running internal comparison: {} {} {}", arg1, query.1, arg2);
        match query.1 {
            "==" => arg1 == arg2,
            "!=" => arg1 != arg2,
            ">=" => arg1 >= arg2,
            "<=" => arg1 <= arg2,
            ">" => arg1 > arg2,
            "<" => arg1 < arg2,
            _ => {
                godot_warn!(
                    "Unrecognized operator in query: {} {} {}",
                    query.0,
                    query.1,
                    query.2
                );
                false
            }
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let entry = Self::get_entry_for(value);
        if entry == Entry::None {
            godot_warn!("Failed to find valid entry for setting to \"{}\"", key);
            return;
        };
        self.entries.insert(key.to_string(), entry.clone());
        // godot_print!("Set value: {} = {}. Enum value: {}", key, value, entry);
    }

    pub fn unset(&mut self, key: &str) {
        let _ = self.entries.remove(&key.to_string());
    }

    pub fn add(&mut self, key: &str, value: &str) {
        if !self.entries.contains_key(&key.to_string()) {
            godot_warn!("Cannot add to \"\"! Does not exist yet!");
            return;
        }
        let entry = Self::get_entry_for(value);
        if entry == Entry::None {
            godot_warn!("Failed to find valid entry for setting to \"{}\"", key);
            return;
        };
        let Some(prev) = self.entries.get(&key.to_string()) else {
            unreachable!()
        };
        let nval = match prev {
            Entry::Number(val) => match entry {
                Entry::Number(entry_val) => Entry::Number(*val + entry_val),
                Entry::String(_) => Entry::None,
                Entry::Bool(entry_val) => Entry::Number(
                    *val + match entry_val {
                        true => 1f32,
                        false => 0f32,
                    },
                ),
                Entry::None => Entry::Number(*val),
            },
            Entry::String(val) => match entry {
                Entry::Number(entry_val) => {
                    Entry::String(val.clone() + entry_val.to_string().as_str())
                }
                Entry::String(entry_val) => Entry::String(val.clone() + entry_val.as_str()),
                Entry::Bool(entry_val) => {
                    Entry::String(val.clone() + entry_val.to_string().as_str())
                }
                Entry::None => Entry::String(val.clone()),
            },
            _ => Entry::None,
        };
        self.entries.insert(key.to_string(), nval);
    }
    pub fn sub(&mut self, key: &str, value: &str) {
        if !self.entries.contains_key(&key.to_string()) {
            godot_warn!("Cannot add to \"\"! Does not exist yet!");
            return;
        }
        let entry = Self::get_entry_for(value);
        if entry == Entry::None {
            godot_warn!("Failed to find valid entry for setting to \"{}\"", key);
            return;
        };
        let Some(prev) = self.entries.get(&key.to_string()) else {
            unreachable!()
        };
        let nval = match prev {
            Entry::Number(val) => match entry {
                Entry::Number(entry_val) => Entry::Number(*val - entry_val),
                Entry::String(_) => Entry::None,
                Entry::Bool(entry_val) => Entry::Number(
                    *val - match entry_val {
                        true => 1f32,
                        false => 0f32,
                    },
                ),
                Entry::None => Entry::Number(*val),
            },
            _ => Entry::None,
        };
        self.entries.insert(key.to_string(), nval);
    }

    pub const EVENT_KEY: &'static str = "__event__";
    pub const EVENT_ARG_KEY: &'static str = "__event_arg__";
    fn set_event(&mut self, event_name: &str, arg: Option<&str>) {
        self.entries.insert(
            Self::EVENT_KEY.to_string(),
            Entry::String(event_name.to_string()),
        );
        if let Some(arg) = arg {
            self.entries
                .insert(Self::EVENT_ARG_KEY.to_string(), Self::get_entry_for(arg));
        }
    }

    pub fn jump(&mut self, target: &str) {
        self.set_event("jump", Some(target));
    }

    pub fn end(&mut self) {
        self.set_event("end", None);
    }

    pub fn get_event(&self) -> Option<(String, Entry)> {
        if let Some(Entry::String(name)) = self.get(Self::EVENT_KEY) {
            let arg = self.get(Self::EVENT_ARG_KEY);
            return Some((name.clone(), arg.unwrap_or_default().clone()));
        }
        None
    }

    pub fn mark_event_handled(&mut self) {
        if self.entries.contains_key(&Self::EVENT_KEY.to_string()) {
            self.unset(Self::EVENT_KEY);
        }
        if self.entries.contains_key(&Self::EVENT_ARG_KEY.to_string()) {
            self.unset(Self::EVENT_ARG_KEY);
        }
    }

    fn get_entry_for(value: &str) -> Entry {
        let var = Json::parse_string(value.to_godot());
        match var.get_type() {
            VariantType::Nil => {
                godot_warn!("Failed to parse \"{}\" into a handled type!", value,);
                Entry::None
            }
            VariantType::Bool => Entry::Bool(var.booleanize()),
            VariantType::Int => Entry::Number(i32::from_variant(&var) as f32),
            VariantType::Float => Entry::Number(f32::from_variant(&var)),
            VariantType::String => Entry::String(String::from_variant(&var)),
            _ => {
                godot_warn!("Fail! \"{}\" is not a handled type!", value,);
                Entry::None
            }
        }
    }
    fn get_numeric_value(&self, key: &str) -> i32 {
        // load entry
        let entry: Entry = if self.entries.contains_key(&key.to_string()) {
            // from variable name
            self.entries.get(&key.to_string()).unwrap().clone() // we should be safe to unwrap here???
        } else {
            // from constant
            let mut json = Json::new_gd();
            let err = json.parse(key.to_godot());
            if err != Error::OK {
                // for whatever reason we failed, return a none value
                Entry::None
            } else {
                let var = json.get_data();
                match var.get_type() {
                    VariantType::Float => Entry::Number(f32::from_variant(&var)),
                    VariantType::Int => Entry::Number(i32::from_variant(&var) as f32),
                    VariantType::Bool => Entry::Bool(bool::from_variant(&var)),
                    VariantType::String => Entry::String(String::from_variant(&var)),
                    _ => Entry::None,
                }
            }
        };
        match entry {
            Entry::Number(val) => f32::floor(val) as i32, // this does force integer accuracy only, but then again, this system is not designed for complex maths
            Entry::String(val) => {
                let mut s = DefaultHasher::new();
                val.hash(&mut s);
                s.finish() as i32
            }
            Entry::Bool(val) => match val {
                true => 1,
                false => 0,
            },
            Entry::None => i32::MIN,
        }
    }

    pub fn format_text(&self, text: String) -> String {
        let mut buffer = String::with_capacity(text.len());
        let mut remaining = text.as_str();
        const LOOP_LIMITER: u32 = 9999;
        const DELIM_OPEN: &str = "{{";
        const DELIM_CLOS: &str = "}}";
        for _ in 0..LOOP_LIMITER {
            if remaining.is_empty() {
                break;
            }
            let next = remaining.split_once(DELIM_OPEN);
            let Some(next) = next else {
                buffer += remaining;
                break;
            };
            let (pre, post) = next;
            buffer += pre;
            let Some(fin) = post.split_once(DELIM_CLOS) else {
                buffer += post;
                break;
            };
            let (key, after) = fin;
            let key = key.trim();
            let var = self.get_variant_entry(key.trim());
            if var.is_nil() {
                godot_warn!("Found null when parsing dialog key: {}", key);
            }
            buffer += var.to_string().as_str();
            remaining = after;
        }
        buffer
    }

    pub fn get_variant_entry(&self, key: &str) -> Variant {
        let Some(entry) = self.entries.get(key) else {
            godot_warn!("Entry not found \"{}\", returning nil", key);
            return Variant::nil();
        };
        match entry {
            Entry::Number(val) => val.to_variant(),
            Entry::String(val) => val.to_variant(),
            Entry::Bool(val) => val.to_variant(),
            Entry::None => Variant::nil(),
        }
    }

    pub fn get(&self, key: &str) -> Option<Entry> {
        Some(self.entries.get(key)?.clone())
    }

    pub fn debug_print(&self) {
        let mappings: Vec<String> = self
            .entries
            .iter()
            .map(|pair| format!("{} = {}, ", pair.0, pair.1))
            .collect();
        let mut buffer: String = "Blackboard { ".into();
        for e in mappings {
            buffer += e.as_str();
            buffer += "\n";
        }
        buffer += " }";
        // godot_print!("{}", buffer);
    }
}

impl fmt::Debug for Blackboard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_map().entries(self.entries.iter()).finish()
    }
}
type CommandFunction = Rc<dyn Fn(&mut Blackboard, VecDeque<String>)>;

#[derive(Clone)]
struct Command {
    name: String,
    args: usize,
    callback: CommandFunction,
}
