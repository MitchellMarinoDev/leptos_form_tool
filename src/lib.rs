#![doc = include_str!("../README.md")]

pub mod controls;
mod form;
mod form_builder;
pub mod styles;
mod validation_builder;

pub use form::{Form, FormToolData, FormValidator};
pub use form_builder::FormBuilder;
pub use validation_builder::ValidationBuilder;
