// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{
    Align, AppChooser, BaselinePosition, Box, Buildable, Container, Menu, Orientable, Orientation,
    ResizeMode, Widget,
};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkAppChooserWidget")]
    pub struct AppChooserWidget(Object<ffi::GtkAppChooserWidget, ffi::GtkAppChooserWidgetClass>) @extends Box, Container, Widget, @implements Buildable, Orientable, AppChooser;

    match fn {
        type_ => || ffi::gtk_app_chooser_widget_get_type(),
    }
}

impl AppChooserWidget {
    pub const NONE: Option<&'static AppChooserWidget> = None;

    #[doc(alias = "gtk_app_chooser_widget_new")]
    pub fn new(content_type: &str) -> AppChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_app_chooser_widget_new(
                content_type.to_glib_none().0,
            ))
            .unsafe_cast()
        }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`AppChooserWidget`] objects.
    ///
    /// This method returns an instance of [`AppChooserWidgetBuilder`](crate::builders::AppChooserWidgetBuilder) which can be used to create [`AppChooserWidget`] objects.
    pub fn builder() -> AppChooserWidgetBuilder {
        AppChooserWidgetBuilder::new()
    }
}

impl Default for AppChooserWidget {
    fn default() -> Self {
        glib::object::Object::new_default::<Self>()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`AppChooserWidget`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct AppChooserWidgetBuilder {
    builder: glib::object::ObjectBuilder<'static, AppChooserWidget>,
}

impl AppChooserWidgetBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn default_text(self, default_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("default-text", default_text.into()),
        }
    }

    pub fn show_all(self, show_all: bool) -> Self {
        Self {
            builder: self.builder.property("show-all", show_all),
        }
    }

    pub fn show_default(self, show_default: bool) -> Self {
        Self {
            builder: self.builder.property("show-default", show_default),
        }
    }

    pub fn show_fallback(self, show_fallback: bool) -> Self {
        Self {
            builder: self.builder.property("show-fallback", show_fallback),
        }
    }

    pub fn show_other(self, show_other: bool) -> Self {
        Self {
            builder: self.builder.property("show-other", show_other),
        }
    }

    pub fn show_recommended(self, show_recommended: bool) -> Self {
        Self {
            builder: self.builder.property("show-recommended", show_recommended),
        }
    }

    pub fn baseline_position(self, baseline_position: BaselinePosition) -> Self {
        Self {
            builder: self
                .builder
                .property("baseline-position", baseline_position),
        }
    }

    pub fn homogeneous(self, homogeneous: bool) -> Self {
        Self {
            builder: self.builder.property("homogeneous", homogeneous),
        }
    }

    pub fn spacing(self, spacing: i32) -> Self {
        Self {
            builder: self.builder.property("spacing", spacing),
        }
    }

    pub fn border_width(self, border_width: u32) -> Self {
        Self {
            builder: self.builder.property("border-width", border_width),
        }
    }

    pub fn child(self, child: &impl IsA<Widget>) -> Self {
        Self {
            builder: self.builder.property("child", child.clone().upcast()),
        }
    }

    pub fn resize_mode(self, resize_mode: ResizeMode) -> Self {
        Self {
            builder: self.builder.property("resize-mode", resize_mode),
        }
    }

    pub fn app_paintable(self, app_paintable: bool) -> Self {
        Self {
            builder: self.builder.property("app-paintable", app_paintable),
        }
    }

    pub fn can_default(self, can_default: bool) -> Self {
        Self {
            builder: self.builder.property("can-default", can_default),
        }
    }

    pub fn can_focus(self, can_focus: bool) -> Self {
        Self {
            builder: self.builder.property("can-focus", can_focus),
        }
    }

    pub fn events(self, events: gdk::EventMask) -> Self {
        Self {
            builder: self.builder.property("events", events),
        }
    }

    pub fn expand(self, expand: bool) -> Self {
        Self {
            builder: self.builder.property("expand", expand),
        }
    }

    pub fn focus_on_click(self, focus_on_click: bool) -> Self {
        Self {
            builder: self.builder.property("focus-on-click", focus_on_click),
        }
    }

    pub fn halign(self, halign: Align) -> Self {
        Self {
            builder: self.builder.property("halign", halign),
        }
    }

