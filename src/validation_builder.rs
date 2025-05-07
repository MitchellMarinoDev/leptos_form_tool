use crate::{controls::ValidationFn, FormToolData};
use std::fmt::Display;

/// A function that validates a field.
///
/// This is similar to [`ValidationFn`](crate::controls::ValidationFn)
/// but takes a &str for the name of the field for improved error messages.
type ValidationBuilderFn<T> = dyn Fn(&str, &T) -> Result<(), String> + Send + Sync + 'static;

/// A helper builder that allows you to specify a validation function
/// declaritivly
///
/// Using this builder is not required as validation functions can just be
/// closures, but for simple validation function this builder can be helpful
///
/// Validations are run in the order that they are called in the builder.
pub struct ValidationBuilder<FD: FormToolData, T: ?Sized + 'static> {
    /// The name of the field, for error messages.
    name: String,
    /// The getter function for the field to validate.
    field_fn: Box<dyn Fn(&FD) -> &T + Send + Sync + 'static>,
    /// The functions to be called when validating.
    functions: Vec<Box<ValidationBuilderFn<T>>>,
}

impl<FD: FormToolData, T: ?Sized + 'static> ValidationBuilder<FD, T> {
    /// Creates a new empty [`ValidationBuilder`] on the given field.
    pub fn for_field(field_fn: impl Fn(&FD) -> &T + Send + Sync + 'static) -> Self {
        ValidationBuilder {
            name: String::from("Field"),
            field_fn: Box::new(field_fn),
            functions: Vec::new(),
        }
    }

    /// The name of the field that is being validated.
    ///
    /// This is the name that will be used for error messages.
    pub fn named(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    /// Adds a custom validation function.
    ///
    /// The function should take the value as an argument and return
    /// a [`Result<(), String>`], just like any other validation function.
    pub fn custom(mut self, f: impl ValidationFn<T>) -> Self {
        self.functions.push(Box::new(move |_name, value| f(value)));
        self
    }

    /// Builds the action validation function.
    pub fn build(self) -> impl ValidationFn<FD> {
        move |form_data| {
            let value = (self.field_fn)(form_data);
            for f in self.functions.iter() {
                match f(self.name.as_str(), value) {
                    Ok(()) => {}
                    err => return err,
                }
            }
            Ok(())
        }
    }
}

impl<FD: FormToolData, T> ValidationBuilder<FD, Option<T>> {
    /// Requires the field to be `Some`.
    pub fn required(mut self) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if value.is_none() {
                Err(format!("{} is required", name))
            } else {
                Ok(())
            }
        }));
        self
    }
}

impl<FD: FormToolData> ValidationBuilder<FD, str> {
    /// Requires the field to not be empty.
    pub fn required(mut self) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if value.is_empty() {
                Err(format!("{} is required", name))
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Requires the field's length to be at least `min_len`.
    pub fn min_len(mut self, min_len: usize) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if value.len() < min_len {
                Err(format!("{} must be >= {} characters", name, min_len))
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Requires the field's length to be less than or equal to `min_len`.
    pub fn max_len(mut self, max_len: usize) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if value.len() > max_len {
                Err(format!("{} must be <= {} characters", name, max_len))
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Requires the field to contain `pattern`.
    pub fn contains(mut self, pattern: impl ToString) -> Self {
        let pattern = pattern.to_string();
        self.functions.push(Box::new(move |name, value| {
            if !value.contains(&pattern) {
                Err(format!("{} must contain {}", name, &pattern))
            } else {
                Ok(())
            }
        }));
        self
    }
}

impl<FD: FormToolData, T: PartialOrd<T> + Display + Send + Sync + 'static>
    ValidationBuilder<FD, T>
{
    /// Requires the value to be at least `min_value` according to
    /// `PartialOrd`.
    pub fn min_value(mut self, min_value: T) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if value < &min_value {
                Err(format!("{} mut be >= {}", name, min_value))
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Requires the value to be at most `max_value` according to
    /// `PartialOrd`.
    pub fn max_value(mut self, max_value: T) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if value > &max_value {
                Err(format!("{} mut be <= {}", name, max_value))
            } else {
                Ok(())
            }
        }));
        self
    }
}

impl<FD: FormToolData, T: PartialEq<T> + Display + Send + Sync + 'static> ValidationBuilder<FD, T> {
    /// Requires the field to be in the provided whitelist.
    pub fn whitelist(mut self, whitelist: Vec<T>) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if !whitelist.contains(value) {
                Err(format!("{} cannot be {}", name, value))
            } else {
                Ok(())
            }
        }));
        self
    }

    /// Requires the field to not be in the provided blacklist.
    pub fn blacklist(mut self, blacklist: Vec<T>) -> Self {
        self.functions.push(Box::new(move |name, value| {
            if blacklist.contains(value) {
                Err(format!("{} cannot be {}", name, value))
            } else {
                Ok(())
            }
        }));
        self
    }
}
