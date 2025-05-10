# Getting Started

This guide will walk you through creating your first leptos_form_tool form.

## Form Data

Start by creating the form data.

This is a struct that should contain the data for the entire form.

```rust
/// All FormToolData implementors must also implement `Clone` and be `'static`.
/// `Serialize` and `Deserialize` are needed to send it to the server.
/// `Default` and `Debug` are not required, but helpful.
#[derive(Clone, Default, Serialize, Deserialize, Debug)]
struct HelloWorldFormData {
  first: String,
  last: String,
  age: u32,
  sport: String,
}
```

## Defining the Form Layout

Then, to define how the form should be rendered, implement the `FormToolData`
trait. This trait requires you to define a *style* and *context* for the form.
For now, we can use the style that leptos_form_tool provides, `GridFormStyle`,
and `()` for the context.

Specify the logic for laying out the form in the `build_form` method using the
provided `FormBuilder<Self>`. A full example of this will be shown later.
The elements you may add to the form are called controls. Controls that do not
accept user input (for example, a heading or a button) are called vanity controls.

### Defining Controls

To add controls to the form you can use the methods on the `FormBuilder`
that allow you to build an individual control. For example, to add a
button to the form:
```rust
form_builder
  .button(|button_builder| {
    button_builder
      .text("Reset Form")
      .action(|_event, form_data| {
        form_data.set(Self::default());
      })
  })
```
As you will soon see, leptos_form_tool makes heavy use of the
[builder pattern](https://en.wikipedia.org/wiki/Builder_pattern).
See the
[docs](https://docs.rs/leptos_form_tool/latest/leptos_form_tool/controls/index.html)
for a full list of the controls.

When building a control, you need to provide a getter and a setter function
that get and set the corresponding field on the form data. This is how Leptos
Form Tool knows what field the control corresponds to. The getter is a function
that takes the full form data and returns the field. The setter is a function
that takes a mut reference to the full form data and a value and sets the
field to that value. For example, here is how you would define a text input
for the user's name:
```rust
form_builder.text_input(|t| {
  t.labeled("First Name")
    .getter(|fd| fd.first.clone())
    .setter(|fd, value| fd.first = value)
    // don't worry about this for now:
    .parse_trimmed()
})
```

VanityControls will never need a setter, since they only display information.
VanityControls may need getters if the information they display is based on the 
form data. For example, the `output` control can display information from the
form data, so it needs a getter.

#### Parse and Unparse Functions

Sometimes, the type that the control returns doesn't match the type of the field
in your form data. For example, you might have a select (drop-down menu) control
that returns a string, but you form data uses an enum. leptos_form_tool has
parse and unparse functions for this conversion.

Parse and unparse functions can always be specified explicitly with the
`parse_custom` method. With `parse_custom` you specify functions that map the
control return type to the field type (and vice-versa). Most of the time,
however, you can use one of several builder methods that create the un/parse
functions for you. For example, if the field type and control type can be
converted to and from each other with the `core::convert::TryFrom` trait then you
can call the `parse_from` method to automatically create the un/parse functions
using that `TryFrom` implementation. If the control's type is String, and the field
type implements `FromStr` and `ToString`, you can call `parse_string` to generate
un/parse functions using those traits. `parse_trimmed` is the same, but it trims
the string before parsing. More of these methods can be seen in the
[docs](https://docs.rs/leptos_form_tool/latest/leptos_form_tool/controls/struct.ControlBuilder.html).
These automatic methods should cover most use cases, but you always have the option
to define your own if not.

It is important to note that parsing from the control's type to the field type
IS allowed to fail. If it fails, it will be displayed like a validation error
(more on that in the next section). Conversion from the field type to the control
type is NOT allowed to fail; the FormData should always be able to be displayed.

#### Validation Functions

Validation functions can be defined on a field to add some extra criteria
for what counts as a valid entry. The validation function takes the entire
form data which allows you to define complex logic (potentially using other form
fields) decide if this field is valid or not. If any validation function fails,
the form will not be allowed to be submitted. In addition, when you build a
validator, it will collect all of the validation functions that you define on
these fields, which can be used to validate the data again on the server side.
See
[FormValidator](https://docs.rs/leptos_form_tool/latest/leptos_form_tool/struct.FormValidator.html).

leptos_form_tool provides a
[`ValidationBuilder`](https://docs.rs/leptos_form_tool/latest/leptos_form_tool/struct.ValidationBuilder.html)
to help you build validation functions, which in some cases might be easier than
defining a closure manually.

Here is an example of manually defining a validation function for a text field:
```rust
form_builder.text_input(|t| {
  t.named("data[last]")
    .labeled("Last Name")
    .getter(|fd| fd.last.clone())
    .setter(|fd, value| fd.last = value)
    // use the `TryFrom` trait to un/parse
    .parse_from()
    .validation_fn(|fd| {
      // manually specify the validation
      if fd.last.is_empty() {
          Err("Last Name must not be empty!".into())
      } else if fd.last.len() < 4 {
          Err("Last Name must not be less than 4 characters!".into())
      } else {
          Ok(())
      }
    })
  })
```

A similar validation function could be built with the `ValidationBuilder`.
```rust
form_builder.text_input(|t| {
  t.named("data[last]")
    .labeled("Last Name")
    .getter(|fd| fd.last.clone())
    .setter(|fd, value| fd.last = value)
    // use the `TryFrom` trait to un/parse
    .parse_from()
    .validation_fn(
      // use the ValidationBuilder to set a required field
      ValidationBuilder::for_field(|fd: &HelloWorldFormData| fd.last.as_str())
        .named("Last Name")
        .required()
        .min_len(4)
        .build(),
    )
  })
