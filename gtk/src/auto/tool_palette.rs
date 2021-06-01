// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Adjustment;
use crate::Align;
use crate::Buildable;
use crate::Container;
use crate::DestDefaults;
use crate::IconSize;
use crate::Orientable;
use crate::Orientation;
use crate::ResizeMode;
use crate::Scrollable;
use crate::ScrollablePolicy;
use crate::SelectionData;
use crate::TargetEntry;
use crate::ToolItem;
use crate::ToolItemGroup;
use crate::ToolPaletteDragTargets;
use crate::ToolbarStyle;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkToolPalette")]
    pub struct ToolPalette(Object<ffi::GtkToolPalette, ffi::GtkToolPaletteClass>) @extends Container, Widget, @implements Buildable, Orientable, Scrollable;

    match fn {
        type_ => || ffi::gtk_tool_palette_get_type(),
    }
}

impl ToolPalette {
    #[doc(alias = "gtk_tool_palette_new")]
    pub fn new() -> ToolPalette {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_tool_palette_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-style object to construct a [`ToolPalette`].
    ///
    /// This method returns an instance of [`ToolPaletteBuilder`] which can be used to create a [`ToolPalette`].
    pub fn builder() -> ToolPaletteBuilder {
        ToolPaletteBuilder::default()
    }

    #[doc(alias = "gtk_tool_palette_get_drag_target_group")]
    #[doc(alias = "get_drag_target_group")]
    pub fn drag_target_group() -> Option<TargetEntry> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_tool_palette_get_drag_target_group()) }
    }

    #[doc(alias = "gtk_tool_palette_get_drag_target_item")]
    #[doc(alias = "get_drag_target_item")]
    pub fn drag_target_item() -> Option<TargetEntry> {
        assert_initialized_main_thread!();
        unsafe { from_glib_none(ffi::gtk_tool_palette_get_drag_target_item()) }
    }
}

