// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{Align, Buildable, Container, Widget};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem, mem::transmute};

glib::wrapper! {
    #[doc(alias = "GtkGLArea")]
    pub struct GLArea(Object<ffi::GtkGLArea, ffi::GtkGLAreaClass>) @extends Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_gl_area_get_type(),
    }
}

impl GLArea {
    pub const NONE: Option<&'static GLArea> = None;

    #[doc(alias = "gtk_gl_area_new")]
    pub fn new() -> GLArea {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_gl_area_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`GLArea`] objects.
    ///
    /// This method returns an instance of [`GLAreaBuilder`](crate::builders::GLAreaBuilder) which can be used to create [`GLArea`] objects.
    pub fn builder() -> GLAreaBuilder {
        GLAreaBuilder::new()
    }
}

impl Default for GLArea {
    fn default() -> Self {
        Self::new()
    }
}

// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`GLArea`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct GLAreaBuilder {
    builder: glib::object::ObjectBuilder<'static, GLArea>,
}

impl GLAreaBuilder {
    fn new() -> Self {
        Self {
            builder: glib::object::Object::builder(),
        }
    }

    pub fn auto_render(self, auto_render: bool) -> Self {
        Self {
            builder: self.builder.property("auto-render", auto_render),
        }
    }

    pub fn has_alpha(self, has_alpha: bool) -> Self {
        Self {
            builder: self.builder.property("has-alpha", has_alpha),
        }
    }

    pub fn has_depth_buffer(self, has_depth_buffer: bool) -> Self {
        Self {
            builder: self.builder.property("has-depth-buffer", has_depth_buffer),
        }
    }

    pub fn has_stencil_buffer(self, has_stencil_buffer: bool) -> Self {
        Self {
            builder: self
                .builder
                .property("has-stencil-buffer", has_stencil_buffer),
        }
    }

    pub fn use_es(self, use_es: bool) -> Self {
        Self {
            builder: self.builder.property("use-es", use_es),
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

    // rustdoc-stripper-ignore-next
    /// Build the [`GLArea`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> GLArea {
        self.builder.build()
    }
}

mod sealed {
    pub trait Sealed {}
    impl<T: super::IsA<super::GLArea>> Sealed for T {}
}

pub trait GLAreaExt: IsA<GLArea> + sealed::Sealed + 'static {
    #[doc(alias = "gtk_gl_area_attach_buffers")]
    fn attach_buffers(&self) {
        unsafe {
            ffi::gtk_gl_area_attach_buffers(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_gl_area_get_auto_render")]
    #[doc(alias = "get_auto_render")]
    fn is_auto_render(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_auto_render(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gl_area_get_context")]
    #[doc(alias = "get_context")]
    fn context(&self) -> Option<gdk::GLContext> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_context(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_gl_area_get_error")]
    #[doc(alias = "get_error")]
    fn error(&self) -> Option<glib::Error> {
        unsafe { from_glib_none(ffi::gtk_gl_area_get_error(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_gl_area_get_has_alpha")]
    #[doc(alias = "get_has_alpha")]
    fn has_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_alpha(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gl_area_get_has_depth_buffer")]
    #[doc(alias = "get_has_depth_buffer")]
    fn has_depth_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_depth_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gl_area_get_has_stencil_buffer")]
    #[doc(alias = "get_has_stencil_buffer")]
    fn has_stencil_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_stencil_buffer(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_gl_area_get_required_version")]
    #[doc(alias = "get_required_version")]
    fn required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::MaybeUninit::uninit();
            let mut minor = mem::MaybeUninit::uninit();
            ffi::gtk_gl_area_get_required_version(
                self.as_ref().to_glib_none().0,
                major.as_mut_ptr(),
                minor.as_mut_ptr(),
            );
            (major.assume_init(), minor.assume_init())
        }
    }

    #[doc(alias = "gtk_gl_area_get_use_es")]
    #[doc(alias = "get_use_es")]
    fn uses_es(&self) -> bool {
        unsafe { from_glib(ffi::gtk_gl_area_get_use_es(self.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "gtk_gl_area_make_current")]
    fn make_current(&self) {
        unsafe {
            ffi::gtk_gl_area_make_current(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_gl_area_queue_render")]
    fn queue_render(&self) {
        unsafe {
            ffi::gtk_gl_area_queue_render(self.as_ref().to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_gl_area_set_auto_render")]
    fn set_auto_render(&self, auto_render: bool) {
        unsafe {
            ffi::gtk_gl_area_set_auto_render(
                self.as_ref().to_glib_none().0,
                auto_render.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_gl_area_set_error")]
    fn set_error(&self, error: Option<&glib::Error>) {
        unsafe {
            ffi::gtk_gl_area_set_error(self.as_ref().to_glib_none().0, error.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_gl_area_set_has_alpha")]
    fn set_has_alpha(&self, has_alpha: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_alpha(self.as_ref().to_glib_none().0, has_alpha.into_glib());
        }
    }

    #[doc(alias = "gtk_gl_area_set_has_depth_buffer")]
    fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_depth_buffer(
                self.as_ref().to_glib_none().0,
                has_depth_buffer.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_gl_area_set_has_stencil_buffer")]
    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_stencil_buffer(
                self.as_ref().to_glib_none().0,
                has_stencil_buffer.into_glib(),
            );
        }
    }

    #[doc(alias = "gtk_gl_area_set_required_version")]
    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gtk_gl_area_set_required_version(self.as_ref().to_glib_none().0, major, minor);
        }
    }

    #[doc(alias = "gtk_gl_area_set_use_es")]
    fn set_use_es(&self, use_es: bool) {
        unsafe {
            ffi::gtk_gl_area_set_use_es(self.as_ref().to_glib_none().0, use_es.into_glib());
        }
    }

    #[doc(alias = "create-context")]
    fn connect_create_context<F: Fn(&Self) -> Option<gdk::GLContext> + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn create_context_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) -> Option<gdk::GLContext> + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            f: glib::ffi::gpointer,
        ) -> *mut gdk::ffi::GdkGLContext {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref()).to_glib_full()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"create-context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    create_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "render")]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> glib::ControlFlow + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn render_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P, &gdk::GLContext) -> glib::ControlFlow + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            context: *mut gdk::ffi::GdkGLContext,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                GLArea::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(context),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"render\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    render_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "resize")]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn resize_trampoline<P: IsA<GLArea>, F: Fn(&P, i32, i32) + 'static>(
            this: *mut ffi::GtkGLArea,
            width: libc::c_int,
            height: libc::c_int,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(
                GLArea::from_glib_borrow(this).unsafe_cast_ref(),
                width,
                height,
            )
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"resize\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    resize_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "auto-render")]
    fn connect_auto_render_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_auto_render_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::auto-render\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_auto_render_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "context")]
    fn connect_context_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_context_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::context\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_context_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-alpha")]
    fn connect_has_alpha_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_alpha_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-alpha\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_alpha_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-depth-buffer")]
    fn connect_has_depth_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_depth_buffer_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-depth-buffer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_depth_buffer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "has-stencil-buffer")]
    fn connect_has_stencil_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_has_stencil_buffer_trampoline<
            P: IsA<GLArea>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::has-stencil-buffer\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_has_stencil_buffer_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "use-es")]
    fn connect_use_es_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_use_es_trampoline<P: IsA<GLArea>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkGLArea,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(GLArea::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::use-es\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_use_es_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl<O: IsA<GLArea>> GLAreaExt for O {}

impl fmt::Display for GLArea {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GLArea")
    }
}
