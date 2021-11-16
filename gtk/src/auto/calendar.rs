// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Align;
use crate::Buildable;
use crate::CalendarDisplayOptions;
use crate::Container;
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
use std::mem;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkCalendar")]
    pub struct Calendar(Object<ffi::GtkCalendar, ffi::GtkCalendarClass>) @extends Widget, @implements Buildable;

    match fn {
        type_ => || ffi::gtk_calendar_get_type(),
    }
}

impl Calendar {
    pub const NONE: Option<&'static Calendar> = None;

    #[doc(alias = "gtk_calendar_new")]
    pub fn new() -> Calendar {
        assert_initialized_main_thread!();
        unsafe { Widget::from_glib_none(ffi::gtk_calendar_new()).unsafe_cast() }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`Calendar`] objects.
    ///
    /// This method returns an instance of [`CalendarBuilder`](crate::builders::CalendarBuilder) which can be used to create [`Calendar`] objects.
    pub fn builder() -> CalendarBuilder {
        CalendarBuilder::default()
    }
}

impl Default for Calendar {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`Calendar`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
pub struct CalendarBuilder {
    day: Option<i32>,
    detail_height_rows: Option<i32>,
    detail_width_chars: Option<i32>,
    month: Option<i32>,
    no_month_change: Option<bool>,
    show_day_names: Option<bool>,
    show_details: Option<bool>,
    show_heading: Option<bool>,
    show_week_numbers: Option<bool>,
    year: Option<i32>,
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
}

impl CalendarBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`CalendarBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`Calendar`].
    #[must_use = "The builder must be built to be used"]
    pub fn build(self) -> Calendar {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        if let Some(ref day) = self.day {
            properties.push(("day", day));
        }
        if let Some(ref detail_height_rows) = self.detail_height_rows {
            properties.push(("detail-height-rows", detail_height_rows));
        }
        if let Some(ref detail_width_chars) = self.detail_width_chars {
            properties.push(("detail-width-chars", detail_width_chars));
        }
        if let Some(ref month) = self.month {
            properties.push(("month", month));
        }
        if let Some(ref no_month_change) = self.no_month_change {
            properties.push(("no-month-change", no_month_change));
        }
        if let Some(ref show_day_names) = self.show_day_names {
            properties.push(("show-day-names", show_day_names));
        }
        if let Some(ref show_details) = self.show_details {
            properties.push(("show-details", show_details));
        }
        if let Some(ref show_heading) = self.show_heading {
            properties.push(("show-heading", show_heading));
        }
        if let Some(ref show_week_numbers) = self.show_week_numbers {
            properties.push(("show-week-numbers", show_week_numbers));
        }
        if let Some(ref year) = self.year {
            properties.push(("year", year));
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
        glib::Object::new::<Calendar>(&properties)
            .expect("Failed to create an instance of Calendar")
    }

    pub fn day(mut self, day: i32) -> Self {
        self.day = Some(day);
        self
    }

    pub fn detail_height_rows(mut self, detail_height_rows: i32) -> Self {
        self.detail_height_rows = Some(detail_height_rows);
        self
    }

    pub fn detail_width_chars(mut self, detail_width_chars: i32) -> Self {
        self.detail_width_chars = Some(detail_width_chars);
        self
    }

    pub fn month(mut self, month: i32) -> Self {
        self.month = Some(month);
        self
    }

    pub fn no_month_change(mut self, no_month_change: bool) -> Self {
        self.no_month_change = Some(no_month_change);
        self
    }

    pub fn show_day_names(mut self, show_day_names: bool) -> Self {
        self.show_day_names = Some(show_day_names);
        self
    }

    pub fn show_details(mut self, show_details: bool) -> Self {
        self.show_details = Some(show_details);
        self
    }

    pub fn show_heading(mut self, show_heading: bool) -> Self {
        self.show_heading = Some(show_heading);
        self
    }

    pub fn show_week_numbers(mut self, show_week_numbers: bool) -> Self {
        self.show_week_numbers = Some(show_week_numbers);
        self
    }

    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
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

    pub fn parent(mut self, parent: &impl IsA<Container>) -> Self {
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
}

pub trait CalendarExt: 'static {
    #[doc(alias = "gtk_calendar_clear_marks")]
    fn clear_marks(&self);

    #[doc(alias = "gtk_calendar_get_date")]
    #[doc(alias = "get_date")]
    fn date(&self) -> (u32, u32, u32);

    #[doc(alias = "gtk_calendar_get_day_is_marked")]
    #[doc(alias = "get_day_is_marked")]
    fn day_is_marked(&self, day: u32) -> bool;

    #[doc(alias = "gtk_calendar_get_detail_height_rows")]
    #[doc(alias = "get_detail_height_rows")]
    fn detail_height_rows(&self) -> i32;

    #[doc(alias = "gtk_calendar_get_detail_width_chars")]
    #[doc(alias = "get_detail_width_chars")]
    fn detail_width_chars(&self) -> i32;

    #[doc(alias = "gtk_calendar_get_display_options")]
    #[doc(alias = "get_display_options")]
    fn display_options(&self) -> CalendarDisplayOptions;

    #[doc(alias = "gtk_calendar_mark_day")]
    fn mark_day(&self, day: u32);

    #[doc(alias = "gtk_calendar_select_day")]
    fn select_day(&self, day: u32);

    #[doc(alias = "gtk_calendar_select_month")]
    fn select_month(&self, month: u32, year: u32);

    #[doc(alias = "gtk_calendar_set_detail_func")]
    fn set_detail_func<P: Fn(&Calendar, u32, u32, u32) -> Option<String> + 'static>(&self, func: P);

    #[doc(alias = "gtk_calendar_set_detail_height_rows")]
    fn set_detail_height_rows(&self, rows: i32);

    #[doc(alias = "gtk_calendar_set_detail_width_chars")]
    fn set_detail_width_chars(&self, chars: i32);

    #[doc(alias = "gtk_calendar_set_display_options")]
    fn set_display_options(&self, flags: CalendarDisplayOptions);

    #[doc(alias = "gtk_calendar_unmark_day")]
    fn unmark_day(&self, day: u32);

    fn day(&self) -> i32;

    fn set_day(&self, day: i32);

    fn month(&self) -> i32;

    fn set_month(&self, month: i32);

    #[doc(alias = "no-month-change")]
    fn is_no_month_change(&self) -> bool;

    #[doc(alias = "no-month-change")]
    fn set_no_month_change(&self, no_month_change: bool);

    #[doc(alias = "show-day-names")]
    fn shows_day_names(&self) -> bool;

    #[doc(alias = "show-day-names")]
    fn set_show_day_names(&self, show_day_names: bool);

    #[doc(alias = "show-details")]
    fn shows_details(&self) -> bool;

    #[doc(alias = "show-details")]
    fn set_show_details(&self, show_details: bool);

    #[doc(alias = "show-heading")]
    fn shows_heading(&self) -> bool;

    #[doc(alias = "show-heading")]
    fn set_show_heading(&self, show_heading: bool);

    #[doc(alias = "show-week-numbers")]
    fn shows_week_numbers(&self) -> bool;

    #[doc(alias = "show-week-numbers")]
    fn set_show_week_numbers(&self, show_week_numbers: bool);

    fn year(&self) -> i32;

    fn set_year(&self, year: i32);

    #[doc(alias = "day-selected")]
    fn connect_day_selected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "day-selected-double-click")]
    fn connect_day_selected_double_click<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "month-changed")]
    fn connect_month_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "next-month")]
    fn connect_next_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "next-year")]
    fn connect_next_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "prev-month")]
    fn connect_prev_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "prev-year")]
    fn connect_prev_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "day")]
    fn connect_day_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "detail-height-rows")]
    fn connect_detail_height_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "detail-width-chars")]
    fn connect_detail_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "month")]
    fn connect_month_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "no-month-change")]
    fn connect_no_month_change_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-day-names")]
    fn connect_show_day_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-details")]
    fn connect_show_details_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-heading")]
    fn connect_show_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "show-week-numbers")]
    fn connect_show_week_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "year")]
    fn connect_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Calendar>> CalendarExt for O {
    fn clear_marks(&self) {
        unsafe {
            ffi::gtk_calendar_clear_marks(self.as_ref().to_glib_none().0);
        }
    }

    fn date(&self) -> (u32, u32, u32) {
        unsafe {
            let mut year = mem::MaybeUninit::uninit();
            let mut month = mem::MaybeUninit::uninit();
            let mut day = mem::MaybeUninit::uninit();
            ffi::gtk_calendar_get_date(
                self.as_ref().to_glib_none().0,
                year.as_mut_ptr(),
                month.as_mut_ptr(),
                day.as_mut_ptr(),
            );
            let year = year.assume_init();
            let month = month.assume_init();
            let day = day.assume_init();
            (year, month, day)
        }
    }

    fn day_is_marked(&self, day: u32) -> bool {
        unsafe {
            from_glib(ffi::gtk_calendar_get_day_is_marked(
                self.as_ref().to_glib_none().0,
                day,
            ))
        }
    }

    fn detail_height_rows(&self) -> i32 {
        unsafe { ffi::gtk_calendar_get_detail_height_rows(self.as_ref().to_glib_none().0) }
    }

    fn detail_width_chars(&self) -> i32 {
        unsafe { ffi::gtk_calendar_get_detail_width_chars(self.as_ref().to_glib_none().0) }
    }

    fn display_options(&self) -> CalendarDisplayOptions {
        unsafe {
            from_glib(ffi::gtk_calendar_get_display_options(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn mark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_mark_day(self.as_ref().to_glib_none().0, day);
        }
    }

    fn select_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_select_day(self.as_ref().to_glib_none().0, day);
        }
    }

    fn select_month(&self, month: u32, year: u32) {
        unsafe {
            ffi::gtk_calendar_select_month(self.as_ref().to_glib_none().0, month, year);
        }
    }

    fn set_detail_func<P: Fn(&Calendar, u32, u32, u32) -> Option<String> + 'static>(
        &self,
        func: P,
    ) {
        let func_data: Box_<P> = Box_::new(func);
        unsafe extern "C" fn func_func<
            P: Fn(&Calendar, u32, u32, u32) -> Option<String> + 'static,
        >(
            calendar: *mut ffi::GtkCalendar,
            year: libc::c_uint,
            month: libc::c_uint,
            day: libc::c_uint,
            user_data: glib::ffi::gpointer,
        ) -> *mut libc::c_char {
            let calendar = from_glib_borrow(calendar);
            let callback: &P = &*(user_data as *mut _);
            let res = (*callback)(&calendar, year, month, day);
            res.to_glib_full()
        }
        let func = Some(func_func::<P> as _);
        unsafe extern "C" fn destroy_func<
            P: Fn(&Calendar, u32, u32, u32) -> Option<String> + 'static,
        >(
            data: glib::ffi::gpointer,
        ) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call3 = Some(destroy_func::<P> as _);
        let super_callback0: Box_<P> = func_data;
        unsafe {
            ffi::gtk_calendar_set_detail_func(
                self.as_ref().to_glib_none().0,
                func,
                Box_::into_raw(super_callback0) as *mut _,
                destroy_call3,
            );
        }
    }

    fn set_detail_height_rows(&self, rows: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_height_rows(self.as_ref().to_glib_none().0, rows);
        }
    }

    fn set_detail_width_chars(&self, chars: i32) {
        unsafe {
            ffi::gtk_calendar_set_detail_width_chars(self.as_ref().to_glib_none().0, chars);
        }
    }

    fn set_display_options(&self, flags: CalendarDisplayOptions) {
        unsafe {
            ffi::gtk_calendar_set_display_options(
                self.as_ref().to_glib_none().0,
                flags.into_glib(),
            );
        }
    }

    fn unmark_day(&self, day: u32) {
        unsafe {
            ffi::gtk_calendar_unmark_day(self.as_ref().to_glib_none().0, day);
        }
    }

    fn day(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "day")
    }

    fn set_day(&self, day: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "day", &day)
    }

    fn month(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "month")
    }

    fn set_month(&self, month: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "month", &month)
    }

    fn is_no_month_change(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "no-month-change")
    }

    fn set_no_month_change(&self, no_month_change: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "no-month-change", &no_month_change)
    }

    fn shows_day_names(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-day-names")
    }

    fn set_show_day_names(&self, show_day_names: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-day-names", &show_day_names)
    }

    fn shows_details(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-details")
    }

    fn set_show_details(&self, show_details: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-details", &show_details)
    }

    fn shows_heading(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-heading")
    }

    fn set_show_heading(&self, show_heading: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-heading", &show_heading)
    }

    fn shows_week_numbers(&self) -> bool {
        glib::ObjectExt::property(self.as_ref(), "show-week-numbers")
    }

    fn set_show_week_numbers(&self, show_week_numbers: bool) {
        glib::ObjectExt::set_property(self.as_ref(), "show-week-numbers", &show_week_numbers)
    }

    fn year(&self) -> i32 {
        glib::ObjectExt::property(self.as_ref(), "year")
    }

    fn set_year(&self, year: i32) {
        glib::ObjectExt::set_property(self.as_ref(), "year", &year)
    }

    fn connect_day_selected<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn day_selected_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"day-selected\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    day_selected_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_day_selected_double_click<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn day_selected_double_click_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"day-selected-double-click\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    day_selected_double_click_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_month_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn month_changed_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"month-changed\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    month_changed_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_next_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_month_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_month_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_next_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn next_year_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"next-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    next_year_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_prev_month<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_month_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_month_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_prev_year<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn prev_year_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"prev-year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    prev_year_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_day_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_day_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::day\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_day_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_detail_height_rows_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detail_height_rows_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::detail-height-rows\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_detail_height_rows_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_detail_width_chars_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_detail_width_chars_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::detail-width-chars\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_detail_width_chars_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_month_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_month_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::month\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_month_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_no_month_change_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_no_month_change_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::no-month-change\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_no_month_change_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_day_names_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_day_names_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-day-names\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_day_names_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_details_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_details_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-details\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_details_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_heading_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_heading_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-heading\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_heading_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_show_week_numbers_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_show_week_numbers_trampoline<
            P: IsA<Calendar>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::show-week-numbers\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_show_week_numbers_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_year_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_year_trampoline<P: IsA<Calendar>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCalendar,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(Calendar::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::year\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_year_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Calendar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Calendar")
    }
}
