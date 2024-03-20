use godot::{
    engine::{
        control::LayoutPreset,
        global::{HorizontalAlignment, VerticalAlignment},
        HSlider, IHSlider, Label,
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

    label_title: Option<Gd<Label>>,
    label_min: Option<Gd<Label>>,
    label_max: Option<Gd<Label>>,
    label_current: Option<Gd<Label>>,

    base: Base<HSlider>,
}

#[godot_api]
impl IHSlider for LabelledHSlider {
    fn enter_tree(&mut self) {
        //pass
        let mut l_title = Label::new_alloc();
        let mut l_min = Label::new_alloc();
        let mut l_max = Label::new_alloc();
        let mut l_cur = Label::new_alloc();
        l_title.set_text(self.text.clone());
        l_min.set_text(Self::as_text(self.base().get_min()));
        l_max.set_text(Self::as_text(self.base().get_max()));
        l_cur.set_text(Self::as_text(self.base().get_value()));

        let call_range = Callable::from_object_method(&self.to_gd(), "update_range");
        let call_value = Callable::from_object_method(&self.to_gd(), "update_value");
        self.base_mut().add_child(l_title.clone().upcast());
        self.base_mut().add_child(l_min.clone().upcast());
        self.base_mut().add_child(l_max.clone().upcast());
        self.base_mut().add_child(l_cur.clone().upcast());
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
    fn update_range(&mut self) {
        let min = self.base().get_min();
        let max = self.base().get_max();
        if let Some(l_min) = &mut self.label_min {
            l_min.set_text(Self::as_text(min));
        }
        if let Some(l_max) = &mut self.label_max {
            l_max.set_text(Self::as_text(max));
        }
    }

    #[func]
    fn update_value(&mut self, value: f64) {
        let Some(label) = &mut self.label_current else {
            return;
        };
        label.set_text(Self::as_text(value));
    }

    fn as_text(num: f64) -> GString {
        format!("{}", num).to_godot()
    }
}
