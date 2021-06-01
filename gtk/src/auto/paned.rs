// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::Orientable;
use crate::Orientation;
use crate::ResizeMode;
use crate::ScrollType;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::object::ObjectExt;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkPaned")]
    pub struct Paned(Object<ffi::GtkPaned, ffi::GtkPanedClass>) @extends Container, Widget, @implements Buildable, Orientable;

    match fn {
        type_ => || ffi::gtk_paned_get_type(),
    }
}

impl Paned {
    #[doc(alias = "gtk_paned_new")]
    pub fn new(orientation: Orientation) -> Paned {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_paned_new(orientation.into_glib())).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`Paned`].
    ///
    /// This method returns an instance of [`PanedBuilder`] which can be used to create a [`Paned`].
    pub fn builder() -> PanedBuilder {
        PanedBuilder::default()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`Paned`].
pub struct PanedBuilder {
    position: Option<i32>,
    position_set: Option<bool>,
    wide_handle: Option<bool>,
    border_width: Option<u32>,
    child: Option<Widget>,
    resize_mode: Option<ResizeMode>,
    app_paintable: Option<bool>,
    can_default: Option<bool>,
    can_focus: Option<bool>,
    events: Option<gdk::EventMask>,
    expand: Option<bool>,
    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    focus_on_click: Option<bool>,
    halign: Option<Align>,
    has_default: Option<bool>,
    has_focus: Option<bool>,
    has_tooltip: Option<bool>,
    height_request: Option<i32>,
    hexpand: Option<bool>,
    hexpand_set: Option<bool>,
    is_focus: Option<bool>,
    margin: Option<i32>,
    margin_bottom: Option<i32>,
    margin_end: Option<i32>,
    margin_start: Option<i32>,
    margin_top: Option<i32>,
    name: Option<String>,
    no_show_all: Option<bool>,
    opacity: Option<f64>,
    parent: Option<Container>,
    receives_default: Option<bool>,
    sensitive: Option<bool>,
    tooltip_markup: Option<String>,
    tooltip_text: Option<String>,
    valign: Option<Align>,
    vexpand: Option<bool>,
    vexpand_set: Option<bool>,
    visible: Option<bool>,
    width_request: Option<i32>,
    orientation: Option<Orientation>,
}

