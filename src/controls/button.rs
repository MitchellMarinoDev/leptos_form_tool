use super::{BuilderCxFn, BuilderFn, ControlRenderData, ShowWhenFn};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::view;
use leptos::RwSignal;
use leptos::Show;
use leptos::Signal;
use std::rc::Rc;
use web_sys::MouseEvent;

type ButtonAction<FD> = dyn Fn(MouseEvent, RwSignal<FD>) + 'static;

/// Data used for the button control.
pub struct ButtonData<FD: FormToolData> {
    pub text: String,
    pub action: Option<Rc<ButtonAction<FD>>>,
}
impl<FD: FormToolData> Default for ButtonData<FD> {
    fn default() -> Self {
        ButtonData {
            text: String::default(),
            action: None,
        }
    }
}
impl<FD: FormToolData> Clone for ButtonData<FD> {
    fn clone(&self) -> Self {
        ButtonData {
            text: self.text.clone(),
            action: self.action.clone(),
        }
    }
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a button and adds it to the form.
    pub fn button(self, builder: impl BuilderFn<ButtonBuilder<FD>>) -> Self {
        let button_builder = ButtonBuilder::new();
        let control = builder(button_builder);
        self.button_helper(control)
    }

    /// Builds a button using the form's context and adds it to the form.
    pub fn button_cx(self, builder: impl BuilderCxFn<ButtonBuilder<FD>, FD::Context>) -> Self {
        let button_builder = ButtonBuilder::new();
        let control = builder(button_builder, self.cx.clone());
        self.button_helper(control)
    }

    /// The common functionality for adding a button.
    fn button_helper(mut self, control: ButtonBuilder<FD>) -> Self {
        let render_data = ControlRenderData {
            data: control.data,
            styles: control.styles,
        };
        let show_when = control.show_when;

        let cx = self.cx.clone();
        let render_fn = move |fs: Rc<FD::Style>, fd: RwSignal<FD>| {
            let render_data = Rc::new(render_data);

            let view = move || fs.clone().button(render_data.clone(), fd);
            let view = match show_when {
                Some(when) => {
                    let when = move || when(fd.into(), cx.clone());
                    view! { <Show when=when>{view.clone()}</Show> }
                }
                None => view(),
            };
            (view, None)
        };
        self.render_fns.push(Box::new(render_fn));

        self
    }
}

/// The struct that allows you to specify the attributes of the button.
pub struct ButtonBuilder<FD: FormToolData> {
    pub(crate) styles: Vec<<FD::Style as FormStyle>::StylingAttributes>,
    pub(crate) data: ButtonData<FD>,
    pub(crate) show_when: Option<Box<dyn ShowWhenFn<FD, FD::Context>>>,
}

impl<FD: FormToolData> ButtonBuilder<FD> {
    /// Creates a new [`ButtonBuilder`].
    fn new() -> Self {
        ButtonBuilder {
            styles: Vec::default(),
            data: ButtonData::default(),
            show_when: None,
        }
    }

    /// Sets the function to decide when to render the control.
    pub fn show_when(
        mut self,
        when: impl Fn(Signal<FD>, Rc<FD::Context>) -> bool + 'static,
    ) -> Self {
        self.show_when = Some(Box::new(when));
        self
    }

    /// Adds a styling attribute to the button.
    pub fn style(mut self, style: <FD::Style as FormStyle>::StylingAttributes) -> Self {
        self.styles.push(style);
        self
    }

    /// Sets the text of the button.
    pub fn text(mut self, text: impl ToString) -> Self {
        self.data.text = text.to_string();
        self
    }

    /// Sets the action that is preformed when the button is clicked.
    pub fn action(mut self, action: impl Fn(MouseEvent, RwSignal<FD>) + 'static) -> Self {
        self.data.action = Some(Rc::new(action));
        self
    }
}
