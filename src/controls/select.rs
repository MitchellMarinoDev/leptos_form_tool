use super::{
    BuilderCxFn, BuilderFn, ControlBuilder, ControlData, ControlRenderData, ValidatedControlData,
    ValidationState,
};
use crate::{form::FormToolData, form_builder::FormBuilder, styles::FormStyle};
use leptos::{IntoSignal, MaybeSignal, RwSignal, Signal, SignalGet, SignalSetter, View};
use std::rc::Rc;

type DynamicOptionsGetter<FD> = Rc<dyn Fn(RwSignal<FD>) -> Vec<(String, String)> + 'static>;
/// Data used for building the select control.
pub struct SelectBuildData<FD: FormToolData> {
    pub name: String,
    pub label: Option<String>,
    /// A derived signal for dynamic options for the select
    ///
    /// This is just a temp value for building, and should not be used
    /// directly
    dynamic_options: Option<DynamicOptionsGetter<FD>>,
    /// The options for the select.
    ///
    /// The first value is the string to display, the second is the value.
    pub options: MaybeSignal<Vec<(String, String)>>,
    /// The display text for the blank option, if there is one.
    pub blank_option: Option<String>,
}
impl<FD: FormToolData> Default for SelectBuildData<FD> {
    fn default() -> Self {
        SelectBuildData {
            name: String::default(),
            label: None,
            dynamic_options: None,
            options: MaybeSignal::default(),
            blank_option: None,
        }
    }
}
impl<FD: FormToolData> Clone for SelectBuildData<FD> {
    fn clone(&self) -> Self {
        SelectBuildData {
            name: self.name.clone(),
            label: self.label.clone(),
            dynamic_options: self.dynamic_options.clone(),
            options: self.options.clone(),
            blank_option: self.blank_option.clone(),
        }
    }
}

/// Data used for the select control.
#[derive(Default, Clone)]
pub struct SelectData {
    pub name: String,
    pub label: Option<String>,
    /// The options for the select.
    ///
    /// The first value is the string to display, the second is the value.
    pub options: MaybeSignal<Vec<(String, String)>>,
    /// The display text for the blank option, if there is one.
    pub blank_option: Option<String>,
}

impl<FD: FormToolData> ControlData<FD> for SelectBuildData<FD> {
    type ReturnType = String;

    fn render_control<FS: FormStyle>(
        fs: &FS,
        fd: RwSignal<FD>,
        control: Rc<ControlRenderData<FS, Self>>,
        value_getter: Signal<Self::ReturnType>,
        value_setter: SignalSetter<Self::ReturnType>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let options = control
            .data
            .dynamic_options
            .as_ref()
            .map(|d| {
                let d = d.clone();
                MaybeSignal::Dynamic((move || d(fd)).into_signal())
            })
            .unwrap_or(control.data.options.clone());

        let new_control = ControlRenderData {
            styles: control.styles.clone(),
            data: SelectData {
                name: control.data.name.clone(),
                label: control.data.label.clone(),
                options,
                blank_option: control.data.blank_option.clone(),
            },
        };
        let new_control = Rc::new(new_control);

        fs.select(new_control, value_getter, value_setter, validation_state)
    }
}
impl<FD: FormToolData> ValidatedControlData<FD> for SelectBuildData<FD> {}

impl<FD: FormToolData> FormBuilder<FD> {
    /// Builds a select control and adds it to the form.
    pub fn select<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderFn<ControlBuilder<FD, SelectBuildData<FD>, FDT>>,
    ) -> Self {
        self.new_control(builder)
    }

    /// Builds a select control using the form's context and adds it to the
    /// form.
    pub fn select_cx<FDT: Clone + PartialEq + 'static>(
        self,
        builder: impl BuilderCxFn<ControlBuilder<FD, SelectBuildData<FD>, FDT>, FD::Context>,
    ) -> Self {
        self.new_control_cx(builder)
    }
}

impl<FD: FormToolData, FDT> ControlBuilder<FD, SelectBuildData<FD>, FDT> {
    /// Sets the name of the radio button inputs.
    ///
    /// This is used for the html element's "name" attribute.
    /// In forms, the name attribute is the key that the data is sent
    /// with.
    pub fn named(mut self, control_name: impl ToString) -> Self {
        self.data.name = control_name.to_string();
        self
    }

    /// Sets the label for the select.
    pub fn labeled(mut self, label: impl ToString) -> Self {
        self.data.label = Some(label.to_string());
        self
    }

    /// Sets the options from the provided iterator.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options(mut self, options: impl Iterator<Item = impl ToString>) -> Self {
        // clear dynamic option
        self.data.dynamic_options = None;

        let options = options.map(|v| (v.to_string(), v.to_string())).collect();
        self.data.options = MaybeSignal::Static(options);
        self
    }

    /// Sets the options to the (display_string, value) pairs from the
    /// provided iterator.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options_valued(
        mut self,
        options: impl Iterator<Item = (impl ToString, impl ToString)>,
    ) -> Self {
        // clear dynamic option
        self.data.dynamic_options = None;

        let options = options
            .map(|(d, v)| (d.to_string(), v.to_string()))
            .collect();
        self.data.options = MaybeSignal::Static(options);
        self
    }

    /// Sets the options from the provided signal.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options_signal(mut self, options: Signal<Vec<String>>) -> Self {
        // clear dynamic option
        self.data.dynamic_options = None;

        let options = move || {
            options
                .get()
                .into_iter()
                .map(|v| (v.clone(), v))
                .collect::<Vec<_>>()
        };
        self.data.options = MaybeSignal::Dynamic(options.into_signal());
        self
    }

    /// Sets the options to the (display_string, value) pairs from the
    /// provided signal.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_options_valued_signal(mut self, options: Signal<Vec<(String, String)>>) -> Self {
        // clear dynamic option
        self.data.dynamic_options = None;

        self.data.options = MaybeSignal::Dynamic(options);
        self
    }

    /// Sets the options to the given derived signal.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_dynamic_options(
        mut self,
        derived_signal: impl Fn(RwSignal<FD>) -> Vec<String> + 'static,
    ) -> Self {
        let derived_signal = move |fd| {
            derived_signal(fd)
                .into_iter()
                .map(|v| (v.clone(), v))
                .collect::<Vec<_>>()
        };
        self.data.dynamic_options = Some(Rc::new(derived_signal));
        self
    }

    /// Sets the options to the (display_string, value) pairs from the
    /// provided derived signal.
    ///
    /// This will overwrite any pervious options setting.
    pub fn with_dynamic_options_valued(
        mut self,
        derived_signal: impl Fn(RwSignal<FD>) -> Vec<(String, String)> + 'static,
    ) -> Self {
        self.data.dynamic_options = Some(Rc::new(derived_signal));
        self
    }

    /// Adds a blank option as the first option for the select.
    pub fn with_blank_option(mut self) -> Self {
        self.data.blank_option = Some(String::new());
        self
    }

    /// Adds a blank option as the first option for the select,
    /// but sets the display string to the given value.
    pub fn with_blank_option_displayed(mut self, display: impl ToString) -> Self {
        self.data.blank_option = Some(display.to_string());
        self
    }
}
