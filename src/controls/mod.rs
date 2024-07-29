use crate::{form::FormToolData, styles::FormStyle};
use leptos::{RwSignal, Signal, SignalSetter, View};
use std::{fmt::Display, rc::Rc, str::FromStr};

pub mod button;
pub mod checkbox;
pub mod custom;
pub mod group;
pub mod heading;
pub mod hidden;
pub mod output;
pub mod radio_buttons;
pub mod select;
pub mod slider;
pub mod spacer;
pub mod stepper;
pub mod submit;
pub mod text_area;
pub mod text_input;

pub trait BuilderFn<B>: Fn(B) -> B {}
pub trait BuilderCxFn<B, CX>: Fn(B, Rc<CX>) -> B {}
pub trait ValidationFn<FD: ?Sized>: Fn(&FD) -> Result<(), String> + 'static {}
pub trait ValidationCb: Fn() -> bool + 'static {}
pub trait ParseFn<CR, FDT>: Fn(CR) -> Result<FDT, String> + 'static {}
pub trait UnparseFn<CR, FDT>: Fn(FDT) -> CR + 'static {}
pub trait FieldGetter<FD, FDT>: Fn(FD) -> FDT + 'static {}
pub trait FieldSetter<FD, FDT>: Fn(&mut FD, FDT) + 'static {}
pub trait ShowWhenFn<FD: 'static, CX>: Fn(Signal<FD>, Rc<CX>) -> bool + 'static {}
pub trait RenderFn<FS, FD: 'static>:
    FnOnce(Rc<FS>, RwSignal<FD>) -> (View, Option<Box<dyn ValidationCb>>) + 'static
{
}

// implement the traits for all valid types
impl<B, T> BuilderFn<B> for T where T: Fn(B) -> B {}
impl<B, CX, T> BuilderCxFn<B, CX> for T where T: Fn(B, Rc<CX>) -> B {}
impl<FDT, T> ValidationFn<FDT> for T where T: Fn(&FDT) -> Result<(), String> + 'static {}
impl<T> ValidationCb for T where T: Fn() -> bool + 'static {}
impl<CR, FDT, F> ParseFn<CR, FDT> for F where F: Fn(CR) -> Result<FDT, String> + 'static {}
impl<CR, FDT, F> UnparseFn<CR, FDT> for F where F: Fn(FDT) -> CR + 'static {}
impl<FD, FDT, F> FieldGetter<FD, FDT> for F where F: Fn(FD) -> FDT + 'static {}
impl<FD, FDT, F> FieldSetter<FD, FDT> for F where F: Fn(&mut FD, FDT) + 'static {}
impl<FD: 'static, CX, F> ShowWhenFn<FD, CX> for F where F: Fn(Signal<FD>, Rc<CX>) -> bool + 'static {}
impl<FS, FD: 'static, F> RenderFn<FS, FD> for F where
    F: FnOnce(Rc<FS>, RwSignal<FD>) -> (View, Option<Box<dyn ValidationCb>>) + 'static
{
}

/// A trait for the data needed to render an read-only control.
pub trait VanityControlData: 'static {
    /// Builds the control, returning the [`View`] that was built.
    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Option<Signal<String>>,
    ) -> View;
}
pub trait GetterVanityControlData: VanityControlData {}

/// A trait for the data needed to render an interactive control.
pub trait ControlData: 'static {
    /// This is the data type returned by this control.
    type ReturnType: Clone;

    /// Builds the control, returning the [`View`] that was built.
    fn build_control<FS: FormStyle>(
        fs: &FS,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<Result<(), String>>,
    ) -> View;
}
pub trait ValidatedControlData: ControlData {}

/// The data needed to render a interactive control of type `C`.
pub struct ControlRenderData<FS: FormStyle + ?Sized, C: ?Sized> {
    pub styles: Vec<FS::StylingAttributes>,
    pub data: C,
}

/// The data needed to render a read-only control of type `C`.
pub struct VanityControlBuilder<FD: FormToolData, C: VanityControlData> {
    pub(crate) style_attributes: Vec<<FD::Style as FormStyle>::StylingAttributes>,
    pub(crate) data: C,
    pub(crate) getter: Option<Rc<dyn FieldGetter<FD, String>>>,
    pub(crate) show_when: Option<Box<dyn ShowWhenFn<FD, FD::Context>>>,
}

pub(crate) struct BuiltVanityControlData<FD: FormToolData, C: VanityControlData> {
    pub(crate) render_data: ControlRenderData<FD::Style, C>,
    pub(crate) getter: Option<Rc<dyn FieldGetter<FD, String>>>,
    pub(crate) show_when: Option<Box<dyn ShowWhenFn<FD, FD::Context>>>,
}

