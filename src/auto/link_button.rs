// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use glib::GString;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct LinkButton(Object<ffi::GtkLinkButton, ffi::GtkLinkButtonClass, LinkButtonClass>) @extends Button, Bin, Container, Widget, @implements Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_link_button_get_type(),
    }
}

impl LinkButton {
    pub fn new(uri: &str) -> LinkButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new(uri.to_glib_none().0)).unsafe_cast()
        }
    }

    pub fn new_with_label<'a, P: Into<Option<&'a str>>>(uri: &str, label: P) -> LinkButton {
        assert_initialized_main_thread!();
        let label = label.into();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new_with_label(uri.to_glib_none().0, label.to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_LINK_BUTTON: Option<&LinkButton> = None;

pub trait LinkButtonExt: 'static {
    fn get_uri(&self) -> Option<GString>;

    fn get_visited(&self) -> bool;

    fn set_uri(&self, uri: &str);

    fn set_visited(&self, visited: bool);

    fn connect_activate_link<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_visited_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<LinkButton>> LinkButtonExt for O {
    fn get_uri(&self) -> Option<GString> {
        unsafe {
            from_glib_none(ffi::gtk_link_button_get_uri(self.as_ref().to_glib_none().0))
        }
    }

    fn get_visited(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_link_button_get_visited(self.as_ref().to_glib_none().0))
        }
    }

    fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_link_button_set_uri(self.as_ref().to_glib_none().0, uri.to_glib_none().0);
        }
    }

    fn set_visited(&self, visited: bool) {
        unsafe {
            ffi::gtk_link_button_set_visited(self.as_ref().to_glib_none().0, visited.to_glib());
        }
    }

    fn connect_activate_link<F: Fn(&Self) -> Inhibit + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"activate-link\0".as_ptr() as *const _,
                Some(transmute(activate_link_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_uri_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::uri\0".as_ptr() as *const _,
                Some(transmute(notify_uri_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_visited_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::visited\0".as_ptr() as *const _,
                Some(transmute(notify_visited_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn activate_link_trampoline<P, F: Fn(&P) -> Inhibit + 'static>(this: *mut ffi::GtkLinkButton, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<LinkButton> {
    let f: &F = transmute(f);
    f(&LinkButton::from_glib_borrow(this).unsafe_cast()).to_glib()
}

unsafe extern "C" fn notify_uri_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLinkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LinkButton> {
    let f: &F = transmute(f);
    f(&LinkButton::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_visited_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkLinkButton, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<LinkButton> {
    let f: &F = transmute(f);
    f(&LinkButton::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for LinkButton {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "LinkButton")
    }
}
