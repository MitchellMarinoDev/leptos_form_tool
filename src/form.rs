use crate::{controls::ValidationFn, form_builder::FormBuilder, styles::FormStyle};
use leptos::{
    server_fn::{client::Client, codec::PostUrl, request::ClientReq, ServerFn},
    *,
};
use serde::de::DeserializeOwned;
use std::rc::Rc;
use web_sys::FormData;

/// A type that can be used to validate the form data.
///
/// This can be useful to use the same validation logic on the front
/// end and backend without duplicating the logic.
pub struct FormValidator<FD> {
    pub(crate) validations: Vec<Rc<dyn ValidationFn<FD>>>,
}

impl<FD: FormToolData> FormValidator<FD> {
    /// Validates the given form data.
    ///
    /// This runs all the validation functions for all the fields
    /// in the form. The first falure to occur (if any) will be returned.
    pub fn validate(&self, form_data: &FD) -> Result<(), String> {
        for v in self.validations.iter() {
            (*v)(form_data)?;
        }
        Ok(())
    }
}

/// A constructed, rendered form object.
///
/// With this, you can render the form, get the form data, or get
/// a validator for the data.
#[derive(Clone)]
pub struct Form<FD: FormToolData> {
    /// The form data signal.
    pub fd: RwSignal<FD>,
    /// The list of validations
    pub(crate) validations: Vec<Rc<dyn ValidationFn<FD>>>,
    pub(crate) view: View,
}

impl<FD: FormToolData> Form<FD> {
    /// Gets the [`FormValidator`] for this form.
    pub fn validator(&self) -> FormValidator<FD> {
        FormValidator {
            validations: self.validations.clone(),
        }
    }

    /// Validates the [`FormToolData`], returning the result.
    pub fn validate(&self) -> Result<(), String> {
        let validator = self.validator();
        validator.validate(&self.fd.get_untracked())
    }

    /// Gets the view associated with this [`Form`].
    pub fn view(&self) -> View {
        self.view.clone()
    }

    /// Splits this [`Form`] into it's parts.
    pub fn to_parts(self) -> (RwSignal<FD>, FormValidator<FD>, View) {
        (
            self.fd,
            FormValidator {
                validations: self.validations,
            },
            self.view,
        )
    }
}

impl<FD: FormToolData> IntoView for Form<FD> {
    fn into_view(self) -> View {
        self.view()
    }
}

/// A trait allowing a form to be built around its containing data.
///
/// This trait defines a function that can be used to build all the data
/// needed to physically lay out a form, and how that data should be parsed
/// and validated.
pub trait FormToolData: Clone + 'static {
    /// The style that this form uses.
    type Style: FormStyle;
    /// The context that this form is rendered in.
    ///
    /// This will need to be provided when building a form or a validator.
    /// Therefore, you will need to be able to replicate this context
    /// on the client for rendering and the server for validating.
    type Context: 'static;

    /// Defines how the form should be laid out and how the data should be
    /// parsed and validated.
    ///
    /// To construct a [`From`] object, use one of the `get_form` methods.
    ///
    /// Uses the given form builder to specify what fields should be present
    /// in the form, what properties those fields should have, and how that
    /// data should be parsed and checked.
    fn build_form(fb: FormBuilder<Self>) -> FormBuilder<Self>;

    /// Constructs a [`Form`] for this [`FormToolData`] type.
    ///
    /// This renders the form as a enhanced
    /// [`ActionForm`](leptos_router::ActionForm) that sends the form data
    /// directly by calling the server function.
    ///
    /// By doing this, we avoid doing the
    /// [`FromFormData`](leptos_router::FromFormData)
    /// conversion. However, to support
    /// [Progressive Enhancement](https://book.leptos.dev/progressive_enhancement/index.html),
    /// you should name the form elements to work with a plain ActionForm
    /// anyway. If progresssive enhancement is not important to you, you may
    /// freely use this version.
    ///
    /// For the other ways to construct a [`Form`], see:
    /// - [`get_action_form`](Self::get_action_form)
    /// - [`get_plain_form`](Self::get_plain_form)
    fn get_form<ServFn>(
        self,
        action: Action<ServFn, Result<ServFn::Output, ServerFnError<ServFn::Error>>>,
        style: Self::Style,
        context: Self::Context,
    ) -> Form<Self>
    where
        ServFn: DeserializeOwned + ServerFn<InputEncoding = PostUrl> + 'static,
        <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
            From<FormData>,
        ServFn: From<Self>,
    {
        let builder = FormBuilder::new(context);
        let builder = Self::build_form(builder);
        builder.build_form(action, self, style)
    }

    /// Constructs a [`Form`] for this [`FormToolData`] type.
    ///
    /// This renders the form as a the leptos_router
    /// [`ActionForm`](leptos_router::ActionForm)
    /// component.
    ///
    /// For the other ways to construct a [`Form`], see:
    /// - [`get_form`](Self::get_form)
    /// - [`get_action_form`](Self::get_action_form)
    fn get_action_form<ServFn>(
        self,
        action: Action<ServFn, Result<ServFn::Output, ServerFnError<ServFn::Error>>>,
        style: Self::Style,
        context: Self::Context,
    ) -> Form<Self>
    where
        ServFn: DeserializeOwned + ServerFn<InputEncoding = PostUrl> + 'static,
        <<ServFn::Client as Client<ServFn::Error>>::Request as ClientReq<ServFn::Error>>::FormData:
            From<FormData>,
    {
        let builder = FormBuilder::new(context);
        let builder = Self::build_form(builder);
        builder.build_action_form(action, self, style)
    }

    /// Constructs a [`Form`] for this [`FormToolData`] type.
    ///
    /// This renders the form as a the leptos_router
    /// [`Form`](leptos_router::Form)
    /// component.
    ///
    /// For the other ways to construct a [`Form`], see:
    /// - [`get_form`](Self::get_form)
    /// - [`get_action_form`](Self::get_action_form)
    fn get_plain_form(
        self,
        action: impl ToString,
        style: Self::Style,
        context: Self::Context,
    ) -> Form<Self> {
        let builder = FormBuilder::new(context);
        let builder = Self::build_form(builder);
        builder.build_plain_form(action.to_string(), self, style)
    }

    /// Gets a [`FormValidator`] for this [`FormToolData`].
    ///
    /// This doesn't render the view, but just collects all the validation
    /// Functions from building the form. That means it can be called on the
    /// Server and no rendering will be done.
    ///
    /// However, the code to render the views are not configured out, it
    /// simply doesn't run, so the view needs to compile even on the server.
    fn get_validator(context: Self::Context) -> FormValidator<Self> {
        let builder = FormBuilder::new(context);
        let builder = Self::build_form(builder);
        builder.validator()
    }

    /// Validates this [`FormToolData`] struct.
    ///
    /// This is shorthand for creating a validator with
    /// [`get_validator`](Self::get_validator)()
    /// and then calling `validator.validate(&self, context)`.
    fn validate(&self, context: Self::Context) -> Result<(), String> {
        let validator = Self::get_validator(context);
        validator.validate(self)
    }
}
