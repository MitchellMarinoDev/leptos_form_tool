mod grid_form;

use crate::{
    controls::{
        button::ButtonData, checkbox::CheckboxData, heading::HeadingData, hidden::HiddenData,
        output::OutputData, radio_buttons::RadioButtonsData, select::SelectData,
        slider::SliderData, spacer::SpacerData, stepper::StepperData, submit::SubmitData,
        text_area::TextAreaData, text_input::TextInputData, ControlData, ControlRenderData,
    },
    FormToolData,
};
use leptos::{RwSignal, Signal, View};
use std::rc::Rc;

pub use grid_form::{GFStyleAttr, GridFormStyle};

/// Defines a way to style a form.
///
/// Provides methods for rendering all the controls.
/// This provider is in charge of figuring out what html elements should be
/// rendered and how they should be styled.
pub trait FormStyle: 'static {
    /// The type of styling attributes that this [`FormStyle`] takes.
    ///
    /// These styling attributes can be applied to any of the controls.
    type StylingAttributes;

    /// Render any containing components for the form.
    ///
    /// This allows you to wrap all the form components
    /// in another component if neccisary.
    ///
    /// Do NOT wrap it in an actual `form` element; any
    /// wrapping should be done with `div` or similar elements.
    fn form_frame(&self, form: ControlRenderData<Self, View>) -> View;

    /// Wraps the view of a custom component.
    ///
    /// The rendering of the custom component is given by the `inner` view.
    /// Here the styler has a chance wrap the view with other components, or
    /// applying the styling attributes.
    ///
    /// This method does not need to be called by the custom component, but
    /// the custom component may make use of this method for the
    /// aforementioned reasons.
    fn custom_component(&self, style: &[Self::StylingAttributes], inner: View) -> View;

    /// Renders a group.
    ///
    /// The inner view for the group's components is provided.
    /// This method should wrap the group in any visual grouping elements,
    /// and apply the styles.
    fn group(&self, group: Rc<ControlRenderData<Self, View>>) -> View;

    /// Renders a spacer.
    ///
    /// See [`SpacerData`].
    fn spacer(&self, control: Rc<ControlRenderData<Self, SpacerData>>) -> View;

    /// Renders a heading for a section of the form.
    fn heading(&self, control: Rc<ControlRenderData<Self, HeadingData>>) -> View;

    /// Renders a submit button.
    ///
    /// See [`SubmitData`].
    fn submit(&self, control: Rc<ControlRenderData<Self, SubmitData>>) -> View;

    /// Renders a button.
    ///
    /// See [`ButtonData`]
    fn button<FD: FormToolData>(
        &self,
        control: Rc<ControlRenderData<Self, ButtonData<FD>>>,
        data_signal: RwSignal<FD>,
    ) -> View;

    /// Renders some output text.
    ///
    /// See [`OutputData`].
    fn output(
        &self,
        control: Rc<ControlRenderData<Self, OutputData>>,
        value_getter: Option<Signal<String>>,
    ) -> View;

    /// Renders a input control that should be hidden from the user.
    ///
    /// See [`HiddenData`].
    fn hidden(
        &self,
        control: Rc<ControlRenderData<Self, HiddenData>>,
        value_getter: Option<Signal<String>>,
    ) -> View;

    /// Renders a text input control.
    ///
    /// See [`TextInputData`].
    fn text_input(
        &self,
        control: Rc<ControlRenderData<Self, TextInputData>>,
        value_getter: Signal<<TextInputData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<TextInputData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View;

    /// Renders a text area control.
    ///
    /// See [`TextAreaData`].
    fn text_area(
        &self,
        control: Rc<ControlRenderData<Self, TextAreaData>>,
        value_getter: Signal<<TextAreaData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<TextAreaData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View;

    /// Renders a group of radio buttons.
    ///
    /// See [`RadioButtonsData`].
    fn radio_buttons(
        &self,
        control: Rc<ControlRenderData<Self, RadioButtonsData>>,
        value_getter: Signal<<RadioButtonsData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<RadioButtonsData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View;

    /// Renders a select (or dropdown) control.
    ///
    /// See [`SelectData`].
    fn select(
        &self,
        control: Rc<ControlRenderData<Self, SelectData>>,
        value_getter: Signal<<SelectData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<SelectData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View;

    /// Renders a checkbox control.
    ///
    /// See [`CheckboxData`].
    fn checkbox(
        &self,
        control: Rc<ControlRenderData<Self, CheckboxData>>,
        value_getter: Signal<<CheckboxData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<CheckboxData as ControlData>::ReturnType)>,
    ) -> View;

    /// Renders a stepper control.
    ///
    /// See [`StepperData`].
    fn stepper(
        &self,
        control: Rc<ControlRenderData<Self, StepperData>>,
        value_getter: Signal<<StepperData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<StepperData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View;

    /// Renders a slider control.
    ///
    /// See [`SliderData`].
    fn slider(
        &self,
        control: Rc<ControlRenderData<Self, SliderData>>,
        value_getter: Signal<<SliderData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<SliderData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View;
}
