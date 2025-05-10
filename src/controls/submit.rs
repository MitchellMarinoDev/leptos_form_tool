use super::{
    BuilderCxFn, BuilderFn, ControlRenderData, GetterVanityControlData, VanityControlBuilder,
    VanityControlData,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::prelude::{AnyView, RwSignal, Signal};
use std::sync::Arc;

/// Data used for the submit button control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SubmitData;

impl<FD: FormToolData> VanityControlData<FD> for SubmitData {
    fn render_control<FS: FormStyle>(
        fs: &FS,
        _fd: RwSignal<FD>,
        control: ControlRenderData<FS, Self>,
        value_getter: Option<Signal<String>>,
    ) -> AnyView {
        fs.submit(control, value_getter)
    }
}
impl<FD: FormToolData> GetterVanityControlData<FD> for SubmitData {}

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
    /// Sets the text of the submit button to a static string.
    ///
    /// For dynamic button text, use the `getter` method.
    pub fn text(mut self, text: impl ToString) -> Self {
        let text = text.to_string();
        self.getter = Some(Arc::new(move |_| text.clone()));
        self
    }
}
