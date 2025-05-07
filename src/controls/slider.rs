use leptos::{
    prelude::{AnyView, MaybeSignal, RwSignal, Signal},
    reactive::wrappers::write::SignalSetter,
};

use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use std::rc::Rc;

/// Data used for the slider control.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct SliderData {
    pub name: String,
    pub label: Option<String>,
    pub step: Option<MaybeSignal<String>>,
    pub min: Option<MaybeSignal<String>>,
    pub max: Option<MaybeSignal<String>>,
}

impl<FD: FormToolData> ControlData<FD> for SliderData {
    /// String to support integers or decimal point types.
    type ReturnType = String;

    fn render_control<FS: FormStyle>(
        fs: &FS,
        _fd: RwSignal<FD>,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<ValidationState>,
    ) -> AnyView {
        fs.slider(control, value_getter, value_setter, validation_state)
    }
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a slider (or range) control and adds it to the form.
    pub fn slider<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, SliderData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Bulids a slider (or range) control using the form's context and adds
    /// it to the form.
    pub fn slider_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, SliderData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, SliderData, FDT> {
    /// Sets the name of the slider.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label for the slider.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Sets the step ammount.
    pub fn step(mut self, step: impl ToString) -> Self {
        self.data.step = Some(MaybeSignal::Static(step.to_string()));
        self
    }

    /// Sets the step ammount to a signal.
    pub fn step_signal(mut self, step: Signal<String>) -> Self {
        self.data.step = Some(MaybeSignal::Dynamic(step));
        self
    }

    /// Sets the minimum value for the slider.
    pub fn min(mut self, min: impl ToString) -> Self {
        self.data.min = Some(MaybeSignal::Static(min.to_string()));
        self
    }

    /// Sets the minimum value for the slider to a signal.
    pub fn min_signal(mut self, min: Signal<String>) -> Self {
        self.data.min = Some(MaybeSignal::Dynamic(min));
        self
    }

    /// Sets the maximum value for the slider.
    pub fn max(mut self, max: impl ToString) -> Self {
        self.data.max = Some(MaybeSignal::Static(max.to_string()));
        self
    }

    /// Sets the maximum value for the slider to a signal.
    pub fn max_signal(mut self, max: Signal<String>) -> Self {
        self.data.max = Some(MaybeSignal::Dynamic(max));
        self
    }
}
