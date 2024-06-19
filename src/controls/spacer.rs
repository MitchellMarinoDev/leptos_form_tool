use super::{BuilderCxFn, BuilderFn, ControlRenderData, VanityControlBuilder, VanityControlData};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{prelude::Signal, View};
use std::rc::Rc;

/// Data used for the spacer control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SpacerData {
    pub height: Option<String>,
}

impl VanityControlData for SpacerData {
    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        _value_getter: Option<Signal<String>>,
    ) -> View {
        fs.spacer(control)
    }
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a spacer and adds it to the form.
    pub fn spacer(self, builder: impl BuilderFn<VanityControlBuilder<FD, SpacerData>>) -> Self {
        self.new_vanity(builder)
    }

    /// Builds a spacer using the form's context and adds it to the form.
    pub fn spacer_cx(
        self,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, SpacerData>, FD::Context>,
    ) -> Self {
        self.new_vanity_cx(builder)
    }
}

impl<FD: FormToolData> VanityControlBuilder<FD, SpacerData> {
    /// Sets the height of the spacer.
    ///
    /// This is a string to allow different units like "10px" or "1.25em".
    ///
    /// This may or may not be respected based on the Style implementation.
    pub fn height(mut self, height: impl ToString) -> Self {
        self.data.height = Some(height.to_string());
        self
    }
}