    pub fn has_default(self, has_default: bool) -> Self {
        Self {
            builder: self.builder.property("has-default", has_default),
        }
    }

    pub fn has_focus(self, has_focus: bool) -> Self {
        Self {
            builder: self.builder.property("has-focus", has_focus),
        }
    }

    pub fn has_tooltip(self, has_tooltip: bool) -> Self {
        Self {
            builder: self.builder.property("has-tooltip", has_tooltip),
        }
    }

    pub fn height_request(self, height_request: i32) -> Self {
        Self {
            builder: self.builder.property("height-request", height_request),
        }
    }

    pub fn hexpand(self, hexpand: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand", hexpand),
        }
    }

    pub fn hexpand_set(self, hexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("hexpand-set", hexpand_set),
        }
    }

    pub fn is_focus(self, is_focus: bool) -> Self {
        Self {
            builder: self.builder.property("is-focus", is_focus),
        }
    }

    pub fn margin(self, margin: i32) -> Self {
        Self {
            builder: self.builder.property("margin", margin),
        }
    }

    pub fn margin_bottom(self, margin_bottom: i32) -> Self {
        Self {
            builder: self.builder.property("margin-bottom", margin_bottom),
        }
    }

    pub fn margin_end(self, margin_end: i32) -> Self {
        Self {
            builder: self.builder.property("margin-end", margin_end),
        }
    }

    pub fn margin_start(self, margin_start: i32) -> Self {
        Self {
            builder: self.builder.property("margin-start", margin_start),
        }
    }

    pub fn margin_top(self, margin_top: i32) -> Self {
        Self {
            builder: self.builder.property("margin-top", margin_top),
        }
    }

    pub fn name(self, name: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("name", name.into()),
        }
    }

    pub fn no_show_all(self, no_show_all: bool) -> Self {
        Self {
            builder: self.builder.property("no-show-all", no_show_all),
        }
    }

    pub fn opacity(self, opacity: f64) -> Self {
        Self {
            builder: self.builder.property("opacity", opacity),
        }
    }

    pub fn parent(self, parent: &impl IsA<Container>) -> Self {
        Self {
            builder: self.builder.property("parent", parent.clone().upcast()),
        }
    }

    pub fn receives_default(self, receives_default: bool) -> Self {
        Self {
            builder: self.builder.property("receives-default", receives_default),
        }
    }

    pub fn sensitive(self, sensitive: bool) -> Self {
        Self {
            builder: self.builder.property("sensitive", sensitive),
        }
    }

    pub fn tooltip_markup(self, tooltip_markup: impl Into<glib::GString>) -> Self {
        Self {
            builder: self
                .builder
                .property("tooltip-markup", tooltip_markup.into()),
        }
    }

    pub fn tooltip_text(self, tooltip_text: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("tooltip-text", tooltip_text.into()),
        }
    }

    pub fn valign(self, valign: Align) -> Self {
        Self {
            builder: self.builder.property("valign", valign),
        }
    }

    pub fn vexpand(self, vexpand: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand", vexpand),
        }
    }

    pub fn vexpand_set(self, vexpand_set: bool) -> Self {
        Self {
            builder: self.builder.property("vexpand-set", vexpand_set),
        }
    }

    pub fn visible(self, visible: bool) -> Self {
        Self {
            builder: self.builder.property("visible", visible),
        }
    }

    pub fn width_request(self, width_request: i32) -> Self {
        Self {
            builder: self.builder.property("width-request", width_request),
        }
    }

    pub fn orientation(self, orientation: Orientation) -> Self {
        Self {
            builder: self.builder.property("orientation", orientation),
        }
    }

    pub fn content_type(self, content_type: impl Into<glib::GString>) -> Self {
        Self {
            builder: self.builder.property("content-type", content_type.into()),
        }
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`AppChooserWidget`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> AppChooserWidget {
        self.builder.build()
    }
}

