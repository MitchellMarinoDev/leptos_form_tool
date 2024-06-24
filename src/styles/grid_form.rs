use super::FormStyle;
use crate::{
    controls::{
        button::ButtonData, checkbox::CheckboxData, heading::HeadingData, hidden::HiddenData,
        output::OutputData, radio_buttons::RadioButtonsData, select::SelectData,
        slider::SliderData, spacer::SpacerData, stepper::StepperData, submit::SubmitData,
        text_area::TextAreaData, text_input::TextInputData, ControlData, ControlRenderData,
    },
    FormToolData,
};
use leptos::*;
use std::rc::Rc;
use web_sys::MouseEvent;

/// Styling attributes for the [`GridFormStyle`].
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum GFStyleAttr {
    /// Set the width of the control out of 12.
    /// Defaults to 12/12 (full width).
    Width(u32),
    /// Adds a tooltip to the control.
    /// This sets the html title attribute, which shows the text when the
    /// user hovers their mouse over the control for a couple seconds.
    Tooltip(String),
}

/// A complete useable example for defining a form style.
///
/// This can be used directly in by your form, or you can copy `grid_form.rs`
/// into your project and make any neccesary change. You will also want to
/// copy `grid_form.scss` from the git repo and put that in the `styles`
/// directory for your leptos project to get all the styling.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct GridFormStyle;

impl FormStyle for GridFormStyle {
    type StylingAttributes = GFStyleAttr;

    fn form_frame(&self, form: ControlRenderData<Self, View>) -> View {
        view! { <div class="form_grid">{form.data}</div> }.into_view()
    }

    /// A common function that wraps the given view in the styles
    fn custom_component(&self, styles: &[Self::StylingAttributes], inner: View) -> View {
        let mut width = 12;
        let mut tooltip = None;
        for style in styles.iter() {
            match style {
                GFStyleAttr::Width(w) => width = *w,
                GFStyleAttr::Tooltip(t) => tooltip = Some(t),
            }
        }

        view! {
            <div style:grid-column=format!("span {}", width) title=tooltip>
                {inner}
            </div>
        }
        .into_view()
    }

    fn group(&self, group: Rc<ControlRenderData<Self, View>>) -> View {
        let view = view! { <div class="form_group form_grid">{&group.data}</div> }.into_view();

        self.custom_component(&group.styles, view)
    }

    fn spacer(&self, control: Rc<ControlRenderData<Self, SpacerData>>) -> View {
        self.custom_component(
            &control.styles,
            view! { <div style:height=control.data.height.as_ref()></div> }.into_view(),
        )
    }

    fn heading(&self, control: Rc<ControlRenderData<Self, HeadingData>>) -> View {
        self.custom_component(
            &control.styles,
            view! { <h2 class="form_heading">{control.data.title.clone()}</h2> }.into_view(),
        )
    }

    fn submit(&self, control: Rc<ControlRenderData<Self, SubmitData>>) -> View {
        self.custom_component(
            &control.styles,
            view! { <input type="submit" value=&control.data.text class="form_submit"/> }
                .into_view(),
        )
    }

