use std::collections::VecDeque;

use godot::{
    engine::{
        control::{LayoutPreset, SizeFlags},
        object::ConnectFlags,
        tween::{EaseType, TransitionType},
        Button, CanvasLayer, Control, HSeparator, ICanvasLayer, InputEvent, Label, MarginContainer,
        PanelContainer, RichTextLabel, Tween, VBoxContainer,
    },
    obj::EngineEnum,
    prelude::*,
};

use crate::{scene::game_globals::CoreGlobals, util::SquigglesUtil};

use super::{
    core_dialog::CoreDialog,
    dialog_events::DialogEvents,
    dialog_settings::{DialogAlign, DialogSettings, EEaseType, ETransType},
    dialog_track::{ChoiceOptionEntry, Line},
};

/// The current state of the Dialog GUI,
#[derive(Debug, Default, PartialEq)]
enum DialogState {
    /// Active meaning we are actively processing nodes and pushing data to be visible
    #[default]
    Active,
    /// Pending meaning we are currently waiting on an external process and don't want to create a degenerate reference to CoreDialog by forcing a push
    Pending,
}

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct DialogGUI {
    tween: Option<Gd<Tween>>,
    pub track: Option<VecDeque<Line>>,
    character_label: Option<Gd<Label>>,
    dialog_text: Option<Gd<RichTextLabel>>,
    options_root: Option<Gd<Control>>,
    current_index: usize,
    state: DialogState,

    base: Base<CanvasLayer>,
}

#[godot_api]
impl ICanvasLayer for DialogGUI {
    fn enter_tree(&mut self) {
        //pass
    }

    fn ready(&mut self) {
        self.create_structure();
        if let Some(line) = self.get_next_text_line() {
            self.load_line(&line);
        } else if self.state != DialogState::Pending {
            godot_warn!(
                "No text nodes found in dialog track on load. Something must have gone wrong?"
            );
        }
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.options_root.is_some() {
            // means there is a dialog choice being made
            return;
        }
        let settings = self.get_settings();
        if !event.is_action_pressed(StringName::from(settings.bind().interact_action.clone())) {
            return;
        }
        let mut progress_next_node_flag = true;
        if let Some(tween) = &mut self.tween {
            if tween.is_running() {
                // forces tween to finish (should usually only run once)
                while tween.custom_step(10f64) {}
                self.tween = None;
                progress_next_node_flag = false;
            }
        }
        if progress_next_node_flag {
            self.load_next_line();
        }
    }
    fn process(&mut self, _delta: f64) {
        if self.state != DialogState::Pending {
            return;
        }
        // downtime should only be for 2-3 frames MAX! so this aaggresive polling ***shouldn't*** have a big effect on the performance??
        self.load_next_line();
    }

    fn exit_tree(&mut self) {
        //pass
        if let Some(event_bus) = &mut CoreDialog::singleton().bind().get_event_bus() {
            event_bus.emit_signal(StringName::from(DialogEvents::SIGNAL_TRACK_ENDED), &[]);
        }
    }
}

