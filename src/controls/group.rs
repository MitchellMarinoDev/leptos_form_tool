use std::rc::Rc;

use super::{ControlRenderData, ValidationCb};
use crate::styles::FormStyle;
use crate::{form::FormToolData, form_builder::FormBuilder};
use leptos::{CollectView, RwSignal};

impl<FD: FormToolData> FormBuilder<FD> {
    /// Creates a form group.
    ///
    /// This creates a subsection of the form that controls can be added to
    /// like a normal form.
    pub fn group(mut self, builder: impl Fn(FormBuilder<FD>) -> FormBuilder<FD>) -> Self {
        let mut group_builder = FormBuilder::new_group(self.cx.clone());
        group_builder = builder(group_builder);

        for validation in group_builder.validations {
            self.validations.push(validation);
        }

        let render_fn = move |fs: Rc<FD::Style>, fd: RwSignal<FD>| {
            let (views, validation_cbs): (Vec<_>, Vec<_>) = group_builder
                .render_fns
                .into_iter()
                .map(|r_fn| r_fn(fs.clone(), fd))
                .unzip();

            let render_data = Rc::new(ControlRenderData {
                data: views.collect_view(),
                styles: group_builder.styles,
            });

            let view = fs.group(render_data.clone());

            let validation_cb = move || {
                let mut success = true;
                for validation in validation_cbs.iter().flatten() {
                    if !validation() {
                        success = false;
                    }
                }
                success
            };
            (view, Some(Box::new(validation_cb) as Box<dyn ValidationCb>))
        };

        self.render_fns.push(Box::new(render_fn));
        self
    }
}