impl PanedBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`PanedBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Paned`].
    pub fn build(self) -> Paned {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref position) = self.position {
            properties.push(("position", position));
        }
        if let Some(ref position_set) = self.position_set {
            properties.push(("position-set", position_set));
        }
        if let Some(ref wide_handle) = self.wide_handle {
            properties.push(("wide-handle", wide_handle));
        }
        if let Some(ref border_width) = self.border_width {
            properties.push(("border-width", border_width));
        }
        if let Some(ref child) = self.child {
            properties.push(("child", child));
        }
        if let Some(ref resize_mode) = self.resize_mode {
            properties.push(("resize-mode", resize_mode));
        }
        if let Some(ref app_paintable) = self.app_paintable {
            properties.push(("app-paintable", app_paintable));
        }
        if let Some(ref can_default) = self.can_default {
            properties.push(("can-default", can_default));
        }
        if let Some(ref can_focus) = self.can_focus {
            properties.push(("can-focus", can_focus));
        }
        if let Some(ref events) = self.events {
            properties.push(("events", events));
        }
        if let Some(ref expand) = self.expand {
            properties.push(("expand", expand));
        }
        #[cfg(any(feature = "v3_20", feature = "dox"))]
        if let Some(ref focus_on_click) = self.focus_on_click {
            properties.push(("focus-on-click", focus_on_click));
        }
        if let Some(ref halign) = self.halign {
            properties.push(("halign", halign));
        }
        if let Some(ref has_default) = self.has_default {
            properties.push(("has-default", has_default));
        }
        if let Some(ref has_focus) = self.has_focus {
            properties.push(("has-focus", has_focus));
        }
        if let Some(ref has_tooltip) = self.has_tooltip {
            properties.push(("has-tooltip", has_tooltip));
        }
        if let Some(ref height_request) = self.height_request {
            properties.push(("height-request", height_request));
        }
        if let Some(ref hexpand) = self.hexpand {
            properties.push(("hexpand", hexpand));
        }
        if let Some(ref hexpand_set) = self.hexpand_set {
            properties.push(("hexpand-set", hexpand_set));
        }
        if let Some(ref is_focus) = self.is_focus {
            properties.push(("is-focus", is_focus));
        }
        if let Some(ref margin) = self.margin {
            properties.push(("margin", margin));
        }
        if let Some(ref margin_bottom) = self.margin_bottom {
            properties.push(("margin-bottom", margin_bottom));
        }
        if let Some(ref margin_end) = self.margin_end {
            properties.push(("margin-end", margin_end));
        }
        if let Some(ref margin_start) = self.margin_start {
            properties.push(("margin-start", margin_start));
        }
        if let Some(ref margin_top) = self.margin_top {
            properties.push(("margin-top", margin_top));
        }
        if let Some(ref name) = self.name {
            properties.push(("name", name));
        }
        if let Some(ref no_show_all) = self.no_show_all {
            properties.push(("no-show-all", no_show_all));
        }
        if let Some(ref opacity) = self.opacity {
            properties.push(("opacity", opacity));
        }
        if let Some(ref parent) = self.parent {
            properties.push(("parent", parent));
        }
        if let Some(ref receives_default) = self.receives_default {
            properties.push(("receives-default", receives_default));
        }
        if let Some(ref sensitive) = self.sensitive {
            properties.push(("sensitive", sensitive));
        }
        if let Some(ref tooltip_markup) = self.tooltip_markup {
            properties.push(("tooltip-markup", tooltip_markup));
        }
        if let Some(ref tooltip_text) = self.tooltip_text {
            properties.push(("tooltip-text", tooltip_text));
        }
        if let Some(ref valign) = self.valign {
            properties.push(("valign", valign));
        }
        if let Some(ref vexpand) = self.vexpand {
            properties.push(("vexpand", vexpand));
        }
        if let Some(ref vexpand_set) = self.vexpand_set {
            properties.push(("vexpand-set", vexpand_set));
        }
        if let Some(ref visible) = self.visible {
            properties.push(("visible", visible));
        }
        if let Some(ref width_request) = self.width_request {
            properties.push(("width-request", width_request));
        }
        if let Some(ref orientation) = self.orientation {
            properties.push(("orientation", orientation));
        }
        glib::Object::new::<Paned>(&properties).expect("Failed to create an instance of Paned")
    }

    pub fn position(mut self, position: i32) -> Self {
        self.position = Some(position);
        self
    }

    pub fn position_set(mut self, position_set: bool) -> Self {
        self.position_set = Some(position_set);
        self
    }

    pub fn wide_handle(mut self, wide_handle: bool) -> Self {
        self.wide_handle = Some(wide_handle);
        self
    }

    pub fn border_width(mut self, border_width: u32) -> Self {
        self.border_width = Some(border_width);
        self
    }

    pub fn child<P: IsA<Widget>>(mut self, child: &P) -> Self {
        self.child = Some(child.clone().upcast());
        self
    }

    pub fn resize_mode(mut self, resize_mode: ResizeMode) -> Self {
        self.resize_mode = Some(resize_mode);
        self
    }

    pub fn app_paintable(mut self, app_paintable: bool) -> Self {
        self.app_paintable = Some(app_paintable);
        self
    }

    pub fn can_default(mut self, can_default: bool) -> Self {
        self.can_default = Some(can_default);
        self
    }

    pub fn can_focus(mut self, can_focus: bool) -> Self {
        self.can_focus = Some(can_focus);
        self
    }

    pub fn events(mut self, events: gdk::EventMask) -> Self {
        self.events = Some(events);
        self
    }

    pub fn expand(mut self, expand: bool) -> Self {
        self.expand = Some(expand);
        self
    }

    #[cfg(any(feature = "v3_20", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v3_20")))]
    pub fn focus_on_click(mut self, focus_on_click: bool) -> Self {
        self.focus_on_click = Some(focus_on_click);
        self
    }

    pub fn halign(mut self, halign: Align) -> Self {
        self.halign = Some(halign);
        self
    }

    pub fn has_default(mut self, has_default: bool) -> Self {
        self.has_default = Some(has_default);
        self
    }

    pub fn has_focus(mut self, has_focus: bool) -> Self {
        self.has_focus = Some(has_focus);
        self
    }

    pub fn has_tooltip(mut self, has_tooltip: bool) -> Self {
        self.has_tooltip = Some(has_tooltip);
        self
    }

    pub fn height_request(mut self, height_request: i32) -> Self {
        self.height_request = Some(height_request);
        self
    }

    pub fn hexpand(mut self, hexpand: bool) -> Self {
        self.hexpand = Some(hexpand);
        self
    }

    pub fn hexpand_set(mut self, hexpand_set: bool) -> Self {
        self.hexpand_set = Some(hexpand_set);
        self
    }

    pub fn is_focus(mut self, is_focus: bool) -> Self {
        self.is_focus = Some(is_focus);
        self
    }

    pub fn margin(mut self, margin: i32) -> Self {
        self.margin = Some(margin);
        self
    }

    pub fn margin_bottom(mut self, margin_bottom: i32) -> Self {
        self.margin_bottom = Some(margin_bottom);
        self
    }

    pub fn margin_end(mut self, margin_end: i32) -> Self {
        self.margin_end = Some(margin_end);
        self
    }

    pub fn margin_start(mut self, margin_start: i32) -> Self {
        self.margin_start = Some(margin_start);
        self
    }

    pub fn margin_top(mut self, margin_top: i32) -> Self {
        self.margin_top = Some(margin_top);
        self
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn no_show_all(mut self, no_show_all: bool) -> Self {
        self.no_show_all = Some(no_show_all);
        self
    }

    pub fn opacity(mut self, opacity: f64) -> Self {
        self.opacity = Some(opacity);
        self
    }

    pub fn parent<P: IsA<Container>>(mut self, parent: &P) -> Self {
        self.parent = Some(parent.clone().upcast());
        self
    }

    pub fn receives_default(mut self, receives_default: bool) -> Self {
        self.receives_default = Some(receives_default);
        self
    }

    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = Some(sensitive);
        self
    }

    pub fn tooltip_markup(mut self, tooltip_markup: &str) -> Self {
        self.tooltip_markup = Some(tooltip_markup.to_string());
        self
    }

    pub fn tooltip_text(mut self, tooltip_text: &str) -> Self {
        self.tooltip_text = Some(tooltip_text.to_string());
        self
    }

    pub fn valign(mut self, valign: Align) -> Self {
        self.valign = Some(valign);
        self
    }

    pub fn vexpand(mut self, vexpand: bool) -> Self {
        self.vexpand = Some(vexpand);
        self
    }

    pub fn vexpand_set(mut self, vexpand_set: bool) -> Self {
        self.vexpand_set = Some(vexpand_set);
        self
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = Some(visible);
        self
    }

    pub fn width_request(mut self, width_request: i32) -> Self {
        self.width_request = Some(width_request);
        self
    }

    pub fn orientation(mut self, orientation: Orientation) -> Self {
        self.orientation = Some(orientation);
        self
    }
}