impl<FD: FormToolData, C: VanityControlData> VanityControlBuilder<FD, C> {
    /// Creates a new [`VanityControlBuilder`] with the given [`VanityControlData`].
    pub(crate) fn new(data: C) -> Self {
        VanityControlBuilder {
            data,
            style_attributes: Vec::new(),
            getter: None,
            show_when: None,
        }
    }

    /// Builds the builder into the data needed to render the control.
    pub(crate) fn build(self) -> BuiltVanityControlData<FD, C> {
        BuiltVanityControlData {
            render_data: ControlRenderData {
                data: self.data,
                styles: self.style_attributes,
            },
            getter: self.getter,
            show_when: self.show_when,
        }
    }

    /// Sets the function to decide when to render the control.
    ///
    /// Validations for components that are not shown DO NOT run.
    pub fn show_when(
        mut self,
        when: impl Fn(Signal<FD>, Rc<FD::Context>) -> bool + 'static,
    ) -> Self {
        self.show_when = Some(Box::new(when));
        self
    }

    /// Adds a styling attribute to this control.
    pub fn style(mut self, attribute: <FD::Style as FormStyle>::StylingAttributes) -> Self {
        self.style_attributes.push(attribute);
        self
    }
}

impl<FD: FormToolData, C: GetterVanityControlData> VanityControlBuilder<FD, C> {
    /// Sets the getter function.
    ///
    /// This function can get a string from the form data to be displayed
    ///
    /// Setting this getter field is NOT required for vanity controls like this one.
    pub fn getter(mut self, getter: impl FieldGetter<FD, String>) -> Self {
        self.getter = Some(Rc::new(getter));
        self
    }
}

/// The possibilities for errors when building a control.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub enum ControlBuildError {
    /// The getter field was not specified.
    MissingGetter,
    /// The setter field was not specified.
    MissingSetter,
    /// The parse function was not specified.
    MissingParseFn,
    /// The unparse function was not specified.
    MissingUnParseFn,
}
impl Display for ControlBuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            ControlBuildError::MissingGetter => "missing getter function",
            ControlBuildError::MissingSetter => "missing setter function",
            ControlBuildError::MissingParseFn => "missing parse function",
            ControlBuildError::MissingUnParseFn => "missing unparse function",
        };
        write!(f, "{}", message)
    }
}

/// The data returned fomr a control's build function.
pub(crate) struct BuiltControlData<FD: FormToolData, C: ControlData, FDT> {
    pub(crate) render_data: ControlRenderData<FD::Style, C>,
    pub(crate) getter: Rc<dyn FieldGetter<FD, FDT>>,
    pub(crate) setter: Rc<dyn FieldSetter<FD, FDT>>,
    pub(crate) parse_fn: Box<dyn ParseFn<C::ReturnType, FDT>>,
    pub(crate) unparse_fn: Box<dyn UnparseFn<C::ReturnType, FDT>>,
    pub(crate) validation_fn: Option<Rc<dyn ValidationFn<FD>>>,
    pub(crate) show_when: Option<Rc<dyn ShowWhenFn<FD, FD::Context>>>,
}

/// A builder for a interactive control.
pub struct ControlBuilder<FD: FormToolData, C: ControlData, FDT> {
    pub(crate) getter: Option<Rc<dyn FieldGetter<FD, FDT>>>,
    pub(crate) setter: Option<Rc<dyn FieldSetter<FD, FDT>>>,
    pub(crate) parse_fn: Option<Box<dyn ParseFn<C::ReturnType, FDT>>>,
    pub(crate) unparse_fn: Option<Box<dyn UnparseFn<C::ReturnType, FDT>>>,
    pub(crate) validation_fn: Option<Rc<dyn ValidationFn<FD>>>,
    pub(crate) style_attributes: Vec<<FD::Style as FormStyle>::StylingAttributes>,
    pub(crate) show_when: Option<Rc<dyn ShowWhenFn<FD, FD::Context>>>,
    pub data: C,
}

impl<FD: FormToolData, C: ControlData, FDT> ControlBuilder<FD, C, FDT> {
    /// Creates a new [`ControlBuilder`] with the given [`ControlData`].
    pub(crate) fn new(data: C) -> Self {
        ControlBuilder {
            data,
            getter: None,
            setter: None,
            parse_fn: None,
            unparse_fn: None,
            validation_fn: None,
            style_attributes: Vec::new(),
            show_when: None,
        }
    }

