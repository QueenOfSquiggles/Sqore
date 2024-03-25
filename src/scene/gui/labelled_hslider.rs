use godot::{
    engine::{
        control::LayoutPreset,
        global::{HorizontalAlignment, VerticalAlignment},
        HSlider, IHSlider, Label, ResourceLoader, Texture2D, TextureButton,
    },
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(tool, init, base=HSlider)]
pub struct LabelledHSlider {
    #[export]
    #[var(get, set=set_text)]
    text: GString,

    #[export]
    #[var(get, set=set_reset_enabled)]
    enable_reset: bool,

    #[export(range=(0.0, 10.0))]
    #[var(get, set=set_label_decimal_places)]
    label_decimal_places: u32,

    #[var]
    reset_value: f64,

    button_reset: Option<Gd<TextureButton>>,
    label_title: Option<Gd<Label>>,
    label_min: Option<Gd<Label>>,
    label_max: Option<Gd<Label>>,
    label_current: Option<Gd<Label>>,

    base: Base<HSlider>,
}

#[godot_api]
impl IHSlider for LabelledHSlider {
    fn ready(&mut self) {
        //pass
        let mut l_title = Label::new_alloc();
        let mut l_min = Label::new_alloc();
        let mut l_max = Label::new_alloc();
        let mut l_cur = Label::new_alloc();
        let mut b_reset = TextureButton::new_alloc();

        l_title.set_text(self.text.clone());
        l_min.set_text(self.as_text(self.base().get_min()));
        l_max.set_text(self.as_text(self.base().get_max()));
        l_cur.set_text(self.as_text(self.base().get_value()));
        b_reset.set_tooltip_text("Reset".into());

        if let Some(texture) = ResourceLoader::singleton()
            .load("res://addons/sqore/assets/runtime/undo_FILL0_wght400_GRAD0_opsz24.svg".into())
        {
            if let Ok(texture) = texture.try_cast::<Texture2D>() {
                b_reset.set_texture_normal(texture.clone());
                b_reset.set_texture_hover(texture.clone());
                b_reset.set_texture_pressed(texture);
                b_reset.set_custom_minimum_size(Vector2 { x: 16.0, y: 16.0 });
            }
        }

        let call_range = Callable::from_object_method(&self.to_gd(), "update_range");
        let call_value = Callable::from_object_method(&self.to_gd(), "update_value");
        self.base_mut().add_child(l_title.clone().upcast());
        self.base_mut().add_child(l_min.clone().upcast());
        self.base_mut().add_child(l_max.clone().upcast());
        self.base_mut().add_child(l_cur.clone().upcast());
        self.base_mut().add_child(b_reset.clone().upcast());
        self.base_mut()
            .connect(StringName::from("changed"), call_range);
        self.base_mut()
            .connect(StringName::from("value_changed"), call_value);

        l_title.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);
        l_title.set_vertical_alignment(VerticalAlignment::TOP);
        l_title.set_horizontal_alignment(HorizontalAlignment::LEFT);

        l_min.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);
        l_min.set_vertical_alignment(VerticalAlignment::BOTTOM);
        l_min.set_horizontal_alignment(HorizontalAlignment::LEFT);

        l_max.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);
        l_max.set_vertical_alignment(VerticalAlignment::BOTTOM);
        l_max.set_horizontal_alignment(HorizontalAlignment::RIGHT);

        l_cur.set_anchors_and_offsets_preset(LayoutPreset::FULL_RECT);
        l_cur.set_vertical_alignment(VerticalAlignment::BOTTOM);
        l_cur.set_horizontal_alignment(HorizontalAlignment::CENTER);

        b_reset.set_anchors_and_offsets_preset(LayoutPreset::TOP_RIGHT);
        b_reset.connect(
            StringName::from("pressed"),
            Callable::from_object_method(&self.to_gd(), "reset"),
        );
        self.reset_value = self.base().get_value();
        self.label_current = Some(l_cur);
        self.label_max = Some(l_max);
        self.label_min = Some(l_min);
        self.button_reset = Some(b_reset);

        self.set_reset_enabled(self.enable_reset);
    }
}

#[godot_api]
impl LabelledHSlider {
    #[func]
    fn set_text(&mut self, n_text: GString) {
        self.text = n_text.clone();
        if let Some(label) = &mut self.label_title {
            label.set_text(n_text);
        }
    }

    #[func]
    fn set_reset_enabled(&mut self, n_enabled: bool) {
        self.enable_reset = n_enabled;
        if let Some(btn) = &mut self.button_reset {
            btn.set_disabled(!n_enabled);
            btn.set_visible(n_enabled);
        }
    }

    #[func]
    fn update_range(&mut self) {
        let min = self.base().get_min();
        let max = self.base().get_max();
        let min_text = self.as_text(min);
        let max_text = self.as_text(max);
        if let Some(l_min) = &mut self.label_min {
            l_min.set_text(min_text);
        }
        if let Some(l_max) = &mut self.label_max {
            l_max.set_text(max_text);
        }
    }

    #[func]
    fn update_value(&mut self, value: f64) {
        let text = self.as_text(value);
        let Some(label) = &mut self.label_current else {
            return;
        };
        label.set_text(text);
    }

    #[func]
    fn reset(&mut self) {
        let nval = self.reset_value;
        self.base_mut().set_value(nval);
    }

    #[func]
    fn set_label_decimal_places(&mut self, n_value: u32) {
        self.label_decimal_places = n_value;

        // update labels
        let val = self.base().get_value();
        self.update_range();
        self.update_value(val);
    }

    fn as_text(&self, num: f64) -> GString {
        let target_decimal = self.label_decimal_places as usize;
        // Rust is mega chad for letting me specify the precision using another variable instead of needing a hardcoded string
        format!("{0:.1$}", num, target_decimal).to_godot()
    }
}
