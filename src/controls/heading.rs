use super::{BuilderCxFn, BuilderFn, ControlRenderData, VanityControlBuilder, VanityControlData};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{MaybeSignal, Signal, View};
use std::rc::Rc;

/// Data used for the heading control.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HeadingData {
    pub title: MaybeSignal<String>,
}

impl VanityControlData for HeadingData {
    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        _value_getter: Option<leptos::prelude::Signal<String>>,
    ) -> View {
        fs.heading(control)
    }
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a heading and adds it to the form.
    pub fn heading(self, builder: impl BuilderFn<VanityControlBuilder<FD, HeadingData>>) -> Self {
        self.new_vanity(builder)
    }

    /// Builds a hehading using the form's context and adds it to the form.
    pub fn heading_cx(
        self,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, HeadingData>, FD::Context>,
    ) -> Self {
        self.new_vanity_cx(builder)
    }
}

impl<FD: FormToolData> VanityControlBuilder<FD, HeadingData> {
    /// Sets the title of this heading.
    pub fn title(mut self, title: impl ToString) -> Self {
        self.data.title = MaybeSignal::Static(title.to_string());
        self
    }

    /// Sets the title of this heading to a signal.
    pub fn title_signal(mut self, title: Signal<String>) -> Self {
        self.data.title = MaybeSignal::Dynamic(title);
        self
    }
}
