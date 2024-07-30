use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidatedControlData,
    ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{Signal, SignalSetter, View};
use std::rc::Rc;

/// Data used for the radio buttons control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RadioButtonsData {
    pub name: String,
    pub label: Option<String>,
    /// The options for the select.
    ///
    /// The first value is the string to display, the second is the value.
    pub options: Vec<(String, String)>,
}

impl ControlData for RadioButtonsData {
    type ReturnType = String;

    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        fs.radio_buttons(control, value_getter, value_setter, validation_state)
    }
}
impl ValidatedControlData for RadioButtonsData {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a radio buttons control and adds it to the form.
    pub fn radio_buttons<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, RadioButtonsData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a radio buttons control using the form's context and adds it to
    /// the form.
    pub fn radio_buttons_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, RadioButtonsData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, RadioButtonsData, FDT> {
    /// Sets the name of the radio button inputs.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label for the radio button group.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Adds the option to the radio button group.
    pub fn with_option(mut self, option: impl ToString) -> Self {
        self.data
            .options
            .push((option.to_string(), option.to_string()));
        self
    }

    /// Adds the option to the radio button group, specifying a different
    /// value than what is displayed.
    pub fn with_option_valued(mut self, display: impl ToString, value: impl ToString) -> Self {
        self.data
            .options
            .push((display.to_string(), value.to_string()));
        self
    }

    /// Adds all the options in the provided iterator to the radio button
    /// group.
    pub fn with_options(mut self, options: impl Iterator<Item = impl ToString>) -> Self {
        for option in options {
            self.data
                .options
                .push((option.to_string(), option.to_string()));
        }
        self
    }

    /// Adds all the (display_string, value) pairs in the provided iterator
    /// to the radio button group.
    pub fn with_options_valued(
        mut self,
        options: impl Iterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        for option in options {
            self.data
                .options
                .push((option.0.to_string(), option.1.to_string()));
        }
        self
    }
}
