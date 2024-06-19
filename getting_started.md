# Getting Started

This guide will walk you through creating your first leptos_form_tool form.

## Form Data

Start by creating the form data.

This struct should contain all the data for the entire form.

```rust
// all FormToolData implementors must also implement Clone and be 'static
// Default and Debug are not required, but helpful
#[derive(Clone, Default, Debug)]
struct HelloWorldFormData {
  first: String,
  last: String,
  age: u32,
  sport: String,
}
```

## Defining the Form Layout

Then, to define how the for should be rendered, implement the `FormToolData`
trait. You will need to define the style that this form will use and what
context it will have.

It is required to define the style here, as the style needs to be known to
add the StylingAttributes to the controls.

The `build_form` method provides you with a `FormBuilder<Self>`. You can define
controls on the form by using the builder methods. Some controls don't accept
input from the user, they just display information. These are refered to as
`VanityControls` by leptos_form_tool. An example of a vanity control would 
be a heading. Other controls do accept user input and are just refered to as 
`Controls` by leptos_form_tool.

### Defining Controls

There are other builder methods defined the the ControlBuilder for certain
controls. For example, the TextInput builder has a `placeholder` method
that sets the placeholder of the text input.

When building a control, you will need to provide a getter and setter
to get the field and set the field on the form data. The getter is a function
that takes the full form data, and returns the field. The setter is a function
that takes the full form data and a value, and sets the field to that value.
Examples for these getters and setters are shown below. 

A VanityControl will never need a setter, as they only display information.
Sometimes they need getters if the information they display is based on the 
form data. For example, the output control can display information from the
form data, so it needs a getter.

#### Parse and Unparse Functions

Controls also need a set of parse and unparse functions. The type that the
control returns could be anything. For example, a range slider might return a
`i32`, a text input might return a `String`. leptos_form_tool needs a way to 
convert the type of the field, to the type of the control and vice versa.
This is what the un/parse functions are for.

The parse and unparse functions can always be specified with the `parse_custom`
method, but there are several builder methods that create the parse functions 
automatically. For example, if the field type and control type can be
converted to and from each other with the `From` trait (this is true if the
two types are the same) then you can call the `parse_from` method to 
automatically create the un/parse functions using that conversion.
If the control's type is String, and the field type implements `FromStr` and
`ToString`, you can call `parse_string` to generate un/parse functions using
that trait. `parse_trimmed` does the same, but trims the string before parsing.
This should cover most use cases, but you always have the option to define
your own.

It is important to note that parsing from the control's type to the field type
IS allowed to fail. If it fails, it will be displayed just like any other
validation error. Conversion from the field type to the control type is NOT
allowed to fail; the FormData should always be able to be displayed.

#### Validation Functions

Validation functions can be defined on a field to add some extra criteria
for what counts as a valid entry. The validation function takes the entire
forms data. This allows you to use the entire state of the form to decide if
this field is valid or not. If any validation function fails, the form will
not be allowed to submit. In addition, when you build a validator, it will
collect all of the validation functions that you define on these fields.

leptos_form_tool provides a `ValidationBuilder` to help you build validation
functions, which, in some cases, might be easier than defining a closure
yourself.

```rust
impl FormToolData for HelloWorldFormData {
  // The form style to use.
  type Style = GridFormStyle;
  // The external context needed for rendering the form.
  // In this case, nothing.
  type Context = ();

  fn build_form(fb: FormBuilder<Self>) -> FormBuilder<Self> {
    fb.heading(|h| h.title("Welcome"))
      .text_input(|t| {
        t.named("data[first]")
          .labeled("First Name")
          .getter(|fd| fd.first)
          .setter(|fd, value| fd.first = value)
          // trim the string before writing to the field
          .parse_trimmed()
          .validation_fn(
            // Using the ValidationBuilder to set a required field
            ValidationBuilder::for_field(|fd: &HelloWorldFormData| fd.first.as_str())
              .named("First Name")
              .required()
              .build(),
          )
          // width out of 12
          .style(Width(4))
          // defines text that shows up when hovering over it
          .style(Tooltip("Your given first name".to_string()))
      })
      .text_input(|t| {
        t.named("data[last]")
          .labeled("Last Name")
          .getter(|fd| fd.last)
          .setter(|fd, value| fd.last = value)
          // dont trim the string, just write it to the field
          .parse_from()
          .validation_fn(
            // using the ValidationBuilder to set a required field
            ValidationBuilder::for_field(|fd: &HelloWorldFormData| fd.last.as_str())
              .named("Last Name")
              .required()
              .build(),
          )
          .style(Width(8))
          .style(Tooltip("Your last name".to_string()))
      })
      // using the _cx varient allows access to the context
      // in this case, its `()` which doesnt help us that much.
      .stepper_cx(|s, _cx| {
        s.named("data[age]")
          .labeled("Age")
          .getter(|fd| fd.age)
          .setter(|fd, value| fd.age = value)
          // trim the string then try to parse to a `u32`
          .parse_trimmed()
          .validation_fn(move |fd| {
            // defining a validation function with a closure
            (fd.age > 13)
            .then_some(())
            .ok_or_else(|| String::from("Too Young"))
          })
          .style(Width(6))
          .style(Tooltip("Your age in years".to_string()))
      })
      .select(|s| {
        s.named("data[sport]")
          .labeled("Favorite Sport")
            .getter(|fd| fd.sport)
            .setter(|fd, value| fd.sport = value)
            .parse_from()
            // set the options for the select along with their values
            .with_options_valued(vec![
              ("Football", "football"),
              ("Soccer", "soccer"),
              ("Ice Hockey", "ice_hockey"),
              ("Golf", "golf"),
            ].into_iter())
            .style(Width(6))
      })
      .submit(|s| s.text("Submit"))
  }
}
```

Now, using the form is quite simple. You just need to provide the form data,
style, context, and where the form should point to.

```rust
let form = ExampleFormData::default().get_plain_form("/api/endpoint", GridFormStyle::default(), ());

view! {
  <div>
    <h1> "This is My Form!" </h1>
    {form}
  </div>
}
```

You can also have the form build to an `ActionForm`, this will be very 
familiar if you've used an `ActionForm` before.

You might have noticed the goofy names that we put in our form above, like
"data[first]" instead of just "first". This is done to allow the form to use
the SubmitForm as an action. See 
[ActionForms](https://book.leptos.dev/progressive_enhancement/action_form.html#complex-inputs)
in the leptos book for more.

```rust
#[component]
fn FormWrapper() -> impl IntoView {
  let server_fn_action = create_server_action::<SubmitForm>();
 
  let form = HelloWorldFormData::default().get_action_form(server_fn_action, GridFormStyle::default(), ());
  let response = server_fn_action.value();

  view! {
    <div>
      <h1> "This is My Form!" </h1>
      // display the form
      {form}
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

Lastly there is the `get_form` method. This almost identical to the ActionForm 
version. In fact, if you do everything right, you wont even notice a
difference. Under the hood, this sends your FormToolData struct directly
by calling the server function, whereas the `ActionForm` version will try
to construct your type using the 
[`FromFormData`](https://docs.rs/leptos_router/latest/leptos_router/trait.FromFormData.html)
trait. Using the `get_form` method instead will allow you to name the inputs
whatever you want (though you should try to name them correctly anyway to 
support progressive enhancement) and it will still work. The example is the 
same as above, just replace `get_action_form` with `get_form`.