#[godot_api]
impl DialogGUI {
    pub fn update_track(&mut self, n_track: VecDeque<Line>) {
        self.track = Some(n_track);
    }
    fn create_structure(&mut self) {
        /* INTENDED LAYOUT
        CanvasLayer (self.base)
        | MarginContainer
        | | PanelContainer
        | | | MarginContainer
        | | | | VBoxContainer
        | | | | | Label (self.character_label)
        | | | | | HSeperator
        | | | | | RichTextLabel (self.dialog_text)
        */
        // load settings
        let settings = self.get_settings(); // create instances.
        let mut margin = MarginContainer::new_alloc();
        let mut panel = PanelContainer::new_alloc();
        let mut panel_margin = MarginContainer::new_alloc();
        let mut vbox = VBoxContainer::new_alloc();
        let mut label = Label::new_alloc();
        let hsep = HSeparator::new_alloc();
        let mut rich_text = RichTextLabel::new_alloc();

        // build reverse
        vbox.add_child(label.clone().upcast());
        vbox.add_child(hsep.upcast());
        vbox.add_child(rich_text.clone().upcast());
        panel_margin.add_child(vbox.upcast());
        panel.add_child(panel_margin.clone().upcast());
        margin.add_child(panel.clone().upcast());
        self.base_mut().add_child(margin.clone().upcast());
        self.character_label = Some(label.clone());
        self.dialog_text = Some(rich_text.clone());

        // layout
        if let Some(label_settings) = &settings.bind().character_name_label_style {
            label.set_label_settings(label_settings.clone());
        }
        rich_text.set_use_bbcode(true);
        rich_text.set_text("[wave] Hello World! [/wave]".to_godot());
        rich_text.set_v_size_flags(SizeFlags::EXPAND_FILL);

        let font_size = settings.bind().dialog_font_size as i32;
        rich_text.add_theme_font_size_override(StringName::from("normal_font_size"), font_size);
        rich_text.add_theme_font_size_override(StringName::from("bold_font_size"), font_size);
        rich_text.add_theme_font_size_override(StringName::from("italics_font_size"), font_size);
        rich_text
            .add_theme_font_size_override(StringName::from("bold_italics_font_size"), font_size);
        rich_text.add_theme_font_size_override(StringName::from("mono_font_size"), font_size);

        panel.set_custom_minimum_size(Vector2 {
            x: 360.0f32, // x size managed by container
            y: 240.0f32, // push min size up
        });

        const PANEL_BOX: i32 = 32;
        panel_margin.add_theme_constant_override(StringName::from("margin_bottom"), PANEL_BOX);
        panel_margin.add_theme_constant_override(StringName::from("margin_top"), PANEL_BOX);
        panel_margin.add_theme_constant_override(StringName::from("margin_left"), PANEL_BOX);
        panel_margin.add_theme_constant_override(StringName::from("margin_right"), PANEL_BOX);

        let margin_lr = settings.bind().dialog_align.get_margins();
        const BOTTOM_MARGIN: i32 = 32;
        margin.add_theme_constant_override(StringName::from("margin_left"), margin_lr.0);
        margin.add_theme_constant_override(StringName::from("margin_right"), margin_lr.1);
        margin.add_theme_constant_override(StringName::from("margin_bottom"), BOTTOM_MARGIN);
        margin.set_anchors_and_offsets_preset(LayoutPreset::BOTTOM_WIDE);
        margin.force_update_transform();
        /*
        	*/
        // Animation
        let margin_size = margin.get_size();
        let margin_pos = margin.get_position();
        margin.set_position(Vector2 {
            x: margin_pos.x,
            y: margin_pos.y + margin_size.y,
        });
        // Handle Tween Creation
        let Some(tween) = &mut self.to_gd().create_tween() else {
            return;
        };
        //         Some(EaseType::from_ord(ease.get_property())),
        // Some(TransitionType::from_ord(trans.get_property())),

        let mut tween = tween
            .set_trans(TransitionType::from_ord(
                settings.bind().anim_appear_ease.get_property(),
            ))
            .unwrap();
        let mut tween = tween
            .set_ease(EaseType::from_ord(
                settings.bind().anim_appear_ease.get_property(),
            ))
            .unwrap();
        tween.tween_property(
            margin.upcast(),
            NodePath::from("position:y"),
            margin_pos.y.to_variant(),
            settings.bind().anim_appear_duration as f64,
        );
    }
    /*
        pub struct ChoiceOptionEntry {
            text: String,
            requires: String,
            action: String,
        }
    */
    fn create_options(&mut self, choices: &[ChoiceOptionEntry]) {
        let mut root = VBoxContainer::new_alloc();
        self.to_gd().add_child(root.clone().upcast());
        self.options_root = Some(root.clone().upcast());
        let mut is_first = self.get_settings().bind().auto_focus_choice_buttons;
        for (index, option) in choices.iter().enumerate() {
            if !option.requires.is_empty()
                && !CoreDialog::singleton()
                    .bind_mut()
                    .blackboard_query(option.requires.clone().into())
            {
                // does not meet conditions
                continue;
            }
            let mut button = Button::new_alloc();
            root.add_child(button.clone().upcast());
            button.set_text(self.parse_text(&option.text).into());
            let action = option.action.clone();
            if !action.is_empty() {
                self.state = DialogState::Pending;
            }
            button
                .connect_ex(
                    "pressed".into(),
                    Callable::from_fn(
                        format!("choice_button_{} ({})", index, option.text),
                        move |_| {
                            CoreDialog::singleton()
                                .bind_mut()
                                .blackboard_action(action.clone().into());
                            let Some(gui) = &mut CoreDialog::singleton().bind().gui.clone() else {
                                godot_error!(
                                    "Failed to find instance of the CoreDialog's DialogGUI"
                                );
                                return Err(());
                            };
                            gui.bind_mut().dialog_choice_was_made_callable(index);
                            Ok(Variant::nil())
                        },
                    ),
                )
                .flags(ConnectFlags::DEFERRED.ord() as u32)
                .done();
            if is_first {
                button.grab_focus();
                is_first = false;
            }
        }
        let align = self.get_settings().bind().choice_buttons_align.clone();
        root.set_anchors_and_offsets_preset(match align {
            DialogAlign::Left => LayoutPreset::CENTER_LEFT,
            DialogAlign::Right => LayoutPreset::CENTER_RIGHT,
            DialogAlign::Center => LayoutPreset::CENTER,
            DialogAlign::FullWide => LayoutPreset::VCENTER_WIDE,
        });
    }