impl Default for ToolPalette {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A builder for generating a [`ToolPalette`].
pub struct ToolPaletteBuilder {
    icon_size: Option<IconSize>,
    icon_size_set: Option<bool>,
    toolbar_style: Option<ToolbarStyle>,
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
    hadjustment: Option<Adjustment>,
    hscroll_policy: Option<ScrollablePolicy>,
    vadjustment: Option<Adjustment>,
    vscroll_policy: Option<ScrollablePolicy>,
}

impl ToolPaletteBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`ToolPaletteBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`ToolPalette`].
    pub fn build(self) -> ToolPalette {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref icon_size) = self.icon_size {
            properties.push(("icon-size", icon_size));
        }
        if let Some(ref icon_size_set) = self.icon_size_set {
            properties.push(("icon-size-set", icon_size_set));
        }
        if let Some(ref toolbar_style) = self.toolbar_style {
            properties.push(("toolbar-style", toolbar_style));
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
        if let Some(ref hadjustment) = self.hadjustment {
            properties.push(("hadjustment", hadjustment));
        }
        if let Some(ref hscroll_policy) = self.hscroll_policy {
            properties.push(("hscroll-policy", hscroll_policy));
        }
        if let Some(ref vadjustment) = self.vadjustment {
            properties.push(("vadjustment", vadjustment));
        }
        if let Some(ref vscroll_policy) = self.vscroll_policy {
            properties.push(("vscroll-policy", vscroll_policy));
        }
        glib::Object::new::<ToolPalette>(&properties)
            .expect("Failed to create an instance of ToolPalette")
    }

    pub fn icon_size(mut self, icon_size: IconSize) -> Self {
        self.icon_size = Some(icon_size);
        self
    }

    pub fn icon_size_set(mut self, icon_size_set: bool) -> Self {
        self.icon_size_set = Some(icon_size_set);
        self
    }

    pub fn toolbar_style(mut self, toolbar_style: ToolbarStyle) -> Self {
        self.toolbar_style = Some(toolbar_style);
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

    pub fn hadjustment<P: IsA<Adjustment>>(mut self, hadjustment: &P) -> Self {
        self.hadjustment = Some(hadjustment.clone().upcast());
        self
    }

    pub fn hscroll_policy(mut self, hscroll_policy: ScrollablePolicy) -> Self {
        self.hscroll_policy = Some(hscroll_policy);
        self
    }

    pub fn vadjustment<P: IsA<Adjustment>>(mut self, vadjustment: &P) -> Self {
        self.vadjustment = Some(vadjustment.clone().upcast());
        self
    }

    pub fn vscroll_policy(mut self, vscroll_policy: ScrollablePolicy) -> Self {
        self.vscroll_policy = Some(vscroll_policy);
        self
    }
}

pub const NONE_TOOL_PALETTE: Option<&ToolPalette> = None;

pub trait ToolPaletteExt: 'static {
    #[doc(alias = "gtk_tool_palette_add_drag_dest")]
    fn add_drag_dest<P: IsA<Widget>>(
        &self,
        widget: &P,
        flags: DestDefaults,
        targets: ToolPaletteDragTargets,
        actions: gdk::DragAction,
    );

    #[doc(alias = "gtk_tool_palette_get_drag_item")]
    #[doc(alias = "get_drag_item")]
    fn drag_item(&self, selection: &SelectionData) -> Option<Widget>;

    #[doc(alias = "gtk_tool_palette_get_drop_group")]
    #[doc(alias = "get_drop_group")]
    fn drop_group(&self, x: i32, y: i32) -> Option<ToolItemGroup>;

    #[doc(alias = "gtk_tool_palette_get_drop_item")]
    #[doc(alias = "get_drop_item")]
    fn drop_item(&self, x: i32, y: i32) -> Option<ToolItem>;

    #[doc(alias = "gtk_tool_palette_get_exclusive")]
    #[doc(alias = "get_exclusive")]
    fn is_exclusive<P: IsA<ToolItemGroup>>(&self, group: &P) -> bool;

    #[doc(alias = "gtk_tool_palette_get_expand")]
    #[doc(alias = "get_expand")]
    fn expands<P: IsA<ToolItemGroup>>(&self, group: &P) -> bool;

    #[doc(alias = "gtk_tool_palette_get_group_position")]
    #[doc(alias = "get_group_position")]
    fn group_position<P: IsA<ToolItemGroup>>(&self, group: &P) -> i32;

    #[doc(alias = "gtk_tool_palette_get_icon_size")]
    #[doc(alias = "get_icon_size")]
    fn icon_size(&self) -> IconSize;

    #[doc(alias = "gtk_tool_palette_get_style")]
    #[doc(alias = "get_style")]
    fn style(&self) -> ToolbarStyle;

    #[doc(alias = "gtk_tool_palette_set_drag_source")]
    fn set_drag_source(&self, targets: ToolPaletteDragTargets);

    #[doc(alias = "gtk_tool_palette_set_exclusive")]
    fn set_exclusive<P: IsA<ToolItemGroup>>(&self, group: &P, exclusive: bool);

    #[doc(alias = "gtk_tool_palette_set_expand")]
    fn set_expand<P: IsA<ToolItemGroup>>(&self, group: &P, expand: bool);

    #[doc(alias = "gtk_tool_palette_set_group_position")]
    fn set_group_position<P: IsA<ToolItemGroup>>(&self, group: &P, position: i32);

    #[doc(alias = "gtk_tool_palette_set_icon_size")]
    fn set_icon_size(&self, icon_size: IconSize);

    #[doc(alias = "gtk_tool_palette_set_style")]
    fn set_style(&self, style: ToolbarStyle);

    #[doc(alias = "gtk_tool_palette_unset_icon_size")]
    fn unset_icon_size(&self);

    #[doc(alias = "gtk_tool_palette_unset_style")]
    fn unset_style(&self);

    #[doc(alias = "icon-size-set")]
    fn is_icon_size_set(&self) -> bool;

    #[doc(alias = "icon-size-set")]
    fn set_icon_size_set(&self, icon_size_set: bool);

    #[doc(alias = "toolbar-style")]
    fn toolbar_style(&self) -> ToolbarStyle;

    #[doc(alias = "toolbar-style")]
    fn set_toolbar_style(&self, toolbar_style: ToolbarStyle);

    #[doc(alias = "icon-size")]
    fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "icon-size-set")]
    fn connect_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "toolbar-style")]
    fn connect_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<ToolPalette>> ToolPaletteExt for O {
    fn add_drag_dest<P: IsA<Widget>>(
        &self,
        widget: &P,
        flags: DestDefaults,
        targets: ToolPaletteDragTargets,
        actions: gdk::DragAction,
    ) {
        unsafe {
            ffi::gtk_tool_palette_add_drag_dest(
                self.as_ref().to_glib_none().0,
                widget.as_ref().to_glib_none().0,
                flags.into_glib(),
                targets.into_glib(),
                actions.into_glib(),
            );
        }
    }

    fn drag_item(&self, selection: &SelectionData) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drag_item(
                self.as_ref().to_glib_none().0,
                selection.to_glib_none().0,
            ))
        }
    }

    fn drop_group(&self, x: i32, y: i32) -> Option<ToolItemGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_group(
                self.as_ref().to_glib_none().0,
                x,
                y,
            ))
        }
    }

    fn drop_item(&self, x: i32, y: i32) -> Option<ToolItem> {
        unsafe {
            from_glib_none(ffi::gtk_tool_palette_get_drop_item(
                self.as_ref().to_glib_none().0,
                x,
                y,
            ))
        }
    }

    fn is_exclusive<P: IsA<ToolItemGroup>>(&self, group: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_exclusive(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
            ))
        }
    }

    fn expands<P: IsA<ToolItemGroup>>(&self, group: &P) -> bool {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_expand(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
            ))
        }
    }

    fn group_position<P: IsA<ToolItemGroup>>(&self, group: &P) -> i32 {
        unsafe {
            ffi::gtk_tool_palette_get_group_position(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
            )
        }
    }

    fn icon_size(&self) -> IconSize {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_icon_size(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_palette_get_style(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_drag_source(&self, targets: ToolPaletteDragTargets) {
        unsafe {
            ffi::gtk_tool_palette_set_drag_source(
                self.as_ref().to_glib_none().0,
                targets.into_glib(),
            );
        }
    }

    fn set_exclusive<P: IsA<ToolItemGroup>>(&self, group: &P, exclusive: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_exclusive(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                exclusive.into_glib(),
            );
        }
    }

    fn set_expand<P: IsA<ToolItemGroup>>(&self, group: &P, expand: bool) {
        unsafe {
            ffi::gtk_tool_palette_set_expand(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                expand.into_glib(),
            );
        }
    }

    fn set_group_position<P: IsA<ToolItemGroup>>(&self, group: &P, position: i32) {
        unsafe {
            ffi::gtk_tool_palette_set_group_position(
                self.as_ref().to_glib_none().0,
                group.as_ref().to_glib_none().0,
                position,
            );
        }
    }

    fn set_icon_size(&self, icon_size: IconSize) {
        unsafe {
            ffi::gtk_tool_palette_set_icon_size(
                self.as_ref().to_glib_none().0,
                icon_size.into_glib(),
            );
        }
    }

    fn set_style(&self, style: ToolbarStyle) {
        unsafe {
            ffi::gtk_tool_palette_set_style(self.as_ref().to_glib_none().0, style.into_glib());
        }
    }

    fn unset_icon_size(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_icon_size(self.as_ref().to_glib_none().0);
        }
    }

    fn unset_style(&self) {
        unsafe {
            ffi::gtk_tool_palette_unset_style(self.as_ref().to_glib_none().0);
        }
    }

    fn is_icon_size_set(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"icon-size-set\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `icon-size-set` getter")
        }
    }

    fn set_icon_size_set(&self, icon_size_set: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"icon-size-set\0".as_ptr() as *const _,
                icon_size_set.to_value().to_glib_none().0,
            );
        }
    }

    fn toolbar_style(&self) -> ToolbarStyle {
        unsafe {
            let mut value = glib::Value::from_type(<ToolbarStyle as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"toolbar-style\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `toolbar-style` getter")
        }
    }

    fn set_toolbar_style(&self, toolbar_style: ToolbarStyle) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"toolbar-style\0".as_ptr() as *const _,
                toolbar_style.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "icon-size")]
    fn connect_icon_size_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_trampoline<
            P: IsA<ToolPalette>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolPalette,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&ToolPalette::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-size\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_size_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "icon-size-set")]
    fn connect_icon_size_set_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_icon_size_set_trampoline<
            P: IsA<ToolPalette>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolPalette,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&ToolPalette::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::icon-size-set\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_icon_size_set_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "toolbar-style")]
    fn connect_toolbar_style_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_toolbar_style_trampoline<
            P: IsA<ToolPalette>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkToolPalette,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&ToolPalette::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::toolbar-style\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_toolbar_style_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ToolPalette {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("ToolPalette")
    }
}
