use super::{
    BuilderCxFn, BuilderFn, ControlRenderData, GetterVanityControlData, VanityControlBuilder,
    VanityControlData,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{Signal, View};
use std::rc::Rc;

/// Data used for the output control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct OutputData;

impl VanityControlData for OutputData {
    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        fs.output(control, value_getter)
    }
}
impl GetterVanityControlData for OutputData {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds an output form control and adds it to the form.
    ///
    /// This control allows you to output some text to the user based on the
    /// form data.
    pub fn output(self, builder: impl BuilderFn<VanityControlBuilder<FD, OutputData>>) -> Self {
        self.new_vanity(builder)
    }

    /// Builds an output form control using the form's context and adds it to
    /// the form.
    ///
    /// This control allows you to output some text to the user based on the
    /// form data and form context.
    pub fn output_cx(
        self,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, OutputData>, FD::Context>,
    ) -> Self {
        self.new_vanity_cx(builder)
    }
}
