# leptos_form_tool

[![crates.io](https://img.shields.io/crates/v/leptos_form_tool)](https://crates.io/crates/leptos_form_tool)
[![docs.rs](https://docs.rs/leptos_form_tool/badge.svg)](https://docs.rs/leptos_form_tool)

A declarative way to create forms for [leptos](https://leptos.dev/).

leptos_form_tool allows you to define forms in a declarative way.
Want a text box? Just call `.text_input` on the form builder. Then, separately,
you define a FormStyle to specify how that text box should be rendered.
This has several advantages:
 - Separates the layout of the form from how it is rendered and styled
 - Allows different styles to be swapped in and out quickly

## Validations

You might find yourself asking, but why not just use components?

The biggest reason for creating leptos_form_tool is support for
validating the fields. This validation logic can get rather complex. You
likely want to perform validation on the client side when the user clicks
submit (to immediately give the user feedback). But you also want to do
the same validation on the server for security reasons.

Additionally, you might want to change the validation of one field based
on the value of another field. For example, you might want to make sure the "day"
field is in a valid range, but that range depends on what the user selects in
the "month" field. Or you might want to make sure the "confirm password" field
matches the "password" field. leptos_form_tool makes this easy, as the
validation function operates on the entire form's data.

Sometimes you might want to hide some controls, and validation for those
controls should only be done when they are visible.

leptos_form_tool takes care of all this for you.

## FormStyle

To define how to render all the components a form might use, you define
a type that implements the `FormStyle` trait. This trait has a method for each
control that the form might need to render. To use this new style, you
just need to change the `Style` associated trait of your form to your new style.

It's actually a little more complicated than that...

To give custom styles a little more freedom to configure how to render their
controls on a per-control basis, the style will define an associated type 
(usually an enum) called `StylingAttributes`. A styling attribute can be added
to a control by calling `.style(/* style */)` on the control builder. These 
styling attributes are accessible to the `FormStyle` implementation when 
rendering that control.

Therefore, swapping out styles also requires swapping out all the `.style()` calls.
Still, a relatively small change. This also means the specific implementation of
how a form gets rendered is decoupled from the form code, which is nice.

## Builders

leptos_form_tool makes heavy use of the
[builder pattern](https://en.wikipedia.org/wiki/Builder_pattern).
You will build the form, controls, and sometimes even validation functions by calling
methods on builders that construct the objects based on the values you give it.

## Context

Sometimes, you might want to be able to use something from the form's context
to render the form. For example, you may want to use a user's token as context,
to only render part of the form if they are an administrator. Or, you may
need to get the options for a certain drop-down dynamically. The form's context
is the solution to these problems.

On the form, you define the associated type `Context`. Then, any time you construct
a `Form`, you must provide that context. The context can be used in
the building of the form and can change what controls are rendered. Each control
builder function (ex. `.text_input()`) has a context variant 
(ex. `.text_input_cx()`) that allows you to use the context
for building that control.

To avoid a whole lot of headaches, the context is immutable once passed into
the form. However, you can use leptos signals in the context just fine, as you don't
need mutable access to call get/set on the signal.

Since the context can change how the form is rendered, and what controls are
shown/hidden (thus changing what controls are validated), the context is
needed to validate the form data on the server side. In general, when 
validating the form on the server, you should pass the same context as 
you did on the client, to make sure the validation logic is the same. 
If you are sure that changing the context won't change any of the validations, 
it is ok to use different contexts on the server and client. 

It is important to note that for controls that are not shown 
(the `.show_when(/* condition */)` condition evaluates to `false`), 
the validation for that field does not run.

## Custom Components

leptos_form_tool also supports custom components that can be defined in the
user space. This keeps leptos_form_tool from putting limits on what you can do.
There are `custom_*` methods on the form builder that allow you to add your
own components.

## Getting Started

Excited and ready to get started?

For a more in-depth guide, see [`getting_started.md`](/getting_started.md).

To learn by example, see the
[example project](https://github.com/MitchellMarinoDev/leptos_form_tool_example).

## Stability

So far, this project is developed and used by me. I will continue to make updates
as I find and fix bugs, but I am sure there are more in there somewhere. If you
find one, please open an issue.

## Compatability

| form_tool version | leptos version |
|-------------------|----------------|
| 0.2.0             | 0.6            |
| 0.1.0             | 0.6            |

## Contributing

To contribute, fork the repo and make a PR.
If you find a bug, feel free to open an issue.

By contributing, you agree that your changes are
subject to the license found in [`/LICENSE`](/LICENSE).
