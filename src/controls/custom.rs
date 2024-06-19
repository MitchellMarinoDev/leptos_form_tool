use super::{BuilderCxFn, BuilderFn, ControlBuilder, ControlData};
use crate::{FormBuilder, FormToolData};

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a custom component and adds it to the form.
    pub fn custom<CC: ControlData, FDT: Clone + PartialEq + 'static>(
        mut self,
        control_data: CC,
        builder: impl BuilderFn<ControlBuilder<FD, CC, FDT>>,
    ) -> Self {
        let control_builder = ControlBuilder::new(control_data);
        let control = builder(control_builder);
        self.add_control(control);
        self
    }

    /// Builds a custom component using the form's context and adds it to the
    /// form.
    pub fn custom_cx<CC: ControlData, FDT: Clone + PartialEq + 'static>(
        mut self,
        control_data: CC,
        builder: impl BuilderCxFn<ControlBuilder<FD, CC, FDT>, FD::Context>,
    ) -> Self {
        let control_builder = ControlBuilder::new(control_data);
        let control = builder(control_builder, self.cx.clone());
        self.add_control(control);
        self
    }

    /// Builds a custom component, starting with the default
    /// CustomControlData, and adds it to the form.
    pub fn custom_default<CC: Default + ControlData, FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, CC, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a custom component, starting with the default
    /// CustomControlData using the form's context, and adds it to the form.
    pub fn custom_default_cx<CC: Default + ControlData, FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, CC, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}
