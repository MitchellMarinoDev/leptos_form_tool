# leptos_form_tool

A declaritive way to create forms for [leptos](https://leptos.dev/).

leptos_form_tool allows you to define forms in a declaritive way.
Want text box? Just call `.text_input` on the form builder. Then sepperatly,
you define a FormStyle to specify how that text box should be rendered.
This has a number of advantages:
 - Sepperates the layout of the form from how it is rendered and styled
 - Allows different styles to be swapped in and out quickly

## Validations

You might find yourself asking, but why not just use components?

The biggest reason for creating leptos_form_tool is support for
validating the fields. This validation logic can get rather complex, for
instance, you likely want to preform validation on the client when the user 
clicks submit to immediatly give the user feedback about any invalid input.
But you often also want to do the same validation on the server to protect
against any requests that don't come from your client or for a user that
doesn't have wasm enabled.

Additionally, you might want to change the validation of one control based
on the value of another. For example, you might want to make sure the "day"
field is in a valid range, but that range depends on what the user selects in
the "month" field. Or you might want to make sure the "confirm password" field
matches the "password" field. leptos_form_tool makes this easy, as the
validation function you provide operates on the entire form's data.

Sometimes you might not want to show some controls, and validation for those
controls should only be done when they are visible.

lepos_form_tool takes care of all this for you.

## FormStyle

To define how to render all the components that a form might use, you define
a type that implements the `FormStyle` trait. This trait has a method for each
control that the form might need to render. Once you implement that trait you
just need to change the `Style` associated trait of your form to use that new
style.

Its actually a little more complicated than that...

To give custom styles a little more freedom to configure how to render their
controls on a per-control-basis, the style will define an associated type 
(usually an enum) called `StylingAttributes`. A styling attribute can be added
to a control by calling `.style(/* style */)` on the control builder. These 
styling attributes are accessable to the `FormStyle` implementation when 
rendering that control.

Therefore, swaping out styles also requires swapping out all the `.style()`
calls.

## Builders

leptos_form_tool makes heavy use of the builder pattern. You will build the
form, controls and sometimes even validation functions by calling methods on
a builder that will construct the object based on the values you give it.

## Context

Sometimes, you might want to be able to use something from the form's context
to render the form. For example, you may want to use a user's token as context
and only render part of the form if they are an administrator. Or, you may
need to get the options for a certain drop-down dynamically. The form's context
is the solution to these problems.

On the form, you define the associated type `Context`. Then, when you construct
the `Form` object, you must provide the context. The context can be used in
the building of the form, and can change what is rendered. Each control
builder function has a context varient that allows you to use the context in
the building of the form.

To avoid a whole lot of headache, the context is immutable once passed into to
the form. However, you can have leptos signals in the context, as they dont
require mutable access to call get/set on the signal.

Since the context can change how the form is rendered, and what controls are
shown/hidden (thus changing what controls are validated), the context is
needed to validate the form data on the server side. If are sure that the
context doesn't change any of the validations, you don't have to make sure
the context is the same on client and server. If the context does change
how the form is validated, make sure to keep the context the same to maximize
the validation that can happen on the clients side, before the user even
submits the form.

## Custom Components

leptos_form_tool also supports custom components that can be defined in the
user space. Though less ergonomic, this keeps leptos_form_tool from putting
limits on what you can do. There are `custom_*` methods on the form builder
that allow you to add your component. Unfortunatly you cannot define methods
on the `ControlBuilder` to help build your controls data, so you must 
construct the ControlData for your custom type before adding it to the form.

## Getting Started

To learn by example, see the example project.

To follow a Getting Started guide, see [`getting_started.md`].

## Contributing

To contribute, fork the repo and make a PR. 
If you find a bug, feel free to open an issue. 

By contributing, you agree that your changes are 
subject to the license found in [`/LICENSE`].
