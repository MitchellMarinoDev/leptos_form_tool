use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidatedControlData,
    ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{IntoSignal, MaybeSignal, Signal, SignalGet, SignalSetter, View};
use std::rc::Rc;

/// Data used for the select control.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SelectData {
    pub name: String,
    pub label: Option<String>,
    /// The options for the select.
    ///
    /// The first value is the string to display, the second is the value.
    pub options: MaybeSignal<Vec<(String, String)>>,
    /// The display text for the blank option, if there is one.
    pub blank_option: Option<String>,
}

impl ControlData for SelectData {
    type ReturnType = String;

    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        fs.select(control, value_getter, value_setter, validation_state)
    }
}
impl ValidatedControlData for SelectData {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a select control and adds it to the form.
    pub fn select<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, SelectData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a select control using the form's context and adds it to the
    /// form.
    pub fn select_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, SelectData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, SelectData, FDT> {
    /// Sets the name of the radio button inputs.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label for the select.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Sets the options from the provided iterator.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options(mut self, options: impl Iterator<Item = impl ToString>) -> Self {
        let options = options.map(|v| (v.to_string(), v.to_string())).collect();
        self.data.options = MaybeSignal::Static(options);
        self
    }

    /// Sets the options to the (display_string, value) pairs from the
    /// provided iterator.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options_valued(
        mut self,
        options: impl Iterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        let options = options
            .map(|(d, v)| (d.to_string(), v.to_string()))
            .collect();
        self.data.options = MaybeSignal::Static(options);
        self
    }

    /// Sets the options from the provided signal.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options_signal(mut self, options: Signal<Vec<String>>) -> Self {
        let options = move || {
            options
                .get()
                .into_iter()
                .map(|v| (v.clone(), v))
                .collect::<Vec<_>>()
        };
        self.data.options = MaybeSignal::Dynamic(options.into_signal());
        self
    }

    /// Sets the options to the (display_string, value) pairs from the
    /// provided signal.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options_valued_signal(mut self, options: Signal<Vec<(String, String)>>) -> Self {
        self.data.options = MaybeSignal::Dynamic(options);
        self
    }

    /// Adds a blank option as the first option for the select.
    pub fn with_blank_option(mut self) -> Self {
        self.data.blank_option = Some(String::new());
        self
    }

    /// Adds a blank option as the first option for the select,
    /// but sets the display string to the given value.
    pub fn with_blank_option_displayed(mut self, display: impl ToString) -> Self {
        self.data.blank_option = Some(display.to_string());
        self
    }
}
