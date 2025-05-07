use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{prelude::*, reactive::wrappers::write::SignalSetter};
use std::rc::Rc;

/// Data used for the checkbox control.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct CheckboxData {
    pub name: String,
    pub label: Option<String>,
}

impl<FD: FormToolData> ControlData<FD> for CheckboxData {
    type ReturnType = bool;

    fn render_control<FS: FormStyle>(
        fs: &FS,
        _fd: RwSignal<FD>,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        _validation_state: Signal<ValidationState>,
    ) -> AnyView {
        fs.checkbox(control, value_getter, value_setter)
    }
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a checkbox and adds it to the form.
    pub fn checkbox<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, CheckboxData, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a checkbox using the form's context and adds it to the form.
    pub fn checkbox_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, CheckboxData, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, CheckboxData, FDT> {
    /// Sets the name of the checkbox.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    ///
    /// For checkbox controls, the value "checked" is sent or no key value
    /// pair is sent.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the text of the checkbox's label.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }
}
