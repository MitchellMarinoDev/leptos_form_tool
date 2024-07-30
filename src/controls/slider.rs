use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{MaybeSignal, Signal, SignalSetter, View};
use std::rc::Rc;

/// Data used for the slider control.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SliderData {
    pub name: String,
    pub label: Option<String>,
    pub min: MaybeSignal<i32>,
    pub max: MaybeSignal<i32>,
}

impl Default for SliderData {
    fn default() -> Self {
        SliderData {
            name: String::new(),
            label: None,
            min: MaybeSignal::Static(0),
            max: MaybeSignal::Static(1),
        }
    }
}

impl ControlData for SliderData {
    type ReturnType = i32;

    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<ValidationState>,
    ) -> View {
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

    /// Sets the minimum value for the slider.
    pub fn min(mut self, min: i32) -> Self {
        self.data.min = MaybeSignal::Static(min);
        self
    }

    /// Sets the minimum value for the slider to a signal.
    pub fn min_signal(mut self, min: Signal<i32>) -> Self {
        self.data.min = MaybeSignal::Dynamic(min);
        self
    }

    /// Sets the maximum value for the slider.
    pub fn max(mut self, max: i32) -> Self {
        self.data.max = MaybeSignal::Static(max);
        self
    }

    /// Sets the maximum value for the slider to a signal.
    pub fn max_signal(mut self, max: Signal<i32>) -> Self {
        self.data.max = MaybeSignal::Dynamic(max);
        self
    }
}
