// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{EventController, PadActionType, PropagationPhase, Widget, Window};
use glib::{prelude::*, translate::*};
use std::fmt;

glib::wrapper! {
    #[doc(alias = "GtkPadController")]
    pub struct PadController(Object<ffi::GtkPadController, ffi::GtkPadControllerClass>) @extends EventController;

    match fn {
        type_ => || ffi::gtk_pad_controller_get_type(),
    }
}

impl PadController {
    #[doc(alias = "gtk_pad_controller_new")]
    pub fn new(
        window: &impl IsA<Window>,
        group: &impl IsA<gio::ActionGroup>,
        pad: Option<&gdk::Device>,
    ) -> PadController {
        skip_assert_initialized!();
        unsafe {
            from_glib_full(ffi::gtk_pad_controller_new(
                window.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                pad.to_glib_none().0,
            ))
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`PadController`] objects.
    ///
    /// This method returns an instance of [`PadControllerBuilder`](crate::builders::PadControllerBuilder) which can be used to create [`PadController`] objects.
    pub fn builder() -> PadControllerBuilder {
        PadControllerBuilder::new()
    }

    #[doc(alias = "gtk_pad_controller_set_action")]
    pub fn set_action(
        &self,
        type_: PadActionType,
        index: i32,
        mode: i32,
        label: &str,
        action_name: &str,
    ) {
        unsafe {
            ffi::gtk_pad_controller_set_action(
                self.to_glib_none().0,
                type_.into_glib(),
                index,
                mode,
                label.to_glib_none().0,
                action_name.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "action-group")]
    pub fn action_group(&self) -> Option<gio::ActionGroup> {
        glib::ObjectExt::property(self, "action-group")
    }

    pub fn pad(&self) -> Option<gdk::Device> {
        glib::ObjectExt::property(self, "pad")
    }
}

impl Default for PadController {
    fn default() -> Self {
        glib::object::Object::new_default::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`PadController`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct PadControllerBuilder {
    builder: glib::object::ObjectBuilder<'static, PadController>,
}

impl PadControllerBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn action_group(self, action_group: &impl IsA<gio::ActionGroup>) -> Self {
        Self {
            builder: self
                .builder
                .property("action-group", action_group.clone().upcast()),
        }
    }

    pub fn pad(self, pad: &gdk::Device) -> Self {
        Self {
            builder: self.builder.property("pad", pad.clone()),
        }
    }

    pub fn propagation_phase(self, propagation_phase: PropagationPhase) -> Self {
        Self {
            builder: self
                .builder
                .property("propagation-phase", propagation_phase),
        }
    }

    pub fn widget(self, widget: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("widget", widget.clone().upcast()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`PadController`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> PadController {
        self.builder.build()
    }
}

impl fmt::Display for PadController {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("PadController")
    }
}
