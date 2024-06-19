use crate::{
    controls::{
        BuilderCxFn, BuilderFn, BuiltControlData, BuiltVanityControlData, ControlBuilder,
        ControlData, ControlRenderData, FieldSetter, ParseFn, RenderFn, ValidationCb, ValidationFn,
        VanityControlBuilder, VanityControlData,
    },
    form::{Form, FormToolData, FormValidator},
    styles::FormStyle,
};
use leptos::{
    server_fn::{client::Client, codec::PostUrl, request::ClientReq, ServerFn},
    *,
};
use leptos_router::{ActionForm, Form};
use serde::de::DeserializeOwned;
use std::rc::Rc;
use web_sys::{FormData, SubmitEvent};

/// A builder for laying out forms.
///
/// This builder allows you to specify what components should make up the form.
pub struct FormBuilder<FD: FormToolData> {
    pub(crate) cx: Rc<FD::Context>,
    /// The list of [`ValidationFn`]s.
    pub(crate) validations: Vec<Rc<dyn ValidationFn<FD>>>,
    /// The list of functions that will render the form.
    pub(crate) render_fns: Vec<Box<dyn RenderFn<FD::Style, FD>>>,
    /// The list of styling attributes applied on the form level.
    pub(crate) styles: Vec<<FD::Style as FormStyle>::StylingAttributes>,
}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Creates a new [`FormBuilder`]
    pub(crate) fn new(cx: FD::Context) -> Self {
        FormBuilder {
            cx: Rc::new(cx),
            validations: Vec::new(),
            render_fns: Vec::new(),
            styles: Vec::new(),
        }
    }

    /// Creates a new [`FormBuilder`] with the given Rc'ed context, for
    //// building a form group.
    pub(crate) fn new_group(cx: Rc<FD::Context>) -> Self {
        FormBuilder {
            cx,
            validations: Vec::new(),
            render_fns: Vec::new(),
            styles: Vec::new(),
        }
    }

    /// Adds a styling attribute to the entire form.
    pub fn style(mut self, style: <FD::Style as FormStyle>::StylingAttributes) -> Self {
        self.styles.push(style);
        self
    }

    /// Adds a new vanity control to the form.
    pub(crate) fn new_vanity<C: VanityControlData + Default>(
        mut self,
        builder: impl BuilderFn<VanityControlBuilder<FD, C>>,
    ) -> Self {
        let vanity_builder = VanityControlBuilder::new(C::default());
        let control = builder(vanity_builder);
        self.add_vanity(control);
        self
    }

    /// Adds a new vanity control to the form using the form's context.
    pub(crate) fn new_vanity_cx<C: VanityControlData + Default>(
        mut self,
        builder: impl BuilderCxFn<VanityControlBuilder<FD, C>, FD::Context>,
    ) -> Self {
        let vanity_builder = VanityControlBuilder::new(C::default());
        let control = builder(vanity_builder, self.cx.clone());
        self.add_vanity(control);
        self
    }

    /// Adds a new control to the form using the form's context.
    pub(crate) fn new_control<C: ControlData + Default, FDT: Clone + PartialEq + 'static>(
        mut self,
        builder: impl BuilderFn<ControlBuilder<FD, C, FDT>>,
    ) -> Self {
        let control_builder = ControlBuilder::new(C::default());
        let control = builder(control_builder);
        self.add_control(control);
        self
    }

    /// Adds a new control to the form using the form's context.
    pub(crate) fn new_control_cx<C: ControlData + Default, FDT: Clone + PartialEq + 'static>(
        mut self,
        builder: impl BuilderCxFn<ControlBuilder<FD, C, FDT>, FD::Context>,
    ) -> Self {
        let control_builder = ControlBuilder::new(C::default());
        let control = builder(control_builder, self.cx.clone());
        self.add_control(control);
        self
    }

    /// Adds a vanity control to the form.
    pub(crate) fn add_vanity<C: VanityControlData>(
        &mut self,
        vanity_control: VanityControlBuilder<FD, C>,
    ) {
        let BuiltVanityControlData {
            render_data,
            getter,
            show_when,
        } = vanity_control.build();

        let cx = self.cx.clone();
        let render_fn = move |fs: Rc<FD::Style>, fd: RwSignal<FD>| {
            let render_data = Rc::new(render_data);
            let value_getter = getter.map(|getter| (move || getter(fd.get())).into_signal());
            let view =
                move || VanityControlData::build_control(&*fs, render_data.clone(), value_getter);
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
    }

    /// Adds a control to the form.
    pub(crate) fn add_control<C: ControlData, FDT: Clone + PartialEq + 'static>(
        &mut self,
        control: ControlBuilder<FD, C, FDT>,
    ) {
        let built_control_data = match control.build() {
            Ok(c) => c,
            Err(e) => {
                let item_name = std::any::type_name::<C>()
                    .rsplit("::")
                    .next()
                    .expect("split to have at least 1 element");
                panic!("Invalid Component ({}): {}", item_name, e)
            }
        };

        if let Some(validation_fn) = built_control_data.validation_fn.clone() {
            let validation_fn = if let Some(show_when) = built_control_data.show_when.clone() {
                // we want the validation function to always succeed for hidden components
                // thus, we need to modify the validation function
                let cx = self.cx.clone();
                let new_validation_fn = move |fd: &FD| {
                    let (fd_signal, _) = create_signal(fd.clone());
                    if !show_when(fd_signal.into(), cx.clone()) {
                        return Ok(());
                    }
                    validation_fn(fd)
                };
                Rc::new(new_validation_fn)
            } else {
                validation_fn
            };

            self.validations.push(validation_fn);
        }

        let cx = self.cx.clone();
        let render_fn = move |fs: Rc<FD::Style>, fd: RwSignal<FD>| {
            let (view, cb) = Self::build_control_view(fd, fs, built_control_data, cx);
            (view, Some(cb))
        };

        self.render_fns.push(Box::new(render_fn));
    }

    /// Helper for building all the functions and everything needed to render
    /// the view.
    fn build_control_view<C: ControlData, FDT: 'static>(
        fd: RwSignal<FD>,
        fs: Rc<FD::Style>,
        control_data: BuiltControlData<FD, C, FDT>,
        cx: Rc<FD::Context>,
    ) -> (View, Box<dyn ValidationCb>) {
        let BuiltControlData {
            render_data,
            getter,
            setter,
            parse_fn,
            unparse_fn,
            validation_fn,
            show_when,
        } = control_data;

        let render_data = Rc::new(render_data);
        let (validation_signal, validation_signal_set) = create_signal(Ok(()));
        let validation_fn_clone = validation_fn.clone();
        let value_getter = move || {
            let fd = fd.get();

            // rerun validation if it is failing
            if validation_signal.get_untracked().is_err() {
                if let Some(ref validation_fn) = validation_fn_clone {
                    let validation_result = validation_fn(&fd);
                    // if validation succeeds this time, resolve the validation error
                    if validation_result.is_ok() {
                        validation_signal_set.set(Ok(()));
                    }
                }
            }
            unparse_fn(getter(fd))
        };
        let value_getter = value_getter.into_signal();

        let cloned_show_when = show_when.clone();
        let cloned_cx = cx.clone();
        let validation_cb = move || {
            // first check if the validation signal is an error so that we
            // can fail on parsing issues too
            if let Some(Err(_)) = validation_signal.try_get_untracked() {
                return false;
            }

            // validation for non-visible fields always succeeds
            if let Some(ref show_when) = cloned_show_when {
                if !show_when(fd.into(), cloned_cx.clone()) {
                    return true;
                }
            }

            // run the validation function on the value now
            let validation_fn = match validation_fn {
                Some(ref v) => v,
                None => return true, // No validation function so validation passes
            };

            let data = fd.get_untracked();
            let validation_result = validation_fn(&data);
            let succeeded = validation_result.is_ok();
            validation_signal_set.set(validation_result);
            succeeded
        };
        let validation_cb = Box::new(validation_cb);

        let value_setter = Self::create_value_setter(
            validation_cb.clone(),
            validation_signal_set,
            parse_fn,
            setter,
            fd,
        );

        let view = move || {
            C::build_control(
                &*fs,
                render_data.clone(),
                value_getter,
                value_setter.clone(),
                validation_signal.into(),
            )
        };
        let view = match show_when {
            Some(when) => {
                let when = move || when(fd.into(), cx.clone());
                view! { <Show when=when>{view.clone()}</Show> }
            }
            None => view(),
        };
        (view, validation_cb)
    }

    /// Helper for creating a setter function.
    fn create_value_setter<CRT: 'static, FDT: 'static>(
        validation_cb: Box<dyn Fn() -> bool + 'static>,
        validation_signal_set: WriteSignal<Result<(), String>>,
        parse_fn: Box<dyn ParseFn<CRT, FDT>>,
        setter: Rc<dyn FieldSetter<FD, FDT>>,
        fd: RwSignal<FD>,
    ) -> Rc<dyn Fn(CRT) + 'static> {
        let value_setter = move |value| {
            let parsed = match parse_fn(value) {
                Ok(p) => {
                    validation_signal_set.set(Ok(()));
                    p
                }
                Err(e) => {
                    validation_signal_set.set(Err(e));
                    return;
                }
            };

            // parse succeeded, update value and validate
            fd.update(|data| {
                setter(data, parsed);
            });

            // run validation
            (validation_cb)();
        };
        Rc::new(value_setter)
    }

    /// Builds the direct send version of the form.
    pub(crate) fn build_form<ServFn>(
        self,
        action: Action<ServFn, Result<ServFn::Output, ServerFnError<ServFn::Error>>>,
        fd: FD,
        fs: FD::Style,
    ) -> Form<FD>
    where
        ServFn: DeserializeOwned + ServerFn<InputEncoding = PostUrl> + 'static,
        <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
            From<FormData>,
        ServFn: From<FD>,
    {
        let fd = create_rw_signal(fd);
        let fs = Rc::new(fs);

        let (views, validation_cbs): (Vec<_>, Vec<_>) = self
            .render_fns
            .into_iter()
            .map(|r_fn| r_fn(fs.clone(), fd))
            .unzip();

        let elements = fs.form_frame(ControlRenderData {
            data: views.into_view(),
            styles: self.styles,
        });

        let on_submit = move |ev: SubmitEvent| {
            let mut failed = false;
            for validation in validation_cbs.iter().flatten() {
                if !validation() {
                    failed = true;
                }
            }
            if failed {
                ev.prevent_default();
                return;
            }

            ev.prevent_default();
            let server_fn = ServFn::from(fd.get_untracked());
            action.dispatch(server_fn);
        };

        let view = view! {
            <ActionForm action=action on:submit=on_submit>
                {elements}
            </ActionForm>
        };

        Form {
            fd,
            validations: self.validations,
            view,
        }
    }

    /// Builds the action form version of the form.
    pub(crate) fn build_action_form<ServFn>(
        self,
        action: Action<ServFn, Result<ServFn::Output, ServerFnError<ServFn::Error>>>,
        fd: FD,
        fs: FD::Style,
    ) -> Form<FD>
    where
        ServFn: DeserializeOwned + ServerFn<InputEncoding = PostUrl> + 'static,
        <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
            From<FormData>,
    {
        let fd = create_rw_signal(fd);
        let fs = Rc::new(fs);

        let (views, validation_cbs): (Vec<_>, Vec<_>) = self
            .render_fns
            .into_iter()
            .map(|r_fn| r_fn(fs.clone(), fd))
            .unzip();

        let elements = fs.form_frame(ControlRenderData {
            data: views.into_view(),
            styles: self.styles,
        });

        let on_submit = move |ev: SubmitEvent| {
            let mut failed = false;
            for validation in validation_cbs.iter().flatten() {
                if !validation() {
                    failed = true;
                }
            }
            if failed {
                ev.prevent_default();
            }
        };

        let view = view! {
            <ActionForm action=action on:submit=on_submit>
                {elements}
            </ActionForm>
        };

        Form {
            fd,
            validations: self.validations,
            view,
        }
    }

    /// builds the plain form version of the form.
    pub(crate) fn build_plain_form(self, url: String, fd: FD, fs: FD::Style) -> Form<FD> {
        let fd = create_rw_signal(fd);
        let fs = Rc::new(fs);

        let (views, validation_cbs): (Vec<_>, Vec<_>) = self
            .render_fns
            .into_iter()
            .map(|r_fn| r_fn(fs.clone(), fd))
            .unzip();

        let elements = fs.form_frame(ControlRenderData {
            data: views.into_view(),
            styles: self.styles,
        });

        let on_submit = move |ev: SubmitEvent| {
            let mut failed = false;
            for validation in validation_cbs.iter().flatten() {
                if !validation() {
                    failed = true;
                }
            }
            if failed {
                ev.prevent_default();
            }
        };

        let view = view! {
            <Form action=url on:submit=on_submit>
                {elements}
            </Form>
        };

        Form {
            fd,
            validations: self.validations,
            view,
        }
    }

    /// Creates a [`FormValidator`] from this builder.
    pub(crate) fn validator(&self) -> FormValidator<FD> {
        FormValidator {
            validations: self.validations.clone(),
        }
    }
}