    /// Builds the builder into the data needed to render the control.
    ///
    /// This fails if a required field was not specified.
    pub(crate) fn build(self) -> Result<BuiltControlData<FD, C, FDT>, ControlBuildError> {
        let getter = match self.getter {
            Some(getter) => getter,
            None => return Err(ControlBuildError::MissingGetter),
        };
        let setter = match self.setter {
            Some(setter) => setter,
            None => return Err(ControlBuildError::MissingSetter),
        };
        let parse_fn = match self.parse_fn {
            Some(parse_fn) => parse_fn,
            None => return Err(ControlBuildError::MissingParseFn),
        };
        let unparse_fn = match self.unparse_fn {
            Some(unparse_fn) => unparse_fn,
            None => return Err(ControlBuildError::MissingUnParseFn),
        };

        Ok(BuiltControlData {
            render_data: ControlRenderData {
                data: self.data,
                styles: self.style_attributes,
            },
            getter,
            setter,
            parse_fn,
            unparse_fn,
            validation_fn: self.validation_fn,
            show_when: self.show_when,
        })
    }

    /// Sets the function to decide when to render the control.
    ///
    /// Validations for components that are not shown DO NOT run.
    pub fn show_when(
        mut self,
        when: impl Fn(Signal<FD>, Rc<FD::Context>) -> bool + 'static,
    ) -> Self {
        self.show_when = Some(Rc::new(when));
        self
    }

    /// Sets the getter function.
    ///
    /// This function should get the field from the form data
    /// for use in the form field.
    ///
    /// Setting this getter field is required.
    pub fn getter(mut self, getter: impl FieldGetter<FD, FDT>) -> Self {
        self.getter = Some(Rc::new(getter));
        self
    }

    /// Sets the setter function.
    ///
    /// This function should get the field from the form data
    /// for use in the form field.
    ///
    /// Setting this setter field is required.
    pub fn setter(mut self, setter: impl FieldSetter<FD, FDT>) -> Self {
        self.setter = Some(Rc::new(setter));
        self
    }

    /// Sets the parse functions to the ones given.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_custom(
        mut self,
        parse_fn: impl ParseFn<C::ReturnType, FDT>,
        unparse_fn: impl UnparseFn<C::ReturnType, FDT>,
    ) -> Self {
        self.parse_fn = Some(Box::new(parse_fn));
        self.unparse_fn = Some(Box::new(unparse_fn));
        self
    }

    /// Adds a styling attribute to this control.
    pub fn style(mut self, attribute: <FD::Style as FormStyle>::StylingAttributes) -> Self {
        self.style_attributes.push(attribute);
        self
    }
}

impl<FD, C, FDT> ControlBuilder<FD, C, FDT>
where
    FD: FormToolData,
    C: ControlData,
    FDT: TryFrom<<C as ControlData>::ReturnType>,
    <FDT as TryFrom<<C as ControlData>::ReturnType>>::Error: ToString,
    <C as ControlData>::ReturnType: From<FDT>,
{
    /// Sets the parse functions to use the [`TryFrom`] and [`From`] traits
    /// for parsing and unparsing respectively.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_from(mut self) -> Self {
        self.parse_fn = Some(Box::new(|control_return_value| {
            FDT::try_from(control_return_value).map_err(|e| e.to_string())
        }));
        self.unparse_fn = Some(Box::new(|field| {
            <C as ControlData>::ReturnType::from(field)
        }));
        self
    }
}

impl<FD, C, FDT> ControlBuilder<FD, C, FDT>
where
    FD: FormToolData,
    C: ControlData,
    FDT: TryFrom<<C as ControlData>::ReturnType>,
    <C as ControlData>::ReturnType: From<FDT>,
{
    /// Sets the parse functions to use the [`TryFrom`] and [`From`] traits
    /// for parsing and unparsing respectively, with a custom error message.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_from_msg(mut self, msg: impl ToString + 'static) -> Self {
        self.parse_fn = Some(Box::new(move |control_return_value| {
            FDT::try_from(control_return_value).map_err(|_| msg.to_string())
        }));
        self.unparse_fn = Some(Box::new(|field| {
            <C as ControlData>::ReturnType::from(field)
        }));
        self
    }
}

