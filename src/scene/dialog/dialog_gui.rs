use godot::{
    engine::{
        control::{LayoutPreset, SizeFlags},
        tween::{EaseType, TransitionType},
        CanvasLayer, HSeparator, ICanvasLayer, InputEvent, Label, LabelSettings, MarginContainer,
        PanelContainer, RichTextLabel, Tween, VBoxContainer,
    },
    prelude::*,
};

use crate::{scene::game_globals::CoreGlobals, util::SquigglesUtil};

use super::{dialog_manager::CoreDialog, dialog_track::Line};

#[derive(GodotClass)]
#[class(init, base=CanvasLayer)]
pub struct DialogGUI {
    tween: Option<Gd<Tween>>,
    pub track: Option<Vec<Line>>,
    character_label: Option<Gd<Label>>,
    dialog_text: Option<Gd<RichTextLabel>>,
    current_index: usize,
    #[base]
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
        } else {
            godot_warn!(
                "No text nodes found in dialog track on load. Something must have gone wrong?"
            );
        }
    }

    fn input(&mut self, _event: Gd<InputEvent>) {}

    fn exit_tree(&mut self) {
        //pass
    }
}

impl DialogGUI {
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
        let settings = CoreDialog::singleton()
            .bind()
            .get_override_settings()
            .unwrap_or(
                CoreGlobals::singleton()
                    .bind()
                    .get_config()
                    .bind()
                    .get_dialog()
                    .unwrap_or(DialogSettings::new_gd()),
            );

        // create instances.
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
        self.base.add_child(margin.clone().upcast());
        self.character_label = Some(label.clone());
        self.dialog_text = Some(rich_text.clone());

        // layout
        if let Some(label_settings) = &settings.bind().character_name_label_style {
            label.set_label_settings(label_settings.clone());
        }
        rich_text.set_use_bbcode(true);
        rich_text.set_text("[wave] Hello World! [/wave]".to_godot());
        rich_text.set_v_size_flags(SizeFlags::SIZE_EXPAND_FILL);

        let font_size = settings.bind().dialog_font_size as i32;
        rich_text.add_theme_font_size_override(StringName::from("normal_font_size"), font_size);
        rich_text.add_theme_font_size_override(StringName::from("bold_font_size"), font_size);
        rich_text.add_theme_font_size_override(StringName::from("italics_font_size"), font_size);
        rich_text
            .add_theme_font_size_override(StringName::from("bold_italics_font_size"), font_size);
        rich_text.add_theme_font_size_override(StringName::from("mono_font_size"), font_size);

        panel.set_custom_minimum_size(Vector2 {
            x: 0f32,     // x size managed by container
            y: 240.0f32, // push min size up
        });

        let panel_box = 32;
        panel_margin.add_theme_constant_override(StringName::from("margin_bottom"), panel_box);
        panel_margin.add_theme_constant_override(StringName::from("margin_top"), panel_box);
        panel_margin.add_theme_constant_override(StringName::from("margin_left"), panel_box);
        panel_margin.add_theme_constant_override(StringName::from("margin_right"), panel_box);

        margin.set_anchors_and_offsets_preset(LayoutPreset::PRESET_BOTTOM_WIDE);
        let margin_lr: (i32, i32) = match settings.bind().dialog_align {
            DialogAlign::Left => (32, 256),
            DialogAlign::Right => (256, 32),
            DialogAlign::Center => (256, 256),
            DialogAlign::FullWide => (32, 32),
        };
        margin.add_theme_constant_override(StringName::from("margin_left"), margin_lr.0);
        margin.add_theme_constant_override(StringName::from("margin_right"), margin_lr.1);
        margin.add_theme_constant_override(StringName::from("margin_bottom"), 32);

        // Handle Tween Creation
        if let Some(tw) = &mut self.tween {
            tw.kill();
        }
        let Some(tween) = &mut SquigglesUtil::create_tween(
            &mut self.to_gd().upcast(),
            Some(EaseType::from_ord(
                settings.bind().anim_appear_ease.get_property(),
            )),
            Some(TransitionType::from_ord(
                settings.bind().anim_appear_trans.get_property(),
            )),
        ) else {
            godot_warn!("Failed to create tween for DialogGUI");
            return;
        };
        self.tween = Some(tween.clone());

