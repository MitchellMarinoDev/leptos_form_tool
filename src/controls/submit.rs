use super::{BuilderCxFn, BuilderFn, ControlRenderData, VanityControlBuilder, VanityControlData};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{prelude::Signal, View};
use std::rc::Rc;

/// Data used for the submit button control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SubmitData {
    pub text: String,
}

impl VanityControlData for SubmitData {
    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        _value_getter: Option<Signal<String>>,
    ) -> View {
        fs.submit(control)
    }
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a submit button and adds it to the form.
    pub fn submit(self, builder: impl BuilderFn<VanityControlBuilder<FD, SubmitData>>) -> Self {
        self.new_vanity(builder)
    }

    /// Builds a submit button using the form's context and adds it to the
    /// form.
    pub fn submit_cx(
        self,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, SubmitData>, FD::Context>,
    ) -> Self {
        self.new_vanity_cx(builder)
    }
}

impl<FD: FormToolData> VanityControlBuilder<FD, SubmitData> {
    /// Sets the submit button's text.
    pub fn text(mut self, text: impl ToString) -> Self {
        self.data.text = text.to_string();
        self
    }
}
