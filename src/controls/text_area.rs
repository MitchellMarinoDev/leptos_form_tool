use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidatedControlData,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{Signal, SignalSetter, View};
use std::rc::Rc;

/// Data used for the text area control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct TextAreaData {
    pub name: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
}

impl ControlData for TextAreaData {
    type ReturnType = String;

    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        fs.text_area(control, value_getter, value_setter, validation_state)
    }
}
impl ValidatedControlData for TextAreaData {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a text area control and adds it to the form.
    pub fn text_area<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, TextAreaData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a text area control using the forms context and adds it to the
    /// form.
    pub fn text_area_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, TextAreaData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, TextAreaData, FDT> {
    /// Sets the name of the text area.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label for the text area.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Sets the placeholder for the text area.
    pub fn placeholder(mut self, placeholder: impl ToString) -> Self {
        self.data.placeholder = Some(placeholder.to_string());
        self
    }
}
