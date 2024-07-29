use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidatedControlData,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{MaybeSignal, Signal, SignalSetter, View};
use std::rc::Rc;

/// Data used for the stepper control.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct StepperData {
    pub name: String,
    pub label: Option<String>,
    pub step: Option<MaybeSignal<i32>>,
    pub min: Option<MaybeSignal<i32>>,
    pub max: Option<MaybeSignal<i32>>,
}

impl ControlData for StepperData {
    type ReturnType = String;

    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        fs.stepper(control, value_getter, value_setter, validation_state)
    }
}
impl ValidatedControlData for StepperData {}

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
    pub fn step(mut self, step: i32) -> Self {
        self.data.step = Some(MaybeSignal::Static(step));
        self
    }

    /// Sets the step ammount.
    pub fn step_signal(mut self, step: Signal<i32>) -> Self {
        self.data.step = Some(MaybeSignal::Dynamic(step));
        self
    }

    /// Sets the minimum value for the slider.
    pub fn min(mut self, min: i32) -> Self {
        self.data.min = Some(MaybeSignal::Static(min));
        self
    }

    /// Sets the minimum value for the slider to a signal.
    pub fn min_signal(mut self, min: Signal<i32>) -> Self {
        self.data.min = Some(MaybeSignal::Dynamic(min));
        self
    }

    /// Sets the maximum value for the slider.
    pub fn max(mut self, max: i32) -> Self {
        self.data.max = Some(MaybeSignal::Static(max));
        self
    }

    /// Sets the maximum value for the slider to a signal.
    pub fn max_signal(mut self, max: Signal<i32>) -> Self {
        self.data.max = Some(MaybeSignal::Dynamic(max));
        self
    }
}
