use godot::{engine::LabelSettings, prelude::*};

#[derive(GodotConvert, Var, Default, Export, Clone)]
#[godot(via=i64)]
pub enum DialogAlign {
    Left = 0,
    Right = 1,
    #[default]
    Center = 2,
    FullWide = 3,
}

impl DialogAlign {
    pub fn get_margins(&self) -> (i32, i32) {
        match self {
            // icky hardcoded values :P
            // I really want 'margin-left: 15%'
            // TODO: have a utility system for getting the viewport pixels for a certain percent of width or height
            DialogAlign::Left => (32, 256),
            DialogAlign::Right => (256, 32),
            DialogAlign::Center => (256, 256),
            DialogAlign::FullWide => (32, 32),
        }
    }
}

#[derive(GodotConvert, Var, Default, Export, Clone)]
#[godot(via = i64)]
pub enum EEaseType {
    #[default]
    In = 0,
    Out = 1,
    InOut = 2,
    OutIn = 3,
}

#[derive(GodotConvert, Var, Default, Export, Clone)]
#[godot(via=i64)]
pub enum ETransType {
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
#[class(init, base=Resource)]
pub struct DialogSettings {
    #[export]
    pub character_name_label_style: Option<Gd<LabelSettings>>,

    #[export]
    #[init(default = 22u32)]
    pub dialog_font_size: u32,

    #[export]
    pub dialog_align: DialogAlign,

    #[export]
    #[init(default = "interact".to_godot())]
    pub interact_action: GString,

    #[export]
    pub anim_appear_ease: EEaseType,

    #[export]
    pub anim_appear_trans: ETransType,

    #[export]
    #[init(default = 1f32)]
    pub anim_appear_duration: f32,

    #[export]
    pub anim_hide_ease: EEaseType,

    #[export]
    pub anim_hide_trans: ETransType,

    #[export]
    #[init(default = 1f32)]
    pub anim_hide_duration: f32,

    #[export]
    #[init(default = true)]
    pub auto_focus_choice_buttons: bool,

    #[export]
    pub choice_buttons_align: DialogAlign,

    #[export]
    #[init(default = 150f32)]
    pub words_per_minute: f32,

    base: Base<Resource>,
}