impl<FD, C, FDT> ControlBuilder<FD, C, FDT>
where
    FD: FormToolData,
    C: ControlData<ReturnType = String>,
    FDT: FromStr + ToString,
    <FDT as FromStr>::Err: ToString,
{
    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and traits
    /// for parsing and unparsing respectively. To trim the string before
    /// parsing, see [`parse_trimmed`](Self::parse_trimmed)().
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_string(mut self) -> Self {
        self.parse_fn = Some(Box::new(|control_return_value| {
            control_return_value
                .parse::<FDT>()
                .map_err(|e| e.to_string())
        }));
        self.unparse_fn = Some(Box::new(|field| field.to_string()));
        self
    }

    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and traits
    /// for parsing and unparsing respectively, similar to
    /// [`parse_string`](Self::parse_string)().
    /// However, this method trims the string before parsing.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_trimmed(mut self) -> Self {
        self.parse_fn = Some(Box::new(|control_return_value| {
            control_return_value
                .trim()
                .parse::<FDT>()
                .map_err(|e| e.to_string())
        }));
        self.unparse_fn = Some(Box::new(|field| field.to_string()));
        self
    }

    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and
    /// traits. Similar to [`parse_string`](Self::parse_string).
    ///
    /// The message passed in is the error message.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_string_msg(mut self, msg: impl ToString + 'static) -> Self {
        self.parse_fn = Some(Box::new(move |control_return_value| {
            control_return_value
                .parse::<FDT>()
                .map_err(|_| msg.to_string())
        }));
        self.unparse_fn = Some(Box::new(|field| field.to_string()));
        self
    }

    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and
    /// traits, trimming beforehand. Similar to
    /// [`parse_trimmed`](Self::parse_trimmed).
    ///
    /// The message passed in is the error message.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_trimmed_msg(mut self, msg: impl ToString + 'static) -> Self {
        self.parse_fn = Some(Box::new(move |control_return_value| {
            control_return_value
                .trim()
                .parse::<FDT>()
                .map_err(|_| msg.to_string())
        }));
        self.unparse_fn = Some(Box::new(|field| field.to_string()));
        self
    }
}

impl<FD, C, FDT> ControlBuilder<FD, C, Option<FDT>>
where
    FD: FormToolData,
    C: ControlData<ReturnType = String>,
    FDT: FromStr + ToString,
{
    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and traits
    /// on on optional value for parsing and unparsing respectively.
    /// If parsing fails, the `None` varient will be passed, otherwise, if
    /// parsing succeeds, `Some(value)` will be passed.
    ///
    /// To trim the string before parsing, see
    /// [`parse_optional_trimmed`](Self::parse_optional_trimmed)().
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_optional(mut self) -> Self {
        self.parse_fn = Some(Box::new(|control_return_value| {
            Ok(control_return_value.parse::<FDT>().ok())
        }));
        self.unparse_fn = Some(Box::new(|field| {
            field.map(|v| v.to_string()).unwrap_or_default()
        }));
        self
    }

    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and traits
    /// on on optional value for parsing and unparsing respectively, similar
    /// to [`parse_optional`](Self::parse_optional)().
    /// However, this method trims the string before parsing.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_optional_trimmed(mut self) -> Self {
        self.parse_fn = Some(Box::new(|control_return_value| {
            Ok(control_return_value.trim().parse::<FDT>().ok())
        }));
        self.unparse_fn = Some(Box::new(|field| {
            field.map(|v| v.to_string()).unwrap_or_default()
        }));
        self
    }
}

impl<FD, C, FDT> ControlBuilder<FD, C, FDT>
where
    FD: FormToolData,
    C: ControlData<ReturnType = String>,
    FDT: FromStr + ToString + Default,
{
    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and traits
    /// for parsing and unparsing respectively.
    /// If parsing fails, the default value will be used.
    ///
    /// To trim the string before parsing, see
    /// [`parse_trimmed_or_default`](Self::parse_trimmed_or_default)().
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_or_default(mut self) -> Self {
        self.parse_fn = Some(Box::new(|control_return_value| {
            Ok(control_return_value.parse::<FDT>().unwrap_or_default())
        }));
        self.unparse_fn = Some(Box::new(|field| field.to_string()));
        self
    }

    /// Sets the parse functions to use the [`FromStr`] [`ToString`] and traits
    /// on on optional value for parsing and unparsing respectively, similar
    /// to [`parse_or_default`](Self::parse_or_default)().
    /// However, this method trims the string before parsing.
    ///
    /// The parse and unparse functions define how to turn what the user
    /// types in the form into what is stored in the form data struct and
    /// vice versa.
    pub fn parse_trimmed_or_default(mut self) -> Self {
        self.parse_fn = Some(Box::new(|control_return_value| {
            Ok(control_return_value
                .trim()
                .parse::<FDT>()
                .unwrap_or_default())
        }));
        self.unparse_fn = Some(Box::new(|field| field.to_string()));
        self
    }
}

impl<FD: FormToolData, C: ValidatedControlData, FDT> ControlBuilder<FD, C, FDT> {
    /// Sets the validation function for this control
    ///
    /// This allows you to check if the parsed value is a valid value.
    ///
    /// You are given the entire [`FormToolData`] struct, but you should only
    /// validate the field you are creating. You can use the other fields in
    /// the struct as context.
    ///
    /// Ex. You have a month and a day field in a form. You use the month
    /// field to help ensure that the day is a valid day of that month.
    pub fn validation_fn(
        mut self,
        validation_fn: impl Fn(&FD) -> Result<(), String> + 'static,
    ) -> Self {
        self.validation_fn = Some(Rc::new(validation_fn));
        self
    }
}
