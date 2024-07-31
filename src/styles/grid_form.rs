use super::FormStyle;
use crate::controls::{
    button::ButtonData, checkbox::CheckboxData, heading::HeadingData, hidden::HiddenData,
    output::OutputData, radio_buttons::RadioButtonsData, select::SelectData, slider::SliderData,
    spacer::SpacerData, stepper::StepperData, submit::SubmitData, text_area::TextAreaData,
    text_input::TextInputData, ControlRenderData, UpdateEvent, ValidationState,
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

    fn heading(
        &self,
        control: Rc<ControlRenderData<Self, HeadingData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        use crate::controls::heading::HeadingLevel::*;

        let title = move || value_getter.map(|v| v.get()).unwrap_or_default();

        let view = match control.data.level {
            H1 => view! { <h1 class="form_heading"> {title} </h1> }.into_view(),
            H2 => view! { <h2 class="form_heading"> {title} </h2> }.into_view(),
            H3 => view! { <h3 class="form_heading"> {title} </h3> }.into_view(),
            H4 => view! { <h4 class="form_heading"> {title} </h4> }.into_view(),
        };

        self.custom_component(&control.styles, view)
    }

    fn submit(
        &self,
        control: Rc<ControlRenderData<Self, SubmitData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let title = move || value_getter.map(|v| v.get()).unwrap_or_default();

        self.custom_component(
            &control.styles,
            view! { <input type="submit" value=title class="form_submit"/> }.into_view(),
        )
    }

    fn button(
        &self,
        control: Rc<ControlRenderData<Self, ButtonData>>,
        value_getter: Option<Signal<String>>,
    ) -> View {
        let action = control.data.action.clone();
        let on_click = move |ev: MouseEvent| {
            if let Some(ref action) = action {
                action(ev)
            }
        };

        let title = move || value_getter.map(|v| v.get()).unwrap_or_default();

        let view = view! {
            <button type="button" class="form_button" on:click=on_click>
                {title}
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
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let input = view! {
            <input
                type=control.data.input_type
                id=&control.data.name
                name=&control.data.name
                placeholder=control.data.placeholder.as_ref()
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                prop:value=move || value_getter.get()
            />
        };

        let input = match control.data.update_event {
            UpdateEvent::OnFocusout => input.on(ev::focusout, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnInput => input.on(ev::input, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnChange => input.on(ev::change, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
        };

        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().take_msg()}</span>
            </div>
            {input}
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn text_area(
        &self,
        control: Rc<ControlRenderData<Self, TextAreaData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let input = view! {
            <textarea
                id=&control.data.name
                name=&control.data.name
                placeholder=control.data.placeholder.as_ref()
                prop:value=move || value_getter.get()
                style="resize: vertical;"
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
            ></textarea>
        };

        let input = match control.data.update_event {
            UpdateEvent::OnFocusout => input.on(ev::focusout, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnInput => input.on(ev::input, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
            UpdateEvent::OnChange => input.on(ev::change, move |ev| {
                value_setter.set(event_target_value(&ev));
            }),
        };

        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().take_msg()}</span>
            </div>
            {input}
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn radio_buttons(
        &self,
        control: Rc<ControlRenderData<Self, RadioButtonsData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
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
                                value_setter.set(value_clone2.clone());
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
                <span class="form_error">{move || validation_state.get().take_msg()}</span>
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
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
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
                <span class="form_error">{move || validation_state.get().take_msg()}</span>
            </div>
            <select
                id=&control.data.name
                name=&control.data.name
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                on:input=move |ev| {
                    value_setter.set(event_target_value(&ev));
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
        value_getter: Signal<bool>,
        value_setter: SignalSetter<bool>,
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
                        value_setter.set(new_value);
                    }
                />
                <span style="margin: auto 0.5rem;">{label}</span>
            </label>
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn stepper(
        &self,
        control: Rc<ControlRenderData<Self, StepperData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().take_msg()}</span>
            </div>
            <input
                type="number"
                id=&control.data.name
                name=&control.data.name
                step=control.data.step.clone()
                min=control.data.min.clone()
                max=control.data.max.clone()
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                prop:value=move || value_getter.get()
                on:input=move |ev| {
                    value_setter.set(event_target_value(&ev));
                }
            />
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }

    fn slider(
        &self,
        control: Rc<ControlRenderData<Self, SliderData>>,
        value_getter: Signal<String>,
        value_setter: SignalSetter<String>,
        validation_state: Signal<ValidationState>,
    ) -> View {
        let view = view! {
            <div>
                <label for=&control.data.name class="form_label">
                    {control.data.label.as_ref()}
                </label>
                <span class="form_error">{move || validation_state.get().take_msg()}</span>
            </div>
            <input
                type="range"
                id=&control.data.name
                name=&control.data.name
                min=control.data.min.clone()
                max=control.data.max.clone()
                class="form_input"
                class=("form_input_invalid", move || validation_state.get().is_err())
                prop:value=move || value_getter.get()
                on:input=move |ev| {
                    let value = event_target_value(&ev);
                    value_setter.set(value);
                }
            />
        }
        .into_view();

        self.custom_component(&control.styles, view)
    }
}