    fn button<FD: FormToolData>(
        &self,
        control: Rc<ControlRenderData<Self, ButtonData<FD>>>,
        data_signal: RwSignal<FD>,
    ) -> View {
        let action = control.data.action.clone();
        let on_click = move |ev: MouseEvent| {
            if let Some(action) = action.clone() {
                action(ev, data_signal)
            }
        };

        let view = view! {
            <button type="button" class="form_button" on:click=on_click>
                {&control.data.text}
            </button>
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn output(
        &self,
        control: Rc<ControlRenderData<Self, OutputData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let view = view! { <span>{move || value_getter.map(|g| g.get())}</span> }.into_view();
        self.custom_component(&control.styles, view)
    }

    fn hidden(
        &self,
        control: Rc<ControlRenderData<Self, HiddenData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let value_getter = move || value_getter.map(|g| g.get());
        view! {
            <input
                name=&control.data.name
                prop:value=value_getter
                style="visibility: hidden; position: absolute;"
            />
        }
        .into_view()
    }

    fn text_input(
        &self,
        control: Rc<ControlRenderData<Self, TextInputData>>,
        value_getter: Signal<<TextInputData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<TextInputData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().err()}</span>
            </div>
            <input
                type=control.data.input_type
                id=&control.data.name
                name=&control.data.name
                placeholder=control.data.placeholder.as_ref()
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                prop:value=move || value_getter.get()
                on:focusout=move |ev| {
                    value_setter(event_target_value(&ev));
                }
            />
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn text_area(
        &self,
        control: Rc<ControlRenderData<Self, TextAreaData>>,
        value_getter: Signal<<TextAreaData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<TextAreaData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().err()}</span>
            </div>
            <textarea
                id=&control.data.name
                name=&control.data.name
                placeholder=control.data.placeholder.as_ref()
                prop:value=move || value_getter.get()
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                on:focusout=move |ev| {
                    value_setter(event_target_value(&ev));
                }
            ></textarea>
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn radio_buttons(
        &self,
        control: Rc<ControlRenderData<Self, RadioButtonsData>>,
        value_getter: Signal<<RadioButtonsData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<RadioButtonsData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        let buttons_view = control
            .data
            .options
            .iter()
            .map(|(display, value)| {
                let value_setter = value_setter.clone();
                let display = display.clone();
                let value = value.clone();
                let value_clone = value.clone();
                let value_clone2 = value.clone();
                view! {
                    <input
                        type="radio"
                        id=&value
                        name=&control.data.name
                        value=&value
                        prop:checked=move || { value_getter.get() == value_clone }
                        on:input=move |ev| {
                            let new_value = event_target_checked(&ev);
                            if new_value {
                                value_setter(value_clone2.clone());
                            }
                        }
                    />

                    <label for=&value>{display}</label>
                    <br/>
                }
            })
            .collect_view();

        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().err()}</span>
            </div>
            <div
                class="form_input"
                class:form_input_invalid=move || validation_state.get().is_err()
            >
                {buttons_view}
            </div>
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn select(
        &self,
        control: Rc<ControlRenderData<Self, SelectData>>,
        value_getter: Signal<<SelectData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<SelectData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        let control_clone = control.clone();
        let options_view = move || {
            control_clone
            .data
            .options
            .get()
            .iter()
            .map(|(display, value)| {
                let display = display.clone();
                let value = value.clone();
                view! {
                    <option value=value.clone() selected=move || { value_getter.get() == *value }>
                        {display}
                    </option>
                }
            })
            .collect_view()
        };

        let blank_option_view = control.data.blank_option.as_ref().map(|display| {
            view! {
                <option value="" selected=move || { value_getter.get().as_str() == "" }>
                    {display}
                </option>
            }
        });

        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().err()}</span>
            </div>
            <select
                id=&control.data.name
                name=&control.data.name
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                on:input=move |ev| {
                    value_setter(event_target_value(&ev));
                }
            >
                {blank_option_view}
                {options_view}
            </select>
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn checkbox(
        &self,
        control: Rc<ControlRenderData<Self, CheckboxData>>,
        value_getter: Signal<<CheckboxData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<CheckboxData as ControlData>::ReturnType)>,
    ) -> View {
        let label = control
            .data
            .label
            .clone()
            .unwrap_or(control.data.name.clone());

        let view = view! {
            <label
                for=&control.data.name
                class="form_checkbox"
                class=("form_checkbox_checked", move || value_getter.get())
                class=("form_checkbox_unchecked", move || !value_getter.get())
            >
                <input
                    type="checkbox"
                    id=&control.data.name
                    name=&control.data.name
                    style="margin: auto 0;"
                    prop:checked=value_getter
                    on:input=move |ev| {
                        let new_value = event_target_checked(&ev);
                        value_setter(new_value);
                    }
                />
                <span style="margin: auto 0.5rem;">{label}</span>
            </label>
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    // TODO: remove stepper, replace with a `.number()` function on the text
    // field.
    fn stepper(
        &self,
        control: Rc<ControlRenderData<Self, StepperData>>,
        value_getter: Signal<<StepperData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<StepperData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().err()}</span>
            </div>
            <input
                type="number"
                id=&control.data.name
                name=&control.data.name
                step=control.data.step
                min=control.data.min
                max=control.data.max
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                prop:value=move || value_getter.get()
                on:change=move |ev| {
                    value_setter(event_target_value(&ev));
                }
            />
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn slider(
        &self,
        control: Rc<ControlRenderData<Self, SliderData>>,
        value_getter: Signal<<SliderData as ControlData>::ReturnType>,
        value_setter: Rc<dyn Fn(<SliderData as ControlData>::ReturnType)>,
        validation_state: Signal<Result<(), String>>,
    ) -> View {
        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().err()}</span>
            </div>
            <input
                type="range"
                id=&control.data.name
                name=&control.data.name
                min=control.data.min
                max=control.data.max
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                prop:value=move || value_getter.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev).parse::<i32>().ok();
                    if let Some(value) = value {
                        value_setter(value);
                    }
                }
            />
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }
}
