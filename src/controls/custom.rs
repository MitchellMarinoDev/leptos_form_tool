use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, VanityControlBuilder, VanityControlData,
};
use crate::{FormBuilder, FormToolData};
use leptos::{RwSignal, View};
use std::rc::Rc;

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

    /// Builds a custom vanity component and adds it to the form.
    pub fn custom_vanity<CC: VanityControlData>(
        mut self,
        control_data: CC,
        builder: impl BuilderFn<VanityControlBuilder<FD, CC>>,
    ) -> Self {
        let control_builder = VanityControlBuilder::new(control_data);
        let control = builder(control_builder);
        self.add_vanity(control);
        self
    }

    /// Builds a custom vanity component using the form's context and adds it
    /// to the form.
    pub fn custom_vanity_cx<CC: VanityControlData>(
        mut self,
        control_data: CC,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, CC>, FD::Context>,
    ) -> Self {
        let control_builder = VanityControlBuilder::new(control_data);
        let control = builder(control_builder, self.cx.clone());
        self.add_vanity(control);
        self
    }

    /// Add a raw view to the form.
    ///
    /// This gives you access to everything you might need when rendering a
    /// read-only view.
    ///
    /// This can be helpful for rendering a custom visual component, but is
    /// often times not needed.
    ///
    /// This method does not give you any access to the validation system
    /// so you should not add controls to the form with this method.
    /// consider defining a custom component for this purpose.
    pub fn raw_view(
        mut self,
        render_fn: impl Fn(Rc<FD::Style>, RwSignal<FD>, Rc<FD::Context>) -> View + 'static,
    ) -> Self {
        let cx = self.cx.clone();
        let render_fn = move |fs: Rc<FD::Style>, fd: RwSignal<FD>| {
            let view = render_fn(fs, fd, cx);
            (view, None)
        };

        self.render_fns.push(Box::new(render_fn));
        self
    }
}