pub const NONE_PANED: Option<&Paned> = None;

pub trait PanedExt: 'static {
    #[doc(alias = "gtk_paned_add1")]
    fn add1<P: IsA<Widget>>(&self, child: &P);

    #[doc(alias = "gtk_paned_add2")]
    fn add2<P: IsA<Widget>>(&self, child: &P);

    #[doc(alias = "gtk_paned_get_child1")]
    #[doc(alias = "get_child1")]
    fn child1(&self) -> Option<Widget>;

    #[doc(alias = "gtk_paned_get_child2")]
    #[doc(alias = "get_child2")]
    fn child2(&self) -> Option<Widget>;

    #[doc(alias = "gtk_paned_get_handle_window")]
    #[doc(alias = "get_handle_window")]
    fn handle_window(&self) -> Option<gdk::Window>;

    #[doc(alias = "gtk_paned_get_position")]
    #[doc(alias = "get_position")]
    fn position(&self) -> i32;

    #[doc(alias = "gtk_paned_get_wide_handle")]
    #[doc(alias = "get_wide_handle")]
    fn is_wide_handle(&self) -> bool;

    #[doc(alias = "gtk_paned_pack1")]
    fn pack1<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool);

    #[doc(alias = "gtk_paned_pack2")]
    fn pack2<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool);

    #[doc(alias = "gtk_paned_set_position")]
    fn set_position(&self, position: i32);

    #[doc(alias = "gtk_paned_set_wide_handle")]
    fn set_wide_handle(&self, wide: bool);

    #[doc(alias = "max-position")]
    fn max_position(&self) -> i32;

    #[doc(alias = "min-position")]
    fn min_position(&self) -> i32;

    #[doc(alias = "position-set")]
    fn is_position_set(&self) -> bool;

    #[doc(alias = "position-set")]
    fn set_position_set(&self, position_set: bool);

    fn child_resizes<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_resize<T: IsA<Widget>>(&self, item: &T, resize: bool);

    fn child_shrinks<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_shrink<T: IsA<Widget>>(&self, item: &T, shrink: bool);

    #[doc(alias = "accept-position")]
    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_accept_position(&self) -> bool;

    #[doc(alias = "cancel-position")]
    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_cancel_position(&self) -> bool;

    #[doc(alias = "cycle-child-focus")]
    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool;

    #[doc(alias = "cycle-handle-focus")]
    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool;

    #[doc(alias = "move-handle")]
    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool;

    #[doc(alias = "toggle-handle-focus")]
    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId;

    fn emit_toggle_handle_focus(&self) -> bool;

    #[doc(alias = "max-position")]
    fn connect_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "min-position")]
    fn connect_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "position")]
    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "position-set")]
    fn connect_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "wide-handle")]
    fn connect_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Paned>> PanedExt for O {
    fn add1<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_paned_add1(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn add2<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_paned_add2(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
            );
        }
    }

    fn child1(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_child1(self.as_ref().to_glib_none().0)) }
    }

    fn child2(&self) -> Option<Widget> {
        unsafe { from_glib_none(ffi::gtk_paned_get_child2(self.as_ref().to_glib_none().0)) }
    }

    fn handle_window(&self) -> Option<gdk::Window> {
        unsafe {
            from_glib_none(ffi::gtk_paned_get_handle_window(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn position(&self) -> i32 {
        unsafe { ffi::gtk_paned_get_position(self.as_ref().to_glib_none().0) }
    }

    fn is_wide_handle(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_paned_get_wide_handle(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn pack1<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack1(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.into_glib(),
                shrink.into_glib(),
            );
        }
    }

    fn pack2<P: IsA<Widget>>(&self, child: &P, resize: bool, shrink: bool) {
        unsafe {
            ffi::gtk_paned_pack2(
                self.as_ref().to_glib_none().0,
                child.as_ref().to_glib_none().0,
                resize.into_glib(),
                shrink.into_glib(),
            );
        }
    }

    fn set_position(&self, position: i32) {
        unsafe {
            ffi::gtk_paned_set_position(self.as_ref().to_glib_none().0, position);
        }
    }

    fn set_wide_handle(&self, wide: bool) {
        unsafe {
            ffi::gtk_paned_set_wide_handle(self.as_ref().to_glib_none().0, wide.into_glib());
        }
    }

    fn max_position(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"max-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-position` getter")
        }
    }

    fn min_position(&self) -> i32 {
        unsafe {
            let mut value = glib::Value::from_type(<i32 as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"min-position\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `min-position` getter")
        }
    }

    fn is_position_set(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"position-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `position-set` getter")
        }
    }

    fn set_position_set(&self, position_set: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"position-set\0".as_ptr() as *const _,
                position_set.to_value().to_glib_none().0,
            );
        }
    }

    fn child_resizes<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"resize\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `resize` getter")
        }
    }

    fn set_child_resize<T: IsA<Widget>>(&self, item: &T, resize: bool) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"resize\0".as_ptr() as *const _,
                resize.to_value().to_glib_none().0,
            );
        }
    }

    fn child_shrinks<T: IsA<Widget>>(&self, item: &T) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            crate::ffi::gtk_container_child_get_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"shrink\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `shrink` getter")
        }
    }

    fn set_child_shrink<T: IsA<Widget>>(&self, item: &T, shrink: bool) {
        unsafe {
            crate::ffi::gtk_container_child_set_property(
                self.to_glib_none().0 as *mut crate::ffi::GtkContainer,
                item.to_glib_none().0 as *mut _,
                b"shrink\0".as_ptr() as *const _,
                shrink.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "accept-position")]
    fn connect_accept_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn accept_position_trampoline<
            P: IsA<Paned>,
            F: Fn(&P) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"accept-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    accept_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_accept_position(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("accept-position", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_accept_position`")
    }

    #[doc(alias = "cancel-position")]
    fn connect_cancel_position<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn cancel_position_trampoline<
            P: IsA<Paned>,
            F: Fn(&P) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cancel-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cancel_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cancel_position(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("cancel-position", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cancel_position`")
    }

    #[doc(alias = "cycle-child-focus")]
    fn connect_cycle_child_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_child_focus_trampoline<
            P: IsA<Paned>,
            F: Fn(&P, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &Paned::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(reversed),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-child-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_child_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_child_focus(&self, reversed: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("cycle-child-focus", &[&reversed])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cycle_child_focus`")
    }

    #[doc(alias = "cycle-handle-focus")]
    fn connect_cycle_handle_focus<F: Fn(&Self, bool) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn cycle_handle_focus_trampoline<
            P: IsA<Paned>,
            F: Fn(&P, bool) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            reversed: glib::ffi::gboolean,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &Paned::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(reversed),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"cycle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    cycle_handle_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_cycle_handle_focus(&self, reversed: bool) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("cycle-handle-focus", &[&reversed])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_cycle_handle_focus`")
    }

    #[doc(alias = "move-handle")]
    fn connect_move_handle<F: Fn(&Self, ScrollType) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn move_handle_trampoline<
            P: IsA<Paned>,
            F: Fn(&P, ScrollType) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            scroll_type: ffi::GtkScrollType,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(
                &Paned::from_glib_borrow(this).unsafe_cast_ref(),
                from_glib(scroll_type),
            )
            .into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"move-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    move_handle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_move_handle(&self, scroll_type: ScrollType) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("move-handle", &[&scroll_type])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_move_handle`")
    }

    #[doc(alias = "toggle-handle-focus")]
    fn connect_toggle_handle_focus<F: Fn(&Self) -> bool + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn toggle_handle_focus_trampoline<
            P: IsA<Paned>,
            F: Fn(&P) -> bool + 'static,
        >(
            this: *mut ffi::GtkPaned,
            f: glib::ffi::gpointer,
        ) -> glib::ffi::gboolean {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref()).into_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"toggle-handle-focus\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    toggle_handle_focus_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn emit_toggle_handle_focus(&self) -> bool {
        let res = unsafe {
            glib::Object::from_glib_borrow(self.as_ptr() as *mut glib::gobject_ffi::GObject)
                .emit_by_name("toggle-handle-focus", &[])
                .unwrap()
        };
        res.unwrap()
            .get()
            .expect("Return Value for `emit_toggle_handle_focus`")
    }

    #[doc(alias = "max-position")]
    fn connect_max_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_max_position_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::max-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_max_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "min-position")]
    fn connect_min_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_min_position_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::min-position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_min_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position")]
    fn connect_position_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "position-set")]
    fn connect_position_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_position_set_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::position-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_position_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "wide-handle")]
    fn connect_wide_handle_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_wide_handle_trampoline<P: IsA<Paned>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkPaned,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&Paned::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::wide-handle\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_wide_handle_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Paned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Paned")
    }
}
