//! Finding a Faust DSP's controls by their labels. The plugin sets every
//! control itself (they're not host parameters), so the labels are flat and
//! unique.

use std::collections::HashMap;

use faust_types::{FaustDsp, ParamIndex, UI};

#[derive(Default)]
pub struct Labels(HashMap<String, ParamIndex>);

impl Labels {
    pub fn of<D: FaustDsp<T = f32>>() -> Self {
        let mut labels = Self::default();
        D::build_user_interface_static(&mut labels);
        labels
    }

    /// The control labelled `label`. Panics if there's none: the DSP and the
    /// plugin disagree, which the tests catch.
    pub fn get(&self, label: &str) -> ParamIndex {
        *self.0.get(label).unwrap_or_else(|| panic!("no Faust control '{label}'"))
    }

    fn add(&mut self, label: &str, param: ParamIndex) {
        let clash = self.0.insert(label.to_string(), param);
        assert!(clash.is_none(), "two Faust controls labelled '{label}'");
    }
}

impl UI<f32> for Labels {
    fn open_tab_box(&mut self, _: &str) {}
    fn open_horizontal_box(&mut self, _: &str) {}
    fn open_vertical_box(&mut self, _: &str) {}
    fn close_box(&mut self) {}
    fn add_button(&mut self, label: &str, param: ParamIndex) {
        self.add(label, param);
    }
    fn add_check_button(&mut self, label: &str, param: ParamIndex) {
        self.add(label, param);
    }
    fn add_vertical_slider(&mut self, label: &str, param: ParamIndex, _: f32, _: f32, _: f32, _: f32) {
        self.add(label, param);
    }
    fn add_horizontal_slider(&mut self, label: &str, param: ParamIndex, _: f32, _: f32, _: f32, _: f32) {
        self.add(label, param);
    }
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, _: f32, _: f32, _: f32, _: f32) {
        self.add(label, param);
    }
    fn add_horizontal_bargraph(&mut self, _: &str, _: ParamIndex, _: f32, _: f32) {}
    fn add_vertical_bargraph(&mut self, _: &str, _: ParamIndex, _: f32, _: f32) {}
    fn declare(&mut self, _: Option<ParamIndex>, _: &str, _: &str) {}
}