```

#### Styling Attributes

Recall that a `FormStyle` defines how a form looks. It is
responsible for actually rendering the form to HTML.
Sometimes, your style might want to have a configuration
value that the `ControlBuilder` doesn't have. This is
where styling attributes come in. When you define a style
(a type that implements `FormStyle`), you must define the
`StylingAttributes` associated type. This is how
leptos_form_tool allows you to pass extra, customizable
attributes to the style for rendering. For example, the
provided style implementation, `GridFormStyle` has the
`GFStyleAttr` type for extra styling:
```rust
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
```
You can apply these styling attributes to any control
via the `.style()` builder method. You may apply as many
styling attributes to a control as you wish.

## Using the Form

Once the form has been defined by implementing the `FormToolData`
trait, using the form is quite simple. You just need to provide
the form data, style, context, and where the form should submit to.

```rust
let form = HelloWorldFormData::default().get_plain_form(
  "/api/endpoint",
  |_, _| {},
  GridFormStyle::default(),
  (),
);

view! {
  <div>
    <h1> "This is My Form!" </h1>
    {form.into_any()}
  </div>
}
```

You can also have the form build to an `ActionForm`, this will be very 
familiar if you've used an `ActionForm` before.

You might have noticed the goofy names that we put in our form above, like
"data[first]" instead of just "first". This is done to allow the form to use
the SubmitForm as an action. See 
[ActionForms](https://book.leptos.dev/progressive_enhancement/action_form.html#complex-inputs)
in the Leptos book for more.

```rust
#[component]
pub fn HelloWorldFormPage() -> impl IntoView {
    let server_fn_action = ServerAction::<SubmitForm>::new();

    let form = HelloWorldFormData::default().get_action_form(
        server_fn_action,
        |_, _| {},
        GridFormStyle::default(),
        (),
    );
    let response = server_fn_action.value();

    view! {
      <div>
        <h1> "This is My Form!" </h1>
        // display the form
        {form.into_any()}
        // display the result from the server
        {move || response.get().map(|result| result.ok())}
      </div>
    }
}

#[server(SubmitForm)]
async fn submit_form(data: HelloWorldFormData) -> Result<String, ServerFnError> {
  data.validate(()).map_err(ServerFnError::new)?;

  Ok(format!(
    "Hello {} {} ({}), You must like {}!",
    data.first, data.last, data.age, data.sport
  ))
}
```

Lastly, there is the `get_form` method. This is almost identical to the ActionForm 
version. In fact, if you do everything right, you won't even notice a
difference. Under the hood, `get_form` serializes and sends your FormToolData 
struct directly by calling the server function.
`get_action_form`, on the other hand, will try to construct your FormToolData
struct from the html form fields using the 
[`FromFormData`](https://docs.rs/leptos_router/latest/leptos_router/trait.FromFormData.html)
trait.

Since the `get_form` method will directly serialize and send the form data,
your form data needs to be `Serialize` and `Deserialize`. Since it doesn't
use the `FromFormData` trait, you can name the inputs whatever you want and 
it will still work (though you should try to name them correctly anyway to 
support progressive enhancement). The example is the same as above, 
just replace `get_action_form` with `get_form`, 
and add the `Serialize` and `Deserialize` derives.

### Context

Sometimes you may want to tweak how the form looks depending on the context.
leptos_form_tool handles this by adding a `Context` associated type to the
`FormToolData` trait. So far, we've been ignoring it by setting the type to
`()`. But, maybe you want to change the options of a select (drop-down) control
based on something outside the form. In that case, you can define the `Context`
to be a `Vec<String>` and use it when rendering the select. Instead of calling
the `.select()` builder method, you will use the `select_cx()` method. Other
controls have similar methods. This gives you access to the context (wrapped in
an `Arc`) that you can use to build your select. Now whenever you build a form,
you need to provide a `Vec<String>` to the context field of the `get_form_*`
methods.

## Server Side Validation

It's common to validate the form on the client (browser) side to give the
use immediate feedback before the form is even submitted. Of course we want
to also want to validate on the server side for security reasons.
leptos_form_tool allows you to reuse the validation logic that you provided
in the form to check the validity of the form data. If a context is used,
you should ensure that the context is the same on the client and the server
to ensure the validation logic is the same. To do this validation just call
`.validate(cx)` on your form data.

It should be noted that this `.validate()` method is not cheep. In order to
figure out what validation functions need to be run, the entire form is built
(but not rendered). If your context doesn't change, you can use the
`FormData::get_validator()` method to build this validator once and cache it.
Then you can use `FormValidator::validate()` repeatedly without rebuilding
the form every time.

It is also important to note that controls that are not shown (the
`.show_when(/* condition */)` condition evaluates to `false`) do not get
validated. This means that if the context changes from call to call, you
cannot build and cache a `FormValidator`. If you are certain, however, that
changing the context will not change the validation functions, you may use
the `FormValidator`.

## Closing Thoughts

This should be enough to get you started! See the
[docs](https://docs.rs/leptos_form_tool/latest/leptos_form_tool/index.html)
for a more detailed API documentation.

For a simple example with all of this put together, see the
[example project](https://github.com/MitchellMarinoDev/leptos_form_tool_example.git).
