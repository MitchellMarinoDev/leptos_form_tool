use super::{
    BuilderCxFn, BuilderFn, ControlRenderData, GetterVanityControlData, VanityControlBuilder,
    VanityControlData,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{prelude::Signal, View};
use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum HeadingLevel {
    #[default]
    H1,
    H2,
    H3,
    H4,
}

/// Data used for the heading control.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct HeadingData {
    pub level: HeadingLevel,
}

impl<FD: FormToolData> VanityControlData<FD> for HeadingData {
    fn render_control<FS: FormStyle>(
        fs: &FS,
        _fd: leptos::prelude::RwSignal<FD>,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        fs.heading(control, value_getter)
    }
}
impl<FD: FormToolData> GetterVanityControlData<FD> for HeadingData {}

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
        let title = title.to_string();
        self.getter = Some(Rc::new(move |_| title.clone()));
        self
    }

    /// Sets this title to be a `h1`.
    pub fn h1(mut self) -> Self {
        self.data.level = HeadingLevel::H1;
        self
    }

    /// Sets this title to be a `h2`.
    pub fn h2(mut self) -> Self {
        self.data.level = HeadingLevel::H2;
        self
    }

    /// Sets this title to be a `h3`.
    pub fn h3(mut self) -> Self {
        self.data.level = HeadingLevel::H3;
        self
    }

    /// Sets this title to be a `h4`.
    pub fn h4(mut self) -> Self {
        self.data.level = HeadingLevel::H4;
        self
    }
}
