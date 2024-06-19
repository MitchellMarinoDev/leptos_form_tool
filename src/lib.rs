//! `leptos_form_tool` offers a declaritve way to create forms for
//! [leptos](https://leptos.dev/).
//!
//! To learn more, see the
//! [README.md](https://github.com/MitchellMarinoDev/leptos_form_tool/blob/main/README.md)
pub mod controls;
mod form;
mod form_builder;
pub mod styles;
mod validation_builder;

pub use form::{Form, FormToolData, FormValidator};
pub use form_builder::FormBuilder;
pub use validation_builder::ValidationBuilder;