    pub fn load_line(&mut self, track: &Line) {
        match track {
            #[allow(unused_variables)]
            Line::Text {
                text,
                character,
                requires,
            } => {
                let parsed_text = self.parse_text(text);
                let parsed_char = self.parse_text(character);

                if let Some(dialog_text) = self.dialog_text.as_mut() {
                    dialog_text.set_text(parsed_text.to_godot());
                }

                if let Some(character_label) = &mut self.character_label {
                    character_label.set_text(parsed_char.to_godot());
                }
            }
            Line::Choice {
                prompt,
                character,
                options,
            } => {
                let parsed_prompt = self.parse_text(prompt);
                let parsed_char = self.parse_text(character);
                if let Some(dialog_text) = &mut self.dialog_text {
                    dialog_text.set_text(parsed_prompt.to_godot());
                }
                if let Some(character_label) = &mut self.character_label {
                    character_label.set_text(parsed_char.to_godot());
                }
                self.create_options(options);
            }
            _ => {
                godot_warn!("DialogGUI does not handle Line of type: {:#?}", track);
            }
        }
        let wpm = self.get_settings().bind().words_per_minute;
        let mut tween = self.get_text_tween(EEaseType::InOut, ETransType::Linear);
        let Some(text) = &mut self.dialog_text else {
            return;
        };

        const ZERO_TEXT: f32 = 0f32;
        const FULL_TEXT: f32 = 1f32;
        const MINUTES_TO_SECONDS: f32 = 60.0;

        let words = text
            .get_text()
            .clone()
            .to_string()
            .split_whitespace()
            .count();
        let duration_seconds = (words as f32) * wpm.powi(-1) * MINUTES_TO_SECONDS;
        text.set_visible_ratio(ZERO_TEXT);
        tween.tween_property(
            text.clone().upcast(),
            NodePath::from("visible_ratio"),
            FULL_TEXT.to_variant(),
            duration_seconds as f64,
        );
    }