pub trait AppChooserWidgetExt: 'static {
    #[doc(alias = "gtk_app_chooser_widget_get_default_text")]
    #[doc(alias = "get_default_text")]
    fn default_text(&self) -> Option<glib::GString>;

    #[doc(alias = "gtk_app_chooser_widget_get_show_all")]
    #[doc(alias = "get_show_all")]
    fn shows_all(&self) -> bool;

    #[doc(alias = "gtk_app_chooser_widget_get_show_default")]
    #[doc(alias = "get_show_default")]
    fn shows_default(&self) -> bool;

    #[doc(alias = "gtk_app_chooser_widget_get_show_fallback")]
    #[doc(alias = "get_show_fallback")]
    fn shows_fallback(&self) -> bool;

    #[doc(alias = "gtk_app_chooser_widget_get_show_other")]
    #[doc(alias = "get_show_other")]
    fn shows_other(&self) -> bool;

    #[doc(alias = "gtk_app_chooser_widget_get_show_recommended")]
    #[doc(alias = "get_show_recommended")]
    fn shows_recommended(&self) -> bool;

    #[doc(alias = "gtk_app_chooser_widget_set_default_text")]
    fn set_default_text(&self, text: &str);

    #[doc(alias = "gtk_app_chooser_widget_set_show_all")]
    fn set_show_all(&self, setting: bool);

    #[doc(alias = "gtk_app_chooser_widget_set_show_default")]
    fn set_show_default(&self, setting: bool);

    #[doc(alias = "gtk_app_chooser_widget_set_show_fallback")]
    fn set_show_fallback(&self, setting: bool);

    #[doc(alias = "gtk_app_chooser_widget_set_show_other")]
    fn set_show_other(&self, setting: bool);

    #[doc(alias = "gtk_app_chooser_widget_set_show_recommended")]
    fn set_show_recommended(&self, setting: bool);

    #[doc(alias = "application-activated")]
    fn connect_application_activated<F: Fn(&Self, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "application-selected")]
    fn connect_application_selected<F: Fn(&Self, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "populate-popup")]
    fn connect_populate_popup<F: Fn(&Self, &Menu, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    #[doc(alias = "default-text")]
    fn connect_default_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-all")]
    fn connect_show_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-default")]
    fn connect_show_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-fallback")]
    fn connect_show_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-other")]
    fn connect_show_other_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-recommended")]
    fn connect_show_recommended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<AppChooserWidget>> AppChooserWidgetExt for O {
    fn default_text(&self) -> Option<glib::GString> {
        unsafe {
            from_glib_none(ffi::gtk_app_chooser_widget_get_default_text(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_all(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_all(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_default(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_default(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_fallback(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_fallback(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_other(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_other(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn shows_recommended(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_app_chooser_widget_get_show_recommended(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_default_text(&self, text: &str) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_default_text(
                self.as_ref().to_glib_none().0,
                text.to_glib_none().0,
            );
        }
    }

    fn set_show_all(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_all(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn set_show_default(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_default(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn set_show_fallback(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_fallback(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn set_show_other(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_other(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn set_show_recommended(&self, setting: bool) {
        unsafe {
            ffi::gtk_app_chooser_widget_set_show_recommended(
                self.as_ref().to_glib_none().0,
                setting.into_glib(),
            );
        }
    }

    fn connect_application_activated<F: Fn(&Self, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn application_activated_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P, &gio::AppInfo) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            application: *mut gio::ffi::GAppInfo,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(application),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"application-activated\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    application_activated_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_application_selected<F: Fn(&Self, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn application_selected_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P, &gio::AppInfo) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            application: *mut gio::ffi::GAppInfo,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(application),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"application-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    application_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_populate_popup<F: Fn(&Self, &Menu, &gio::AppInfo) + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn populate_popup_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P, &Menu, &gio::AppInfo) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            menu: *mut ffi::GtkMenu,
            application: *mut gio::ffi::GAppInfo,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(menu),
                &from_glib_borrow(application),
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"populate-popup\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    populate_popup_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_default_text_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_text_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-text\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_text_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_all_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_all_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-all\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_all_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_default_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_default_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-default\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_default_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_fallback_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_fallback_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-fallback\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_fallback_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_other_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_other_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-other\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_other_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_recommended_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_recommended_trampoline<
            P: IsA<AppChooserWidget>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkAppChooserWidget,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(AppChooserWidget::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-recommended\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_recommended_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for AppChooserWidget {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("AppChooserWidget")
    }
}
