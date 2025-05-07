use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidatedControlData,
    ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{
    prelude::{AnyView, RwSignal, Signal},
    reactive::wrappers::write::SignalSetter,
};
use std::sync::Arc;

/// Data used for the stepper control.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StepperData {
    pub name: String,
    pub label: Option<String>,
    pub step: Option<Signal<String>>,
    pub min: Option<Signal<String>>,
    pub max: Option<Signal<String>>,
}

impl<FD: FormToolData> ControlData<FD> for StepperData {
    /// String, as a user can still enter characters in a number fields.
    type ReturnType = String;

    fn render_control<FS: FormStyle>(
        fs: &FS,
        _fd: RwSignal<FD>,
        control: Arc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<ValidationState>,
    ) -> AnyView {
        fs.stepper(control, value_getter, value_setter, validation_state)
    }
}
impl<FD: FormToolData> ValidatedControlData<FD> for StepperData {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a stepper control and adds it to the form.
    pub fn stepper<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, StepperData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a new stepper control using the form's context and adds it to
    /// the form.
    pub fn stepper_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, StepperData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, StepperData, FDT> {
    /// Sets the name of the stepper control.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label of the stepper.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Sets the step ammount.
    pub fn step(mut self, step: impl ToString) -> Self {
        self.data.step = Some(Signal::stored(step.to_string()));
        self
    }

    /// Sets the step ammount to a signal.
    pub fn step_signal(mut self, step: Signal<String>) -> Self {
        self.data.step = Some(step);
        self
    }

    /// Sets the minimum value for the stepper.
    pub fn min(mut self, min: impl ToString) -> Self {
        self.data.min = Some(Signal::stored(min.to_string()));
        self
    }

    /// Sets the minimum value for the stepper to a signal.
    pub fn min_signal(mut self, min: Signal<String>) -> Self {
        self.data.min = Some(min);
        self
    }

    /// Sets the maximum value for the stepper.
    pub fn max(mut self, max: impl ToString) -> Self {
        self.data.max = Some(Signal::stored(max.to_string()));
        self
    }

    /// Sets the maximum value for the stepper to a signal.
    pub fn max_signal(mut self, max: Signal<String>) -> Self {
        self.data.max = Some(max);
        self
    }
}