        // Animation
        let margin_size = margin.get_size();
        let margin_pos = margin.get_position();
        margin.set_position(Vector2 {
            x: margin_pos.x,
            y: margin_pos.y + margin_size.y,
        });
        tween.tween_property(
            margin.upcast(),
            NodePath::from("position:y"),
            margin_pos.y.to_variant(),
            settings.bind().anim_appear_duration as f64,
        );
    }

    pub fn load_line(&mut self, track: &Line) {
        match track {
            Line::Text { text, character } => {
                if let Some(dialog_text) = &mut self.dialog_text {
                    dialog_text.set_text(text.to_godot());
                }
                if let Some(character_label) = &mut self.character_label {
                    character_label.set_text(character.to_godot());
                }
            }
            Line::Choice {
                prompt,
                character,
                options,
            } => {
                let _ = options;
                if let Some(dialog_text) = &mut self.dialog_text {
                    dialog_text.set_text(prompt.to_godot());
                }
                if let Some(character_label) = &mut self.character_label {
                    character_label.set_text(character.to_godot());
                }
                // TODO: add options
            }
            _ => {
                godot_warn!("DialogGUI does not handle Line of type: {:#?}", track);
            }
        }
    }

    fn get_next_text_line(&mut self) -> Option<Line> {
        let Some(track) = &mut self.track else {
            return None;
        };
        track.reverse();
        #[allow(unused_variables)]
        while let Some(line) = track.pop() {
            let result: Option<Line> = match line.clone() {
                Line::Text { text, character } => Some(line),
                Line::Choice {
                    prompt,
                    character,
                    options,
                } => Some(line),
                Line::Action { action } => {
                    CoreDialog::singleton().bind_mut().handle_line_action(&line);
                    continue;
                }
                Line::Signal { name, args } => {
                    CoreDialog::singleton().bind_mut().handle_line_action(&line);
                    continue;
                }
                Line::None => continue,
            };
            if result.is_some() {
                if !track.is_empty() {
                    track.reverse();
                }
                return result;
            }
        }
        None
    }
}

#[repr(u32)]
#[derive(Property, Default, Export)]
enum DialogAlign {
    Left = 0,
    Right = 1,
    #[default]
    Center = 2,
    FullWide = 3,
}

#[repr(i32)]
#[derive(Property, Default, Export)]
enum EEaseType {
    #[default]
    In = 0,
    Out = 1,
    InOut = 2,
    OutIn = 3,
}

#[repr(i32)]
#[derive(Property, Default, Export)]
enum ETransType {
    #[default]
    Linear = 0,
    Sine = 1,
    Quint = 2,
    Quart = 3,
    Quad = 4,
    Expo = 5,
    Elastic = 6,
    Cubic = 7,
    Circ = 8,
    Bounce = 9,
    Back = 10,
    Spring = 11,
}
#[derive(GodotClass)]
#[class(base=Resource)]
pub struct DialogSettings {
    #[export]
    character_name_label_style: Option<Gd<LabelSettings>>,
    #[export]
    dialog_font_size: u32,
    #[export]
    dialog_align: DialogAlign,
    #[export]
    interact_action: GString,
    #[export]
    anim_appear_ease: EEaseType,
    #[export]
    anim_appear_trans: ETransType,
    #[export]
    anim_appear_duration: f32,
    #[export]
    anim_hide_ease: EEaseType,
    #[export]
    anim_hide_trans: ETransType,
    #[export]
    anim_hide_duration: f32,

    #[base]
    base: Base<Resource>,
}

#[godot_api]
impl IResource for DialogSettings {
    fn init(base: Base<Resource>) -> Self {
        Self {
            base,
            dialog_font_size: 22u32,
            character_name_label_style: None,
            dialog_align: DialogAlign::Center,
            interact_action: "interact".to_godot(),
            anim_appear_duration: 1f32,
            anim_hide_duration: 1f32,
            anim_appear_ease: Default::default(),
            anim_appear_trans: Default::default(),
            anim_hide_ease: Default::default(),
            anim_hide_trans: Default::default(),
        }
    }
}
