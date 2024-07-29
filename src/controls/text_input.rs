use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidatedControlData,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{Signal, SignalSetter, View};
use std::rc::Rc;

/// Data used for the text input control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TextInputData {
    pub name: String,
    pub label: Option<String>,
    pub placeholder: Option<String>,
    pub input_type: &'static str,
}

impl Default for TextInputData {
    fn default() -> Self {
        TextInputData {
            name: String::new(),
            placeholder: None,
            label: None,
            input_type: "input",
        }
    }
}

impl ControlData for TextInputData {
    type ReturnType = String;

    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        fs.text_input(control, value_getter, value_setter, validation_state)
    }
}
impl ValidatedControlData for TextInputData {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a text input control and adds it to the form.
    pub fn text_input<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, TextInputData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a text input control using the form's context and adds it to
    /// the form.
    pub fn text_input_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, TextInputData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, TextInputData, FDT> {
    /// Sets the name of the text input.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label for the text input.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Sets the placeholder for the text input.
    pub fn placeholder(mut self, placeholder: impl ToString) -> Self {
        self.data.placeholder = Some(placeholder.to_string());
        self
    }

    /// Sets the text input to be the "password" type.
    pub fn password(mut self) -> Self {
        self.data.input_type = "password";
        self
    }

    /// Sets the text input to be the "date" type.
    pub fn date(mut self) -> Self {
        self.data.input_type = "date";
        self
    }

    /// Sets the text input to be the specified type.
    pub fn input_type(mut self, input_type: &'static str) -> Self {
        self.data.input_type = input_type;
        self
    }
}