    fn get_next_text_line(&mut self) -> Option<Line> {
        let Some(track) = &mut self.track else {
            return None;
        };
        #[allow(unused_variables)]
        while let Some(line) = track.pop_front() {
            let result: Option<Line> = match line.clone() {
                Line::Text {
                    text,
                    character,
                    requires,
                } => {
                    if requires.is_empty()
                        || CoreDialog::singleton()
                            .bind_mut()
                            .blackboard_query(requires.to_godot())
                    {
                        Some(line)
                    } else {
                        None
                    }
                }
                Line::Choice {
                    prompt,
                    character,
                    options,
                } => Some(line),
                Line::Action { action } => {
                    self.state = DialogState::Pending;
                    godot_print!("Pending processing for action {}", action);
                    CoreDialog::singleton()
                        .call_deferred("blackboard_action".into(), &[action.to_variant()]);
                    return None; // force break to allow processing events
                }
                Line::Signal { name, args } => {
                    CoreDialog::singleton()
                        .bind_mut()
                        .handle_dialog_signal(&line);
                    continue;
                }
                Line::None => continue,
            };
            if result.is_some() {
                return result;
            }
        }
        None
    }

    fn get_text_tween(&mut self, ease: EEaseType, trans: ETransType) -> Gd<Tween> {
        if let Some(tw) = &mut self.tween {
            tw.kill();
        }
        if let Some(tween) = &mut SquigglesUtil::create_tween(
            &mut self.to_gd().upcast(),
            Some(EaseType::from_ord(ease.get_property())),
            Some(TransitionType::from_ord(trans.get_property())),
        ) {
            self.tween = Some(tween.clone());
            return tween.clone();
        };
        godot_warn!("Failed to create tween for DialogGUI");
        let tween = self.base_mut().create_tween().unwrap();
        self.tween = Some(tween.clone());
        tween
    }

    fn get_settings(&self) -> Gd<DialogSettings> {
        CoreDialog::singleton()
            .bind()
            .get_override_settings()
            .unwrap_or(
                CoreGlobals::singleton()
                    .bind()
                    .get_config()
                    .bind()
                    .get_dialog()
                    .unwrap_or(DialogSettings::new_gd()),
            )
    }

    fn dialog_choice_was_made_callable(&mut self, _index: usize) {
        let Some(root) = &mut self.options_root.clone() else {
            return;
        };
        root.queue_free();
        self.options_root = None;
        self.load_next_line();
    }

    #[func]
    pub fn make_dialog_choice(&mut self, index: i32) -> bool {
        let Some(root) = self.options_root.clone() else {
            return false;
        };
        let Some(child) = root.get_child(index) else {
            return false;
        };
        let Ok(child) = &mut child.try_cast::<Button>() else {
            return false;
        };
        child.set_deferred("pressed".into(), true.to_variant());
        true
    }

    fn load_next_line(&mut self) {
        if self.state == DialogState::Pending {
            return;
        }

        let Some(margin) = self.base().get_child(0) else {
            godot_error!("Failed to access child of DialogGUI!");
            return;
        };
        let margin = margin.cast::<Control>();
        if let Some(line) = self.get_next_text_line() {
            self.load_line(&line);
        } else {
            let settings = self.get_settings();
            let mut tween = self.get_text_tween(
                settings.bind().anim_hide_ease.clone(),
                settings.bind().anim_hide_trans.clone(),
            );
            let margin_size = margin.get_size();
            let margin_pos = margin.get_position();

            tween.tween_property(
                margin.upcast(),
                NodePath::from("position:y"),
                (margin_pos.y + margin_size.y).to_variant(),
                settings.bind().get_anim_hide_duration() as f64,
            );
            tween.tween_callback(Callable::from_object_method(&self.to_gd(), "queue_free"));
        }
    }

    pub fn mark_event_handled(&mut self) {
        godot_print!("Action marked as handled");
        self.state = DialogState::Active;
    }

    fn parse_text(&self, in_text: &String) -> String {
        let trans = self.base().tr(in_text.into()).into();
        CoreDialog::singleton().bind().blackboard_parse(trans)
    }
}
