use super::{BuilderCxFn, BuilderFn, ControlRenderData, VanityControlBuilder};
use super::{GetterVanityControlData, VanityControlData};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::prelude::{AnyView, RwSignal, Signal};
use std::rc::Rc;
use web_sys::MouseEvent;

type ButtonAction<FD> = dyn Fn(MouseEvent, RwSignal<FD>) + 'static;

/// Data used for the building button control.
pub struct ButtonBuildData<FD: FormToolData> {
    pub action: Option<Rc<ButtonAction<FD>>>,
}
impl<FD: FormToolData> Default for ButtonBuildData<FD> {
    fn default() -> Self {
        ButtonBuildData { action: None }
    }
}
impl<FD: FormToolData> Clone for ButtonBuildData<FD> {
    fn clone(&self) -> Self {
        ButtonBuildData {
            action: self.action.clone(),
        }
    }
}

/// Data used for the button control.
pub struct ButtonData {
    pub action: Option<Rc<dyn Fn(MouseEvent)>>,
}

impl<FD: FormToolData> VanityControlData<FD> for ButtonBuildData<FD> {
    fn render_control<FS: FormStyle>(
        fs: &FS,
        fd: RwSignal<FD>,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Option<Signal<String>>,
    ) -> AnyView {
        let action = control.data.action.as_ref().map(|a| {
            let a = a.clone();
            let action = move |ev: MouseEvent| a(ev, fd);
            Rc::new(action) as Rc<dyn Fn(MouseEvent)>
        });

        let new_control = ControlRenderData {
            styles: control.styles.clone(),
            data: ButtonData { action },
        };
        let new_control = Rc::new(new_control);
        fs.button(new_control, value_getter)
    }
}
impl<FD: FormToolData> GetterVanityControlData<FD> for ButtonBuildData<FD> {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a button and adds it to the form.
    pub fn button(
        self,
        builder: impl BuilderFn<VanityControlBuilder<FD, ButtonBuildData<FD>>>,
    ) -> Self {
        self.new_vanity(builder)
    }

    /// Builds a button using the form's context and adds it to the form.
    pub fn button_cx(
        self,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, ButtonBuildData<FD>>, FD::Context>,
    ) -> Self {
        self.new_vanity_cx(builder)
    }
}

impl<FD: FormToolData> VanityControlBuilder<FD, ButtonBuildData<FD>> {
    /// Sets the text of the button to a static string.
    ///
    /// For dynamic button text, use the `getter` method.
    pub fn text(mut self, text: impl ToString) -> Self {
        let text = text.to_string();
        self.getter = Some(Rc::new(move |_| text.clone()));
        self
    }

    /// Sets the action that is preformed when the button is clicked.
    pub fn action(mut self, action: impl Fn(MouseEvent, RwSignal<FD>) + 'static) -> Self {
        self.data.action = Some(Rc::new(action));
        self
    }
}
