use super::{
    BuilderCxFn, BuilderFn, ControlRenderData, GetterVanityControlData, VanityControlBuilder,
    VanityControlData,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{Signal, View};
use std::rc::Rc;

/// Data used for the hidden control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct HiddenData {
    pub name: String,
}

impl VanityControlData for HiddenData {
    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        fs.hidden(control, value_getter)
    }
}
impl GetterVanityControlData for HiddenData {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a hidden form control and adds it to the form.
    ///
    /// This will be an input in the html form allowing you to insert some
    /// data the you might not want the user modifying.
    pub fn hidden(self, builder: impl BuilderFn<VanityControlBuilder<FD, HiddenData>>) -> Self {
        self.new_vanity(builder)
    }

    /// Builds a hidden form control using the form's context  and adds it to
    /// the form.
    ///
    /// This will be an input in the html form allowing you to insert some
    /// data the you might not want the user modifying.
    pub fn hidden_cx(
        self,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, HiddenData>, FD::Context>,
    ) -> Self {
        self.new_vanity_cx(builder)
    }
}

impl<FD: FormToolData> VanityControlBuilder<FD, HiddenData> {
    /// Sets the name of the hidden control.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }
}
